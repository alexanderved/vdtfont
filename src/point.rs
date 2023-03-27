use crate::ocl::prm::Float2;

use std::cmp::{Eq, PartialEq};

use arena_system::{Handle, RawHandle};

pub type PointId = i64;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    coords: Float2,
    is_bounding: bool,
    previous_in_outline: PointId,
}

impl Point {
    pub fn new(x: f32, y: f32, is_bounding: bool, previous_in_outline: PointId) -> Self {
        Self { coords: Float2::new(x, y), is_bounding, previous_in_outline }
    }

    pub fn x(&self) -> f32 {
        self.coords[0]
    }

    pub fn y(&self) -> f32 {
        self.coords[1]
    }

    pub fn coords(&self) -> Float2 {
        self.coords
    }

    pub fn is_bounding(&self) -> bool {
        self.is_bounding
    }

    pub fn cross_product(&self, origin: &Self, other: &Self) -> f32 {
        let a = Self::new(self.x() - origin.x(), self.y() - origin.y(), false, -1);
        let b = Self::new(other.x() - origin.x(), other.y() - origin.y(), false, -1);

        a.x() * b.y() - a.y() * b.x()
    }
}

#[derive(Debug, Clone)]
pub struct PointHandle<'arena> {
    raw: RawHandle<'arena, Point>,
}

impl<'arena> PointHandle<'arena> {
    pub fn x(&self) -> f32 {
        self.get().unwrap().coords[0]
    }

    pub fn y(&self) -> f32 {
        self.get().unwrap().coords[1]
    }

    pub fn coords(&self) -> Float2 {
        self.get().unwrap().coords
    }

    pub fn is_bounding(&self) -> bool {
        self.get().unwrap().is_bounding
    }

    pub fn cross_product(&self, origin: &Self, other: &Self) -> f32 {
        let this = self.get().unwrap();
        let origin = origin.get().unwrap();
        let other = other.get().unwrap();

        this.cross_product(&origin, &other)
    }
}

impl<'arena> Handle<'arena> for PointHandle<'arena> {
    type Type = Point;
    type Userdata = ();

    fn from_raw(raw: RawHandle<'arena, Self::Type>, _userdata: Self::Userdata) -> Self {
        Self { raw }
    }

    fn as_raw(&self) -> &RawHandle<'arena, Self::Type> {
        &self.raw
    }
}

impl PartialEq for PointHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl Eq for PointHandle<'_> {}
