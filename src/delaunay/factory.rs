use arena_system::{Arena, Handle};

use super::bounds::Bounds;
use super::triangle::{DelaunayTriangle, TriangleId};
use super::Delaunay;

use crate::delaunay::DelaunayTriangleHandle;
use crate::opencl::Buffer;
use crate::point::{Point, PointHandle, PointId};
use crate::voronoi::{Pixel, VoronoiImage};

pub struct DelaunayFactory {
    count_triangles_kernel: ocl::Kernel,
    triangle_number_buffer: Buffer<i32>,

    build_triangles_kernel: ocl::Kernel,
    triangles_buffer: Buffer<DelaunayTriangle>,
    free_triangle_index_buffer: Buffer<i32>,

    find_neighbours_kernel: ocl::Kernel,
}

impl DelaunayFactory {
    pub fn new(queue: ocl::Queue) -> anyhow::Result<Self> {
        let program = ocl::Program::builder()
            .src_file("src/kernels/delaunay.cl")
            .build(&queue.context())?;

        let count_triangles_kernel = ocl::Kernel::builder()
            .queue(queue.clone())
            .program(&program)
            .name("count_triangles")
            .arg(None::<&ocl::Image<i32>>)
            .arg(None::<&ocl::Buffer<i32>>)
            .build()?;
        let mut triangle_number_buffer = Buffer::<i32>::new(queue.clone())?;
        triangle_number_buffer.write(&[0])?;

        let build_triangles_kernel = ocl::Kernel::builder()
            .queue(queue.clone())
            .program(&program)
            .name("build_triangles")
            .arg(None::<&ocl::Image<i32>>)
            .arg(None::<&ocl::Buffer<DelaunayTriangle>>)
            .arg(None::<&ocl::Buffer<TriangleId>>)
            .build()?;
        let triangles_buffer = Buffer::<DelaunayTriangle>::new(queue.clone())?;
        let mut free_triangle_index_buffer = Buffer::<i32>::new(queue.clone())?;
        free_triangle_index_buffer.write(&[0])?;

        let find_neighbours_kernel = ocl::Kernel::builder()
            .queue(queue.clone())
            .program(&program)
            .name("find_neighbours")
            .arg(None::<&ocl::Buffer<DelaunayTriangle>>)
            .build()?;

        Ok(Self {
            count_triangles_kernel,
            triangle_number_buffer,

            build_triangles_kernel,
            triangles_buffer,
            free_triangle_index_buffer,

            find_neighbours_kernel,
        })
    }

    pub fn construct(&mut self, voronoi_image: &VoronoiImage<'_>) -> anyhow::Result<Delaunay> {
        let dim = voronoi_image.dim();
        let mut points = voronoi_image
            .sites()
            .handle_iter::<PointHandle>(())
            .map(|site| {
                Point::with_previous(
                    site.x().floor(),
                    site.y().floor(),
                    site.previous_in_outline().index().into(),
                )
            })
            .collect::<Arena<Point>>();
        let mut triangles = self.build_triangles(voronoi_image, &points)?;

        let mut voronoi_image_pixels = voronoi_image.to_pixels()?;
        let bounds = self.add_bounds(dim, &mut points, &mut voronoi_image_pixels);

        self.fix_convex_hull(dim, &points, &mut triangles, &voronoi_image_pixels)?;
        self.find_neighbours(&mut triangles)?;

        let triangles = triangles
            .into_iter()
            .collect::<Arena<DelaunayTriangle>>();
        self.flip_triangles(&triangles, &points);

        Ok(Delaunay::new(dim, points, triangles, bounds))
    }

    fn count_triangles(&mut self, voronoi_image: &VoronoiImage<'_>) -> anyhow::Result<i32> {
        self.count_triangles_kernel
            .set_default_global_work_size((voronoi_image.dim(), voronoi_image.dim()).into())
            .set_default_local_work_size((8, 8).into());

        self.count_triangles_kernel.set_arg(0, voronoi_image.image().ocl_image())?;
        self.count_triangles_kernel
            .set_arg(1, self.triangle_number_buffer.as_raw())?;

        unsafe {
            self.count_triangles_kernel.enq()?;
        }

        let triangle_number = self.triangle_number_buffer.first()?;

        Ok(triangle_number)
    }

    fn build_triangles(
        &mut self,
        voronoi_image: &VoronoiImage<'_>,
        points: &Arena<Point>,
    ) -> anyhow::Result<Vec<DelaunayTriangle>> {
        let triangle_number = self.count_triangles(voronoi_image)?;
        if triangle_number == 0 {
            return Ok(vec![]);
        }

        let mut triangles = vec![DelaunayTriangle::default(); triangle_number as usize];
        self.triangles_buffer.write(&triangles)?;

        self.build_triangles_kernel
            .set_default_global_work_size((voronoi_image.dim(), voronoi_image.dim()).into())
            .set_default_local_work_size((8, 8).into());

        self.build_triangles_kernel.set_arg(0, voronoi_image.image().ocl_image())?;
        self.build_triangles_kernel.set_arg(1, self.triangles_buffer.as_raw())?;
        self.build_triangles_kernel
            .set_arg(2, self.free_triangle_index_buffer.as_raw())?;

        unsafe {
            self.build_triangles_kernel.enq()?;
        }

        self.triangles_buffer.read(&mut triangles)?;
        triangles.iter_mut().for_each(|t| t.make_counterclockwise(points));

        Ok(triangles)
    }

    fn add_bounds(
        &self,
        dim: usize,
        points: &mut Arena<Point>,
        voronoi_image_pixels: &mut Vec<Pixel>,
    ) -> Bounds {
        let first_bounding_point_id = points.len() as i64;

        self.add_bounding_points(dim, points);
        self.add_bounding_pixels(dim, first_bounding_point_id, voronoi_image_pixels);

        Bounds::new([
            first_bounding_point_id,
            first_bounding_point_id + 1,
            first_bounding_point_id + 2,
            first_bounding_point_id + 3,
        ])
    }

    fn add_bounding_points(&self, dim: usize, points: &mut Arena<Point>) {
        let min_x = -(dim as f32 * 10.0);
        let min_y = -(dim as f32 * 10.0);
        let max_x = dim as f32 * 10.0;
        let max_y = dim as f32 * 10.0;

        points.add(Point::with_is_bounding(min_x, min_y, true));
        points.add(Point::with_is_bounding(max_x, min_y, true));
        points.add(Point::with_is_bounding(max_x, max_y, true));
        points.add(Point::with_is_bounding(min_x, max_y, true));
    }

    fn add_bounding_pixels(
        &self,
        dim: usize,
        first_bounding_point_id: PointId,
        voronoi_image_pixels: &mut Vec<Pixel>,
    ) {
        let min_x = 0;
        let min_y = 0;
        let max_x = dim - 1;
        let max_y = dim - 1;

        voronoi_image_pixels[min_x + min_y * dim] = Pixel::new(
            min_x,
            min_y,
            [-(dim as i64 * 10), -(dim as i64 * 10), first_bounding_point_id],
        );
        voronoi_image_pixels[max_x + min_y * dim] = Pixel::new(
            max_x,
            min_y,
            [dim as i64 * 10, -(dim as i64 * 10), first_bounding_point_id + 1],
        );
        voronoi_image_pixels[max_x + max_y * dim] = Pixel::new(
            max_x,
            max_y,
            [dim as i64 * 10, dim as i64 * 10, first_bounding_point_id + 2],
        );
        voronoi_image_pixels[min_x + max_y * dim] = Pixel::new(
            min_x,
            max_y,
            [-(dim as i64 * 10), dim as i64 * 10, first_bounding_point_id + 3],
        );
    }

    fn fix_convex_hull(
        &self,
        dim: usize,
        points: &Arena<Point>,
        triangles: &mut Vec<DelaunayTriangle>,
        voronoi_image_pixels: &Vec<Pixel>,
    ) -> anyhow::Result<()> {
        let mut pixel_stack: Vec<&Pixel> = vec![];

        'pixels: for pixel in voronoi_image_border_pixels_iter(dim, &voronoi_image_pixels) {
            'vertices: while let Some(last) = pixel_stack.last() {
                if last.nearest_site_id() == pixel.nearest_site_id() {
                    continue 'pixels;
                }

                let a = match pixel_stack.get(pixel_stack.len() - 2) {
                    Some(val) => val.nearest_site_id(),
                    None => break 'vertices,
                };
                let b = last.nearest_site_id();
                let c = pixel.nearest_site_id();

                let triangle = DelaunayTriangle::new([a, b, c]);
                if triangle.is_counterclockwise(points) {
                    triangles.push(triangle);
                    pixel_stack.pop();
                } else {
                    break 'vertices;
                }
            }

            pixel_stack.push(pixel);
        }

        Ok(())
    }

    fn find_neighbours(&mut self, triangles: &mut Vec<DelaunayTriangle>) -> anyhow::Result<()> {
        self.triangles_buffer.write(triangles)?;

        self.find_neighbours_kernel
            .set_default_global_work_size((triangles.len(), triangles.len()).into())
            .set_default_local_work_size((1, 1).into());

        self.find_neighbours_kernel.set_arg(0, self.triangles_buffer.as_raw())?;

        unsafe {
            self.find_neighbours_kernel.enq()?;
        }

        self.triangles_buffer.read(triangles)?;

        Ok(())
    }

    fn flip_triangles(&self, triangles: &Arena<DelaunayTriangle>, points: &Arena<Point>) {
        (0..triangles.len())
            .into_iter()
            .map(|i| i.into())
            .map(|i| triangles.handle::<DelaunayTriangleHandle>(i, points))
            .for_each(|mut triangle_handle| {
                triangle_handle
                    .neighbours()
                    .into_iter()
                    .all(|mut neighbour| triangle_handle.flip_with(&mut neighbour, 128));
            });
    }
}

#[rustfmt::skip]
fn voronoi_image_border_pixels_iter(
    dim: usize,
    voronoi_image_pixels: &Vec<Pixel>,
) -> impl Iterator<Item = &Pixel> + '_ {
    let bottom_border_iter = voronoi_image_pixels
        .iter()
        .filter(move |pixel| pixel.y() == 0);
    let left_border_iter = voronoi_image_pixels
        .iter()
        .filter(move |pixel| pixel.x() == dim - 1);
    let top_border_iter = voronoi_image_pixels
        .iter()
        .filter(move |pixel| pixel.y() == dim - 1)
        .rev();
    let right_border_iter = voronoi_image_pixels
        .iter()
        .filter(move |pixel| pixel.x() == 0)
        .rev();

    bottom_border_iter
        .chain(left_border_iter)
        .chain(top_border_iter)
        .chain(right_border_iter)
}
