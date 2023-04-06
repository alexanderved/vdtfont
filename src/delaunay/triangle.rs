use super::edge::Edge;

use crate::point::{Point, PointHandle, PointId};

use std::fmt;

use arena_system::{Arena, Handle, Index, RawHandle};
use smallvec::SmallVec;

pub type TriangleId = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct DelaunayTriangle {
    pub vertices: [PointId; 3],
    pub neighbours: [TriangleId; 3],
    pub neighbours_number: i32,
    pub is_visible: bool,
}

impl DelaunayTriangle {
    pub fn new(vertices: [PointId; 3]) -> Self {
        Self { vertices, neighbours: [-1; 3], neighbours_number: 0, is_visible: true }
    }

    pub fn is_counterclockwise(&self, points: &Arena<Point>) -> bool {
        points.handle::<PointHandle>(self.vertices[1].into(), None).skew_product(
            &points.handle(self.vertices[0].into(), None),
            &points.handle(self.vertices[2].into(), None),
        ) < 0.0
    }

    pub fn make_counterclockwise(&mut self, points: &Arena<Point>) {
        if !self.is_counterclockwise(points) {
            let (vertices0, vertices1) = self.vertices[1..].split_at_mut(1);

            std::mem::swap(&mut vertices0[0], &mut vertices1[0]);
        }
    }
}

impl std::default::Default for DelaunayTriangle {
    fn default() -> Self {
        Self::new([-1; 3])
    }
}

unsafe impl ocl::traits::OclPrm for DelaunayTriangle {}

#[derive(Clone, Copy)]
pub struct DelaunayTriangleHandle<'arena> {
    raw: RawHandle<'arena, DelaunayTriangle>,
    points: &'arena Arena<Point>,
}

impl<'arena> DelaunayTriangleHandle<'arena> {
    pub fn points(&self) -> [PointHandle<'arena>; 3] {
        let vertices = self.get().unwrap().vertices;

        [
            self.points.handle(vertices[0].into(), Some(self.arena())),
            self.points.handle(vertices[1].into(), Some(self.arena())),
            self.points.handle(vertices[2].into(), Some(self.arena())),
        ]
    }

    pub fn set_points(&self, points: [PointHandle; 3]) {
        let mut this = self.get_mut().unwrap();

        this.vertices =
            [points[0].index().into(), points[1].index().into(), points[2].index().into()]
    }

    pub fn neighbours(&self) -> SmallVec<[DelaunayTriangleHandle<'arena>; 3]> {
        let neighbour_ids = self.get().unwrap().neighbours;
        neighbour_ids
            .into_iter()
            .filter(|neighbour_id| *neighbour_id != -1)
            .map(|neighbour_id| self.arena().handle(neighbour_id.into(), self.points))
            .collect()
    }

    pub fn set_neighbours(&self, new_neighbours: SmallVec<[DelaunayTriangleHandle<'arena>; 3]>) {
        let neighbours = &mut self.get_mut().unwrap().neighbours;
        *neighbours = [-1; 3];

        neighbours
            .iter_mut()
            .zip(new_neighbours.into_iter())
            .for_each(|(neighbour, new_neighbour)| *neighbour = new_neighbour.index().into());
    }

    pub fn is_visible(&self) -> bool {
        self.get_mut().unwrap().is_visible
    }

    pub fn set_is_visible(&self, is_visible: bool) {
        self.get_mut().unwrap().is_visible = is_visible;
    }

    pub fn edges(&self) -> SmallVec<[Edge<'arena>; 3]> {
        let vertices = self.points();
        (0..3)
            .into_iter()
            .map(|i| i as usize)
            .map(|i| {
                [
                    vertices[i],
                    vertices[(i + 1) % 3],
                ]
            })
            .map(|e| e.into())
            .collect::<SmallVec<[Edge<'arena>; 3]>>()
    }

    pub fn edges_except(&self, exception: Edge<'arena>) -> SmallVec<[Edge<'arena>; 2]> {
        self.edges().into_iter().filter(|edge| edge != &exception).collect()
    }

    pub fn is_neighbour(&self, neighbour: &DelaunayTriangleHandle<'arena>) -> bool {
        self.neighbours().contains(neighbour)
    }

    pub fn try_replace_neighbour(
        &self,
        index: Index,
        new_neighbour: DelaunayTriangleHandle<'arena>,
    ) {
        let neighbour_ids = &mut self.get_mut().unwrap().neighbours;
        let position = neighbour_ids.iter().position(|n| *n == index.into());

        if let Some(position) = position {
            neighbour_ids[position] = new_neighbour.index().into();
        }
    }

    pub fn try_add_neighbour(&self, new_neighbour: DelaunayTriangleHandle<'arena>) -> bool {
        let neighbour_ids = &mut self.get_mut().unwrap().neighbours;
        let position = neighbour_ids.iter().position(|n| *n == -1);

        if let Some(position) = position {
            neighbour_ids[position] = new_neighbour.index().into();

            true
        } else {
            false
        }
    }

    pub fn try_remove_neighbour(&self, index: Index) -> bool {
        let neighbour_ids = &mut self.get_mut().unwrap().neighbours;
        let position = neighbour_ids.iter().position(|n| *n == index.into());

        if let Some(position) = position {
            neighbour_ids[position] = -1i64;

            true
        } else {
            false
        }
    }

    pub fn is_counterclockwise(&self) -> bool {
        let this = self.get().unwrap();

        this.is_counterclockwise(self.points)
    }

    pub fn make_counterclockwise(&mut self) {
        let mut this = self.get_mut().unwrap();

        this.make_counterclockwise(self.points);
    }

    pub fn shared_points_with(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> SmallVec<[PointHandle; 2]> {
        let other_points = other.points();

        self.points()
            .into_iter()
            .filter(|point| other_points.contains(point))
            .collect()
    }

    pub fn opposite_points_with(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> SmallVec<[PointHandle; 2]> {
        let shared_points = self.shared_points_with(other);
        self.points()
            .into_iter()
            .chain(other.points())
            .filter(|point| !shared_points.contains(point))
            .collect()
    }

    pub fn opposite_edge_to(&self, vertex: PointHandle) -> Edge<'arena> {
        self.points()
            .into_iter()
            .filter(|point| *point != vertex)
            .collect::<SmallVec<[PointHandle; 2]>>()
            .into()
    }

    pub fn neighbour_on_edge(&self, edge: Edge) -> DelaunayTriangleHandle<'arena> {
        self.neighbours()
            .into_iter()
            .find(|n| edge == self.shared_points_with(n).into())
            .expect("No neighbour which shares the specified edge")
    }

    pub fn is_in_circle_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let s = self.shared_points_with(other);
        let o = self.opposite_points_with(other);

        let p = [o[0], s[1], o[1], s[0]];

        let center = p[3];
        let mut points = p[0..3]
            .into_iter()
            .map(|p| Point::new(p.x() - center.x(), p.y() - center.y()))
            .collect::<Vec<Point>>();

        points.sort_by(|a, b| {
            libm::atan2f(a.y(), a.x())
                .partial_cmp(&libm::atan2f(b.y(), b.x()))
                .unwrap()
        });

        let p = points.into_iter().rev().collect::<Vec<Point>>();

        let abdet = p[0].x() * p[1].y() - p[1].x() * p[0].y();
        let bcdet = p[1].x() * p[2].y() - p[2].x() * p[1].y();
        let cadet = p[2].x() * p[0].y() - p[0].x() * p[2].y();

        let alift = p[0].x() * p[0].x() + p[0].y() * p[0].y();
        let blift = p[1].x() * p[1].x() + p[1].y() * p[1].y();
        let clift = p[2].x() * p[2].x() + p[2].y() * p[2].y();

        alift * bcdet + blift * cadet + clift * abdet > 0.0
    }

    pub fn is_flippable_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let shared_points = self.shared_points_with(other);
        let opposite_points = self.opposite_points_with(other);

        if !(shared_points.len() == 2 && opposite_points.len() == 2) {
            println!("Bad");
            return false;
        }

        let is_opposite_point_on_bounds =
            opposite_points[0].is_bounding() || opposite_points[1].is_bounding();

        let sp0 = shared_points[0].skew_product(&opposite_points[0], &opposite_points[1]);
        let sp1 = shared_points[1].skew_product(&opposite_points[0], &opposite_points[1]);
        let by_the_same_side_after_flip = sp0.signum() == sp1.signum();

        let has_contour_edge = shared_points[0].previous_in_outline() == shared_points[1]
            || shared_points[1].previous_in_outline() == shared_points[0];

        let satisfies_delaunay_condition = !self.is_in_circle_with(other);

        if by_the_same_side_after_flip {
            return false;
        }

        if has_contour_edge {
            return false;
        }

        if is_opposite_point_on_bounds {
            return false;
        }

        if satisfies_delaunay_condition {
            return false;
        }

        true
    }

    pub fn flip_with(
        &mut self,
        other: &mut DelaunayTriangleHandle<'arena>,
        mut deep: usize,
    ) -> bool {
        let is_flippable = self.is_flippable_with(other) && deep != 0;
        if is_flippable {
            self.flip_edge(other);
            self.update_neighbours(other);

            deep -= 1;

            self.flip_with_neighbours_except(Some(*other), deep);
            other.flip_with_neighbours_except(Some(*self), deep);
        }

        is_flippable
    }

    fn flip_edge(&mut self, other: &mut DelaunayTriangleHandle<'arena>) {
        {
            let shared_points = self.shared_points_with(other);
            let opposite_points = self.opposite_points_with(other);

            self.set_points([shared_points[0], opposite_points[0], opposite_points[1]]);
            other.set_points([shared_points[1], opposite_points[0], opposite_points[1]]);
        }

        self.make_counterclockwise();
        other.make_counterclockwise();
    }

    fn update_neighbours(&self, other: &DelaunayTriangleHandle<'arena>) {
        let mut neighbourhood = self.neighbours();
        let mut other_neighbours = other
            .neighbours()
            .into_iter()
            .filter(|n| !neighbourhood.contains(n))
            .collect::<SmallVec<[_; 3]>>();
        neighbourhood.append(&mut other_neighbours);

        let triangles = [*self, *other];
        for i in 0..2 {
            let triangle = triangles[i];
            let other_triangle = triangles[(i + 1) % 2];

            let new_neighbours = neighbourhood
                .iter()
                .copied()
                .filter(|neighbour| triangle.shared_points_with(&neighbour).len() == 2)
                .collect::<SmallVec<[DelaunayTriangleHandle; 3]>>();

            new_neighbours
                .iter()
                .for_each(|n| n.try_replace_neighbour(other_triangle.index(), triangle));

            triangle.set_neighbours(new_neighbours);
        }
    }

    pub fn flip_with_neighbours_except(
        &mut self,
        exception: Option<DelaunayTriangleHandle>,
        deep: usize,
    ) -> bool {
        self.neighbours()
            .into_iter()
            .filter(
                |neighbour| {
                    if let Some(exception) = exception {
                        *neighbour != exception
                    } else {
                        true
                    }
                },
            )
            .any(|mut neighbour| self.flip_with(&mut neighbour, deep))
    }
}

impl<'arena> Handle<'arena> for DelaunayTriangleHandle<'arena> {
    type Type = DelaunayTriangle;
    type Userdata = &'arena Arena<Point>;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> Self {
        Self { raw, points: userdata }
    }

    fn to_raw(&self) -> RawHandle<'arena, Self::Type> {
        self.raw
    }
}

impl fmt::Debug for DelaunayTriangleHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("DelaunayTriangleHandle({:?})", self.to_raw()))
    }
}

impl PartialEq for DelaunayTriangleHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw() == other.to_raw()
    }
}

impl Eq for DelaunayTriangleHandle<'_> {}

impl PartialOrd for DelaunayTriangleHandle<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_raw().partial_cmp(&other.to_raw())
    }
}

impl Ord for DelaunayTriangleHandle<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_raw().cmp(&other.to_raw())
    }
}
