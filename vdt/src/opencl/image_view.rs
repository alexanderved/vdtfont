use std::{convert, ops};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct ImageView<T: ocl::OclPrm> {
    ocl_image: ocl::Image<T>,

    origin: Dims,
    region: Dims,
}

impl<T: ocl::OclPrm> ImageView<T> {
    pub fn new<D: convert::Into<Dims>>(
        ocl_image: ocl::Image<T>,
        origin: Option<D>,
        region: Option<D>,
    ) -> anyhow::Result<Self> {
        let full_image_region: Dims = ocl_image.dims().into();

        let origin: Dims = origin.map(D::into).unwrap_or([0; 3].into());
        let region: Dims = region.map(D::into).unwrap_or(full_image_region);

        if !is_region_suitable_for_image(&ocl_image, origin, region) {
            anyhow::bail!(
                "The given region {:?} which starts at the origin {:?}
                exceeds the size of the full image {:?}",
                region,
                origin,
                full_image_region
            );
        }

        Ok(Self {
            ocl_image,

            origin,
            region,
        })
    }

    pub fn ocl_image(&self) -> &ocl::Image<T> {
        &self.ocl_image
    }

    pub fn origin(&self) -> Dims {
        self.origin
    }

    pub fn region(&self) -> Dims {
        self.region
    }

    pub fn set_origin<O: convert::Into<Dims>>(&mut self, origin: O) -> anyhow::Result<()> {
        let origin: Dims = origin.into();

        if !is_region_suitable_for_image(&self.ocl_image, origin, self.region) {
            anyhow::bail!("The given origin {:?} isn't suitable for image", origin);
        }

        self.origin = origin;

        Ok(())
    }

    pub fn set_region<R: convert::Into<Dims>>(&mut self, region: R) -> anyhow::Result<()> {
        let region = region.into();

        if !is_region_suitable_for_image(&self.ocl_image, self.origin, region) {
            anyhow::bail!("The given region {:?} isn't suitable for image", region);
        }

        self.region = region;

        Ok(())
    }

    pub fn read(&self, dst_data: &mut [T]) -> anyhow::Result<()> {
        self.ocl_image
            .cmd()
            .origin(self.origin)
            .region(self.region)
            .read(dst_data)
            .enq()?;

        Ok(())
    }

    pub fn write(&self, src_data: &[T]) -> anyhow::Result<()> {
        self.ocl_image
            .cmd()
            .origin(self.origin)
            .region(self.region)
            .write(src_data)
            .enq()?;

        Ok(())
    }

    pub fn deepcopy(&self) -> anyhow::Result<Self> {
        let queue = self
            .ocl_image
            .default_queue()
            .context("The original image is expected to have a default queue")?
            .clone();

        let copied_image = ocl::Image::builder()
            .queue(queue)
            .channel_data_type(ocl::enums::ImageChannelDataType::SignedInt32)
            .channel_order(ocl::enums::ImageChannelOrder::Rgba)
            .image_type(ocl::enums::MemObjectType::Image2d)
            .dims(self.ocl_image.dims())
            .build()?;
        self.ocl_image.cmd().copy(&copied_image, [0; 3]).enq()?;

        Ok(Self {
            ocl_image: copied_image,

            origin: self.origin,
            region: self.region,
        })
    }

    pub fn to_vec(&self) -> anyhow::Result<Vec<T>> {
        let mut data = vec![
            T::default();
            self.region[0]
                * self.region[1]
                * self.region[2]
                * self.ocl_image.pixel_element_len()
        ];
        self.read(&mut data)?;

        Ok(data)
    }
}

fn is_region_suitable_for_image<T: ocl::OclPrm>(
    ocl_image: &ocl::Image<T>,
    origin: Dims,
    region: Dims,
) -> bool {
    let full_image_region: Dims = ocl_image.dims().into();

    const MAX_DIM_COUNT: usize = 3;
    for i in 0..MAX_DIM_COUNT {
        if origin[i] + region[i] > full_image_region[i] {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Dims([usize; 3]);

impl Dims {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self([x, y, z])
    }
}

impl convert::From<usize> for Dims {
    fn from(x: usize) -> Self {
        Self::new(x, 1, 1)
    }
}

impl convert::From<(usize,)> for Dims {
    fn from(dims: (usize,)) -> Self {
        Self::new(dims.0, 1, 1)
    }
}

impl convert::From<[usize; 1]> for Dims {
    fn from(dims: [usize; 1]) -> Self {
        Self::new(dims[0], 1, 1)
    }
}

impl convert::From<(usize, usize)> for Dims {
    fn from(dims: (usize, usize)) -> Self {
        Self::new(dims.0, dims.1, 1)
    }
}

impl convert::From<[usize; 2]> for Dims {
    fn from(dims: [usize; 2]) -> Self {
        Self::new(dims[0], dims[1], 1)
    }
}

impl convert::From<(usize, usize, usize)> for Dims {
    fn from(dims: (usize, usize, usize)) -> Self {
        Self::new(dims.0, dims.1, dims.2)
    }
}

impl convert::From<[usize; 3]> for Dims {
    fn from(dims: [usize; 3]) -> Self {
        Self::new(dims[0], dims[1], dims[2])
    }
}

impl convert::From<ocl::SpatialDims> for Dims {
    fn from(spatial_dims: ocl::SpatialDims) -> Self {
        match spatial_dims.to_lens() {
            Ok(dims) => dims.into(),
            Err(_) => [1; 3].into(),
        }
    }
}

impl convert::From<&ocl::SpatialDims> for Dims {
    fn from(spatial_dims: &ocl::SpatialDims) -> Self {
        match spatial_dims.to_lens() {
            Ok(dims) => dims.into(),
            Err(_) => [1; 3].into(),
        }
    }
}

impl convert::Into<[usize; 3]> for Dims {
    fn into(self) -> [usize; 3] {
        self.0
    }
}

impl convert::Into<ocl::SpatialDims> for Dims {
    fn into(self) -> ocl::SpatialDims {
        self.0.into()
    }
}

impl ops::Index<usize> for Dims {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Dims {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
