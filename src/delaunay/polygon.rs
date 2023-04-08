use super::DelaunayTriangleHandle;

use crate::point::PointHandle;
use crate::point::Point;

use std::collections::HashSet;
use std::convert;

pub struct Polygon<'arena> {
    points: Vec<PointHandle<'arena>>,
}

impl<'arena> Polygon<'arena> {
    pub fn new(points: Vec<PointHandle<'arena>>) -> Self {
        Self { points }
    }

    pub fn from_triangles(triangles: &Vec<DelaunayTriangleHandle<'arena>>) -> Self {
        triangles
            .iter()
            .flat_map(|t| t.points())
            .collect::<HashSet<PointHandle>>()
            .into_iter()
            .collect::<Vec<PointHandle>>()
            .into()
    }

    pub fn points(&self) -> &Vec<PointHandle<'arena>> {
        &self.points
    }

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

impl<'arena> convert::Into<Vec<PointHandle<'arena>>> for Polygon<'arena> {
    fn into(self) -> Vec<PointHandle<'arena>> {
        self.points
    }
}
