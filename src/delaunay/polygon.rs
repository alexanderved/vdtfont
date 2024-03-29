use super::DelaunayTriangleHandle;

use crate::point::Point;
use crate::point::PointHandle;

use std::collections::HashSet;
use std::convert;

/// A set of points.
pub struct Polygon<'arena> {
    points: Vec<PointHandle<'arena>>,
}

impl<'arena> Polygon<'arena> {
    /// Creates a new [`Polygon`] from the given points.
    pub fn new(points: Vec<PointHandle<'arena>>) -> Self {
        Self { points }
    }

    /// Creates a new [`Polygon`] from the vertices of the given triangles.
    pub fn from_triangles(triangles: &[DelaunayTriangleHandle<'arena>]) -> Self {
        triangles
            .iter()
            .flat_map(|t| t.points())
            .collect::<HashSet<PointHandle>>()
            .into_iter()
            .collect::<Vec<PointHandle>>()
            .into()
    }

    /// Returns a reference to the points in the polygon.
    pub fn points(&self) -> &Vec<PointHandle<'arena>> {
        &self.points
    }

    /// Sorts the points in the polygon with the origin at `origin` by angle.
    pub fn sort_by_angle(&mut self, origin: PointHandle<'arena>) {
        self.points.sort_by(|a, b| {
            let a = Point::new(a.x() - origin.x(), a.y() - origin.y());
            let b = Point::new(b.x() - origin.x(), b.y() - origin.y());

            libm::atan2f(a.y(), a.x())
                .partial_cmp(&libm::atan2f(b.y(), b.x()))
                .unwrap()
        });
    }
}

impl<'arena> convert::From<Vec<PointHandle<'arena>>> for Polygon<'arena> {
    fn from(points: Vec<PointHandle<'arena>>) -> Self {
        Self::new(points)
    }
}

impl<'arena> convert::From<Vec<DelaunayTriangleHandle<'arena>>> for Polygon<'arena> {
    fn from(triangles: Vec<DelaunayTriangleHandle<'arena>>) -> Self {
        Self::from_triangles(&triangles)
    }
}

impl<'arena> convert::From<&Vec<DelaunayTriangleHandle<'arena>>> for Polygon<'arena> {
    fn from(triangles: &Vec<DelaunayTriangleHandle<'arena>>) -> Self {
        Self::from_triangles(triangles)
    }
}

impl<'arena> convert::From<Polygon<'arena>> for Vec<PointHandle<'arena>> {
    fn from(polygon: Polygon<'arena>) -> Self {
        polygon.points
    }
}
