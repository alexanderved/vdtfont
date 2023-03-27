use crate::ocl::prm::Float2;

use arena_system::{Handle, RawHandle};

pub(super) type PointId = i64;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DelaunayPoint {
    coords: Float2,
    is_bounding: bool,
}

impl DelaunayPoint {
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

pub struct DelaunayPointHandle<'arena> {
    raw: RawHandle<'arena, DelaunayPoint>,
}

impl<'arena> DelaunayPointHandle<'arena> {
    pub fn cross_product(&self, origin: &Self, other: &Self) -> f32 {
        let this = self.get().unwrap();
        let origin = origin.get().unwrap();
        let other = other.get().unwrap();

        this.cross_product(&origin, &other)
    }
}

impl<'arena> Handle<'arena> for DelaunayPointHandle<'arena> {
    type Type = DelaunayPoint;
    type Userdata = ();

    fn from_raw(raw: RawHandle<'arena, Self::Type>, _userdata: Self::Userdata) -> Self {
        Self { raw }
    }

    fn as_raw(&self) -> &RawHandle<'arena, Self::Type> {
        &self.raw
    }

    fn as_mut_raw(&mut self) -> &mut RawHandle<'arena, Self::Type> {
        &mut self.raw
    }
}
