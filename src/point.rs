use crate::delaunay::{DelaunayTriangle, DelaunayTriangleHandle, TriangleId};
use crate::ocl::prm::Float2;

use std::fmt;
use std::hash;

use arena_system::{Arena, Handle, Index, RawHandle};
use smallvec::SmallVec;

pub type PointId = i64;

/// A (x; y) coordinate with additional information.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point {
    coords: Float2,

    is_bounding: bool,
    previous_in_outline: PointId,

    triangle_fan: SmallVec<[TriangleId; 6]>,
}

impl Point {
    /// Creates a new [`Point`].
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            coords: Float2::new(x, y),

            is_bounding: false,
            previous_in_outline: -1,

            triangle_fan: SmallVec::new(),
        }
    }

    /// Creates a new [`Point`] with the information if it is bounding or not.
    pub fn with_is_bounding(x: f32, y: f32, is_bounding: bool) -> Self {
        Self {
            coords: Float2::new(x, y),

            is_bounding,
            previous_in_outline: -1,

            triangle_fan: SmallVec::new(),
        }
    }

    /// Creates a new [`Point`] with the information about the previous one in the outline.
    pub fn with_previous(x: f32, y: f32, previous_in_outline: PointId) -> Self {
        Self {
            coords: Float2::new(x, y),

            is_bounding: false,
            previous_in_outline,

            triangle_fan: SmallVec::new(),
        }
    }

    /// Creates a new [`Point`] with the information about the previous one in the outline
    /// and if it is bounding or not.
    pub fn with_is_bounding_and_previous(
        x: f32,
        y: f32,
        is_bounding: bool,
        previous_in_outline: PointId,
    ) -> Self {
        Self {
            coords: Float2::new(x, y),

            is_bounding,
            previous_in_outline,

            triangle_fan: SmallVec::new(),
        }
    }

    /// Returns the `x` coordinate of the point.
    pub fn x(&self) -> f32 {
        self.coords[0]
    }

    /// Returns the `y` coordinate of the point.
    pub fn y(&self) -> f32 {
        self.coords[1]
    }

    pub fn coords(&self) -> Float2 {
        self.coords
    }

    /// Returns coordinates of the point.
    pub fn set_coords(&mut self, coords: Float2) {
        self.coords = coords;
    }

    /// Checks if the point is bounding.
    pub fn is_bounding(&self) -> bool {
        self.is_bounding
    }

    /// Returns the previous point in the outline.
    pub fn previous_in_outline(&self) -> PointId {
        self.previous_in_outline
    }

    /// Sets the previous point in the outline.
    pub fn set_previous_in_outline(&mut self, previous_in_outline: PointId) {
        self.previous_in_outline = previous_in_outline;
    }

    /// Returns the triangle fan of the point.
    pub fn triangle_fan(&self) -> &SmallVec<[TriangleId; 6]> {
        &self.triangle_fan
    }

    /// Sets the triangle fan of the point.
    pub fn set_triangle_fan(&mut self, triangle_fan: SmallVec<[TriangleId; 6]>) {
        self.triangle_fan = triangle_fan;
    }

    /// Returns a point between two points.
    pub fn midpoint(&self, other: &Point) -> Point {
        Point::new((self.x() + other.x()) / 2.0, (self.y() + other.y()) / 2.0)
    }

    /// Returns the squared distance between two points.
    pub fn distance_squared(&self, other: &Point) -> f32 {
        let p = Point::new(self.x() - other.x(), self.y() - other.y());

        p.x().powi(2) + p.y().powi(2)
    }

    /// Returns the distance between two points.
    pub fn distance(&self, other: &Point) -> f32 {
        self.distance_squared(other).sqrt()
    }
}

/// A handle of the [`Point`] which is used by [`Arena`].
#[derive(Clone, Copy)]
pub struct PointHandle<'arena> {
    raw: RawHandle<'arena, Point>,
    triangles: Option<&'arena Arena<DelaunayTriangle>>,
}

impl<'arena> PointHandle<'arena> {
    /// Returns the `x` coordinate of the point.
    pub fn x(&self) -> f32 {
        self.get().unwrap().coords[0]
    }

    /// Returns the `y` coordinate of the point.
    pub fn y(&self) -> f32 {
        self.get().unwrap().coords[1]
    }

    /// Returns coordinates of the point.
    pub fn coords(&self) -> Float2 {
        self.get().unwrap().coords
    }

    /// Sets the coordinates of the point.
    pub fn set_coords(&mut self, coords: Float2) {
        self.get_mut().unwrap().coords = coords;
    }

    /// Checks if the point is bounding.
    pub fn is_bounding(&self) -> bool {
        self.get().unwrap().is_bounding
    }

    /// Returns the previous point in the outline.
    pub fn previous_in_outline(&self) -> PointHandle<'arena> {
        let this = self.get().expect("Can't get the point");

        self.arena().handle(this.previous_in_outline().into(), self.triangles)
    }

    /// Returns the triangle fan of the point.
    pub fn triangle_fan(&self) -> SmallVec<[DelaunayTriangleHandle<'arena>; 6]> {
        self.get()
            .unwrap()
            .triangle_fan
            .iter()
            .copied()
            .map(|i| i.into())
            .map(|i| self.triangles.unwrap().handle(i, self.arena()))
            .collect()
    }

    /// Sets the triangle fan of the point.
    pub fn set_triangle_fan(
        &mut self,
        triangle_fan: SmallVec<[DelaunayTriangleHandle<'arena>; 6]>,
    ) {
        self.get_mut().unwrap().triangle_fan =
            triangle_fan.into_iter().map(|h| h.index().into()).collect();
    }

    /// Adds `triangle` to the triangle fan of the point.
    pub fn add_triangle_to_fan(&self, triangle: DelaunayTriangleHandle) {
        let triangle_fan = &mut self.get_mut().unwrap().triangle_fan;

        triangle_fan.push(triangle.index().into());
    }

    /// Removes `triangle` from the triangle fan of the point.
    pub fn remove_triangle_from_fan(&self, triangle_index: Index) {
        let triangle_fan = &mut self.get_mut().unwrap().triangle_fan;
        let position = triangle_fan.iter().position(|t| *t == triangle_index.into());

        if let Some(position) = position {
            triangle_fan.remove(position);
        }
    }

    /// Checks if the point is connected to the other one.
    pub fn is_connected_to(&self, other: PointHandle<'arena>) -> bool {
        self.triangle_fan().into_iter().any(|t| t.points().contains(&other)) && *self != other
    }

    /// Calculates skew product of `self` and `other` points
    /// with the origin of coordinates at `origin`
    pub fn skew_product(&self, origin: &Self, other: &Self) -> f32 {
        let a = Point::new(self.x() - origin.x(), self.y() - origin.y());
        let b = Point::new(other.x() - origin.x(), other.y() - origin.y());

        a.x() * b.y() - a.y() * b.x()
    }

    /// Returns the distance between two points.
    pub fn distance(&self, other: &PointHandle) -> f32 {
        let this = self.get().expect("Can't get the point");
        let other = other.get().expect("Can't get the other point");

        this.distance(&other)
    }
}

impl<'arena> Handle<'arena> for PointHandle<'arena> {
    type Type = Point;
    type Userdata = Option<&'arena Arena<DelaunayTriangle>>;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> Self {
        Self { raw, triangles: userdata }
    }

    fn to_raw(&self) -> RawHandle<'arena, Self::Type> {
        self.raw
    }
}

impl fmt::Debug for PointHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PointHandle({:?})", self.to_raw()))
    }
}

impl PartialEq for PointHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw() == other.to_raw()
    }
}
impl Eq for PointHandle<'_> {}

impl PartialOrd for PointHandle<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_raw().partial_cmp(&other.to_raw())
    }
}

impl Ord for PointHandle<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_raw().cmp(&other.to_raw())
    }
}

impl hash::Hash for PointHandle<'_> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let index: i64 = self.index().into();
        index.hash(state);
    }
}
