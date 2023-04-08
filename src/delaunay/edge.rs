use super::DelaunayTriangleHandle;

use crate::point::PointHandle;

use std::convert;
use std::ops::ControlFlow;

use arena_system::Handle;
use smallvec::SmallVec;

/// A straight line which is bounded by two points.
#[derive(Debug, Clone, Copy)]
pub struct Edge<'arena> {
    points: [PointHandle<'arena>; 2],
}

impl<'arena> Edge<'arena> {
    /// Creates a new [`Edge`] from the given `points`.
    pub fn new(points: [PointHandle<'arena>; 2]) -> Self {
        Self { points }
    }

    /// Returns the points which bounds the edge.
    pub fn points(&self) -> [PointHandle<'arena>; 2] {
        self.points
    }

    /// Checks if the edge is in contour.
    pub fn is_contour(&self) -> bool {
        (self.points[0] == self.points[1].previous_in_outline()
            && !self.points[0].index().is_invalid())
            || (self.points[1] == self.points[0].previous_in_outline()
                && !self.points[1].index().is_invalid())
    }

    /// Checks if the edge contains the `point`.
    pub fn contains(&self, point: PointHandle<'arena>) -> bool {
        self.points.contains(&point)
    }

    /// Checks if the edge intersects the `other` edge.
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

    /// Checks if the edge is equal to the `other` edge.
    pub fn is_equal_to(&self, other: &Self) -> bool {
        self.points[0] == other.points[0] && self.points[1] == other.points[1]
            || self.points[0] == other.points[1] && self.points[1] == other.points[0]
    }

    /// Finds all edges and triangles which are intersected by the edge.
    pub fn find_triangle_track(
        &'arena self,
    ) -> (Vec<Edge<'arena>>, Vec<DelaunayTriangleHandle<'arena>>) {
        // Find the first edge and triangle which are intersected by the edge.
        let res = self.points()[0].triangle_fan().into_iter().try_for_each(|t| {
            let opposite_edge = t.opposite_edge_to(self.points()[0]);
            if t.points().contains(&self.points()[1]) || opposite_edge.intersects(self) {
                return ControlFlow::Break((opposite_edge, t));
            }

            ControlFlow::Continue(())
        });

        let (mut e, mut t) = match res {
            ControlFlow::Break((e, t)) => (e, t),
            _ => panic!("Couldn't find any triangles in track"),
        };
        let mut edges = vec![e];
        let mut triangles = vec![t];

        // If the first triangle contains the whole edge, return from the function.
        if t.points().contains(&self.points()[1]) {
            return (edges, triangles);
        }

        loop {
            // Obtain the next triangle which is intersected by the edge.
            let n = t.neighbour_on_edge(e);
            triangles.push(n);

            // If the triangle contains the whole edge, all intersected triangles are found,
            // so return from the function.
            if n.points().contains(&self.points()[1]) {
                return (edges, triangles);
            }

            let es = n.edges_except(e);

            // Obtain the next edge which is intersected by the edge.
            if es[0].intersects(self) {
                e = es[0];
            } else if es[1].intersects(self) {
                e = es[1];
            }

            edges.push(e);

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

impl<'arena> convert::From<Edge<'arena>> for [PointHandle<'arena>; 2] {
    fn from(edge: Edge<'arena>) -> Self {
        edge.points
    }
}

impl<'arena> convert::From<Edge<'arena>> for SmallVec<[PointHandle<'arena>; 2]> {
    fn from(edge: Edge<'arena>) -> Self {
        edge.points.into()
    }
}