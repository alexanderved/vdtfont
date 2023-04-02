use crate::ocl::prm::Float2;

use arena_system::{Handle, RawHandle};

pub type PointId = i64;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    coords: Float2,
    is_bounding: bool,
    previous_in_outline: PointId,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { coords: Float2::new(x, y), is_bounding: false, previous_in_outline: -1 }
    }

    pub fn with_is_bounding(x: f32, y: f32, is_bounding: bool) -> Self {
        Self { coords: Float2::new(x, y), is_bounding, previous_in_outline: -1 }
    }

    pub fn with_previous(x: f32, y: f32, previous_in_outline: PointId) -> Self {
        Self { coords: Float2::new(x, y), is_bounding: false, previous_in_outline }
    }

    pub fn with_is_bounding_and_previous(
        x: f32,
        y: f32,
        is_bounding: bool,
        previous_in_outline: PointId,
    ) -> Self {
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

    pub fn set_coords(&mut self, coords: Float2) {
        self.coords = coords;
    }

    pub fn is_bounding(&self) -> bool {
        self.is_bounding
    }

    pub fn previous_in_outline(&self) -> PointId {
        self.previous_in_outline
    }

    pub fn set_previous_in_outline(&mut self, previous_in_outline: PointId) {
        self.previous_in_outline = previous_in_outline;
    }

    pub fn midpoint(&self, other: &Point) -> Point {
        Point::new((self.x() + other.x()) / 2.0, (self.y() + other.y()) / 2.0)
    }

    pub fn distance_squared(&self, other: &Point) -> f32 {
        let p = Point::new(self.x() - other.x(), self.y() - other.y());

        p.x().powi(2) + p.y().powi(2)
    }

    pub fn distance(&self, other: &Point) -> f32 {
        self.distance_squared(other).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn set_coords(&mut self, coords: Float2) {
        self.get_mut().unwrap().coords = coords;
    }

    pub fn is_bounding(&self) -> bool {
        self.get().unwrap().is_bounding
    }

    pub fn previous_in_outline(&self) -> PointHandle<'arena> {
        let this = self.get().expect("Can't get the point");

        self.arena().handle(this.previous_in_outline().into(), ())
    }

    pub fn skew_product(&self, origin: &Self, other: &Self) -> f32 {
        let a = Point::new(self.x() - origin.x(), self.y() - origin.y());
        let b = Point::new(other.x() - origin.x(), other.y() - origin.y());

        a.x() * b.y() - a.y() * b.x()
    }
}

impl<'arena> Handle<'arena> for PointHandle<'arena> {
    type Type = Point;
    type Userdata = ();

    fn from_raw(raw: RawHandle<'arena, Self::Type>, _userdata: Self::Userdata) -> Self {
        Self { raw }
    }

    fn to_raw(&self) -> RawHandle<'arena, Self::Type> {
        self.raw
    }
}
