mod factory;
mod pixel;
mod site;
mod swapchain;

pub use factory::VoronoiImageFactory;
pub use pixel::Pixel;
pub use site::{Site, SiteId};

use crate::opencl::ImageView;

use std::borrow::Cow;

pub struct VoronoiImage<'a> {
    dim: usize,

    sites: Vec<Site>,
    image: Cow<'a, ImageView<i32>>,
}

impl<'a> VoronoiImage<'a> {
    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn sites(&self) -> &Vec<Site> {
        &self.sites
    }

    pub fn image(&self) -> &ImageView<i32> {
        match self.image {
            Cow::Owned(ref image) => image,
            Cow::Borrowed(image) => image,
        }
    }

    pub fn to_pixels(&self) -> anyhow::Result<Vec<Pixel>> {
        Ok(self
            .image
            .to_vec()?
            .chunks_exact(4)
            .enumerate()
            .map(|(i, raw_pixel)| {
                let x = i % self.dim;
                let y = i / self.dim;
                let raw_pixel = [raw_pixel[0] as i64, raw_pixel[1] as i64, raw_pixel[2] as i64];
                Pixel::new(x, y, raw_pixel)
            })
            .collect())
    }

    fn new(dim: usize, sites: Vec<Site>, image: Cow<'a, ImageView<i32>>) -> Self {
        Self { dim, sites, image }
    }
}
