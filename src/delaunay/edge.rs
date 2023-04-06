use crate::point::PointHandle;

use std::convert;

use smallvec::SmallVec;

#[derive(Debug, Clone, Copy)]
pub struct Edge<'arena> {
    points: [PointHandle<'arena>; 2],
}

impl<'arena> Edge<'arena> {
    pub fn new(points: [PointHandle<'arena>; 2]) -> Self {
        Self { points }
    }

    pub fn points(&self) -> [PointHandle<'arena>; 2] {
        self.points
    }

    pub fn contains(&self, point: PointHandle<'arena>) -> bool {
        self.points.contains(&point)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let det = (self.points[1].x() - self.points[0].x())
            * (other.points[1].y() - other.points[0].y())
            - (other.points[1].x() - other.points[0].x())
                * (self.points[1].y() - self.points[0].y());

        if det == 0.0 {
            false
        } else {
            let lambda = ((other.points[1].y() - other.points[0].y())
                * (other.points[1].x() - self.points[0].x())
                - (other.points[1].x() - other.points[0].x())
                    * (other.points[1].y() - self.points[0].y()))
                / det;
            let gamma = ((self.points[0].y() - self.points[1].y())
                * (other.points[1].x() - self.points[0].x())
                + (self.points[1].x() - self.points[0].x())
                    * (other.points[1].y() - self.points[0].y()))
                / det;

            (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0)
        }
    }

    pub fn is_equal_to(&self, other: &Self) -> bool {
        self.points[0] == other.points[0] && self.points[1] == other.points[1]
            || self.points[0] == other.points[1] && self.points[1] == other.points[0]
    }
}

impl PartialEq for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal_to(other)
    }
}

impl Eq for Edge<'_> {}

impl<'arena> convert::From<[PointHandle<'arena>; 2]> for Edge<'arena> {
    fn from(points: [PointHandle<'arena>; 2]) -> Self {
        Self::new(points)
    }
}

impl<'arena> convert::From<SmallVec<[PointHandle<'arena>; 2]>> for Edge<'arena> {
    fn from(points: SmallVec<[PointHandle<'arena>; 2]>) -> Self {
        Self::new(points.into_inner().expect("SmallVec is larger than [PointHandle; 2]"))
    }
}

impl<'arena> convert::Into<[PointHandle<'arena>; 2]> for Edge<'arena> {
    fn into(self) -> [PointHandle<'arena>; 2] {
        self.points
    }
}

impl<'arena> convert::Into<SmallVec<[PointHandle<'arena>; 2]>> for Edge<'arena> {
    fn into(self) -> SmallVec<[PointHandle<'arena>; 2]> {
        self.points.into()
    }
}
