use crate::point::PointHandle;

use std::convert;

pub struct Edge<'arena> {
    points: [PointHandle<'arena>; 2],
}

impl<'arena> Edge<'arena> {
    fn new(points: [PointHandle<'arena>; 2]) -> Self {
        Self {
            points
        }
    }

    pub fn intersects(&self, other: Self) -> bool {
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
}

impl<'arena> convert::From<[PointHandle<'arena>; 2]> for Edge<'arena> {
    fn from(points: [PointHandle<'arena>; 2]) -> Self {
        Self::new(points)
    }
}