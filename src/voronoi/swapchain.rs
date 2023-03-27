use crate::ocl;
use crate::opencl::ImageView;

pub(super) struct Swapchain {
    max_dim: usize,
    dim: usize,

    images: [ImageView<i32>; 2],
    last: usize,
}

impl Swapchain {
    pub(super) fn new(queue: &ocl::Queue, dim: usize) -> anyhow::Result<Self> {
        if !dim.is_power_of_two() {
            anyhow::bail!("The given maximal dimension {dim} isn't the power of two");
        }

        let undefined_data = vec![-1; dim * dim * 4];

        let input_image = ImageView::new::<usize>(
            ocl::Image::builder()
                .queue(queue.clone())
                .channel_data_type(ocl::enums::ImageChannelDataType::SignedInt32)
                .channel_order(ocl::enums::ImageChannelOrder::Rgba)
                .image_type(ocl::enums::MemObjectType::Image2d)
                .dims((dim, dim))
                .copy_host_slice(&undefined_data)
                .build()?,
            None,
            None,
        )?;

        let output_image = ImageView::new::<usize>(
            ocl::Image::builder()
                .queue(queue.clone())
                .channel_data_type(ocl::enums::ImageChannelDataType::SignedInt32)
                .channel_order(ocl::enums::ImageChannelOrder::Rgba)
                .image_type(ocl::enums::MemObjectType::Image2d)
                .dims((dim, dim))
                .copy_host_slice(&undefined_data)
                .build()?,
            None,
            None,
        )?;

        Ok(Self { max_dim: dim, dim, images: [input_image, output_image], last: 0 })
    }

    pub(super) fn dim(&self) -> usize {
        self.dim
    }

    pub(super) fn set_dim(&mut self, dim: usize) -> anyhow::Result<()> {
        if !dim.is_power_of_two() {
            anyhow::bail!("The given dimension {dim} isn't the power of two");
        }

        if dim > self.max_dim {
            anyhow::bail!(
                "The given dimension {dim} exceeds the maximal dimension of the image {}",
                self.max_dim
            )
        }

        self.dim = dim;
        self.images[0].set_region([dim; 2])?;
        self.images[1].set_region([dim; 2])?;

        Ok(())
    }

    pub(super) fn render<F, T>(&mut self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(&ImageView<i32>, &ImageView<i32>) -> anyhow::Result<T>,
    {
        let last_frame = self.last();
        let next_frame = self.next();

        let render_result = f(last_frame, next_frame);
        self.swap();

        render_result
    }

    pub(super) fn last(&self) -> &ImageView<i32> {
        &self.images[self.last]
    }

    fn next(&self) -> &ImageView<i32> {
        &self.images[(self.last + 1) % 2]
    }

    fn swap(&mut self) {
        self.last = (self.last + 1) % 2;
    }

    pub(super) fn clear(&self) -> anyhow::Result<()> {
        let undefined_data = vec![-1; self.dim * self.dim * 4];

        self.last().write(&undefined_data)?;
        self.next().write(&undefined_data)?;

        Ok(())
    }
}
