use crate::ocl::prm::Float2;

pub(super) type PointId = i32;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    coords: Float2,
    is_bounding: bool,
}

impl Point {
    pub fn new(x: f32, y: f32, is_bounding: bool) -> Self {
        Self { coords: Float2::new(x, y), is_bounding }
    }

    pub fn x(&self) -> f32 {
        self.coords[0]
    }

    pub fn y(&self) -> f32 {
        self.coords[1]
    }

    pub fn cross_product(&self, origin: &Self, other: &Self) -> f32 {
        let a = Self::new(self.x() - origin.x(), self.y() - origin.y(), false);
        let b = Self::new(other.x() - origin.x(), other.y() - origin.y(), false);

        a.x() * b.y() - a.y() * b.x()
    }
}
