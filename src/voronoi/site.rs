use crate::ocl::prm::Float2;

pub type SiteId = i64;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Site(pub Float2);

impl Site {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Float2::new(x, y))
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn coords(&self) -> Float2 {
        self.0
    }
}
