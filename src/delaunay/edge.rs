use super::DelaunayTriangleHandle;

use crate::point::PointHandle;

use std::convert;
use std::ops::ControlFlow;

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

    pub fn find_triangle_track(&'arena self) -> Vec<DelaunayTriangleHandle<'arena>> {
        let res = self.points()[0].triangle_fan().into_iter().try_for_each(|t| {
            let opposite_edge = t.opposite_edge_to(self.points()[0]);

            if t.points().contains(&self.points()[1]) || opposite_edge.intersects(&self) {
                return ControlFlow::Break((opposite_edge, t));
            }

            ControlFlow::Continue(())
        });

        let (mut e, mut t) = match res {
            ControlFlow::Break((e, t)) => (e, t),
            _ => panic!("Triangle not found"),
        };
        let mut triangles = vec![t];

        if t.points().contains(&self.points()[1]) {
            return triangles;
        }

        loop {
            let n = t.neighbour_on_edge(e);
            triangles.push(n);

            if n.points().contains(&self.points()[1]) {
                return triangles;
            }

            let edges = n.edges_except(e);

            if edges[0].intersects(&self) {
                e = edges[0];
            } else if edges[1].intersects(&self) {
                e = edges[1];
            }

            t = n;
        }
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
