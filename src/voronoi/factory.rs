use arena_system::Arena;

use super::swapchain::Swapchain;
use super::VoronoiImage;

use crate::ocl::{self, prm::Float2};
use crate::opencl::Buffer;
use crate::point::{Point, PointHandle};

use std::{borrow::Cow, iter};

pub struct VoronoiImageFactory {
    swapchain: Swapchain,

    plot_sites_kernel: ocl::Kernel,
    sites_buffer: Buffer<Float2>,

    fill_voronoi_kernel: ocl::Kernel,

    conquer_islands_kernel: ocl::Kernel,
    changed_pixels_number_buffer: Buffer<i32>,
}

impl VoronoiImageFactory {
    pub fn new(queue: ocl::Queue, max_dim: usize) -> anyhow::Result<Self> {
        let swapchain = Swapchain::new(&queue, max_dim)?;
        let program = ocl::Program::builder()
            .src_file("src/opencl/kernels/voronoi.cl")
            .build(&queue.context())?;

        let plot_sites_kernel = ocl::Kernel::builder()
            .queue(queue.clone())
            .program(&program)
            .name("plot_sites")
            .arg(None::<&ocl::Buffer<Float2>>)
            .arg(None::<&ocl::Image<i32>>)
            .build()?;
        let sites_buffer = Buffer::new(queue.clone())?;

        let fill_voronoi_kernel = ocl::Kernel::builder()
            .queue(queue.clone())
            .program(&program)
            .name("fill_voronoi")
            .arg(None::<&ocl::Image<i32>>)
            .arg(None::<&ocl::Image<i32>>)
            .arg(0)
            .build()?;

        let conquer_islands_kernel = ocl::Kernel::builder()
            .queue(queue.clone())
            .program(&program)
            .name("conquer_islands")
            .arg(None::<&ocl::Image<i32>>)
            .arg(None::<&ocl::Image<i32>>)
            .arg(None::<&ocl::Buffer<i32>>)
            .build()?;
        let mut changed_pixels_number_buffer = Buffer::new(queue.clone())?;
        changed_pixels_number_buffer.write(&[0])?;

        Ok(Self {
            swapchain,

            plot_sites_kernel,
            sites_buffer,

            fill_voronoi_kernel,

            conquer_islands_kernel,
            changed_pixels_number_buffer,
        })
    }

    pub fn construct_owned(
        &mut self,
        sites: Arena<Point>,
        dim: usize,
    ) -> anyhow::Result<VoronoiImage<'static>> {
        self.draw_voronoi(&sites, dim)?;

        let copied_image = self.swapchain.last().deepcopy()?;

        Ok(VoronoiImage::new(dim, sites, Cow::Owned(copied_image)))
    }

    pub fn construct_borrowed<'s>(
        &'s mut self,
        sites: Arena<Point>,
        dim: usize,
    ) -> anyhow::Result<VoronoiImage<'s>> {
        self.draw_voronoi(&sites, dim)?;

        Ok(VoronoiImage::new(dim, sites, Cow::Borrowed(self.swapchain.last())))
    }

    fn draw_voronoi(&mut self, sites: &Arena<Point>, dim: usize) -> anyhow::Result<()> {
        self.swapchain.set_dim(dim)?;
        self.swapchain.clear()?;

        self.plot_sites(sites)?;
        self.fill_voronoi()?;
        self.conquer_islands()?;

        Ok(())
    }

    fn plot_sites(&mut self, sites: &Arena<Point>) -> anyhow::Result<()> {
        let raw_sites = sites
            .handle_iter::<PointHandle>(None)
            .map(|s| s.coords())
            .collect::<Vec<Float2>>();
        self.sites_buffer.write(&raw_sites)?;

        self.plot_sites_kernel
            .set_default_global_work_size(self.sites_buffer.len().into())
            .set_default_local_work_size(1.into());

        self.swapchain.render(|_, next_frame| {
            self.plot_sites_kernel.set_arg(0, self.sites_buffer.as_raw())?;
            self.plot_sites_kernel.set_arg(1, next_frame.ocl_image())?;

            unsafe {
                self.plot_sites_kernel.enq()?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn fill_voronoi(&mut self) -> anyhow::Result<()> {
        let dim = self.swapchain.dim();
        self.fill_voronoi_kernel
            .set_default_global_work_size((dim, dim).into())
            .set_default_local_work_size((8, 8).into());

        #[allow(non_snake_case)]
        let N = dim;
        let max_n = dim.ilog2();
        iter::once(max_n).chain(1..=max_n).map(|n| N / (1 << n)).for_each(|k| {
            self.swapchain
                .render(|last_frame, next_frame| {
                    self.fill_voronoi_kernel.set_arg(0, last_frame.ocl_image())?;
                    self.fill_voronoi_kernel.set_arg(1, next_frame.ocl_image())?;
                    self.fill_voronoi_kernel.set_arg(2, k as i32)?;

                    unsafe {
                        self.fill_voronoi_kernel.enq()?;
                    }

                    Ok(())
                })
                .unwrap();
        });

        Ok(())
    }

    fn conquer_islands(&mut self) -> anyhow::Result<()> {
        let dim = self.swapchain.dim();
        self.conquer_islands_kernel
            .set_default_global_work_size((dim, dim).into())
            .set_default_local_work_size((8, 8).into());

        let mut is_conquered = false;
        while !is_conquered {
            self.swapchain.render(|last_frame, next_frame| {
                self.conquer_islands_kernel.set_arg(0, last_frame.ocl_image())?;
                self.conquer_islands_kernel.set_arg(1, next_frame.ocl_image())?;
                self.conquer_islands_kernel
                    .set_arg(2, self.changed_pixels_number_buffer.as_raw())?;

                unsafe {
                    self.conquer_islands_kernel.enq()?;
                }

                Ok(())
            })?;

            let changed_pixels_number = self.changed_pixels_number_buffer.first()?;
            self.changed_pixels_number_buffer.clear()?;

            println!("Changed pixel number: {}", changed_pixels_number);

            is_conquered = changed_pixels_number == 0;
        }

        Ok(())
    }
}
