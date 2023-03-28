#![allow(unused)]

use crate::point::{Point, PointHandle, PointId};

use arena_system::{Arena, Handle, Index, RawHandle};
use smallvec::SmallVec;

pub(super) type TriangleId = i64;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(super) struct DelaunayTriangle {
    pub(super) vertices: [PointId; 3],
    pub(super) neighbours: [TriangleId; 3],
    pub(super) neighbours_number: i32,
}

impl DelaunayTriangle {
    pub(super) fn new(vertices: [PointId; 3]) -> Self {
        Self { vertices, neighbours: [-1; 3], neighbours_number: 0 }
    }

    pub(super) fn is_counterclockwise(&self, points: &Arena<Point>) -> bool {
        points.handle::<PointHandle>(self.vertices[1].into(), ()).cross_product(
            &points.handle(self.vertices[0].into(), ()),
            &points.handle(self.vertices[2].into(), ()),
        ) < 0.0
    }

    pub(super) fn make_counterclockwise(&mut self, points: &Arena<Point>) {
        if !self.is_counterclockwise(points) {
            unsafe {
                let vertex1 = &mut self.vertices[1] as *mut _;
                let vertex2 = &mut self.vertices[2] as *mut _;

                std::mem::swap(&mut *vertex1, &mut *vertex2);
            }
        }
    }
}

impl std::default::Default for DelaunayTriangle {
    fn default() -> Self {
        Self::new([-1; 3])
    }
}

unsafe impl ocl::traits::OclPrm for DelaunayTriangle {}

#[derive(Debug, Clone)]
pub(super) struct DelaunayTriangleHandle<'arena> {
    raw: RawHandle<'arena, DelaunayTriangle>,
    points: &'arena Arena<Point>,
}

impl<'arena> DelaunayTriangleHandle<'arena> {
    pub(super) fn points(&self) -> [PointHandle<'arena>; 3] {
        let vertices = self.get().unwrap().vertices;

        [
            self.points.handle(vertices[0].into(), ()),
            self.points.handle(vertices[1].into(), ()),
            self.points.handle(vertices[2].into(), ()),
        ]
    }

    pub(super) fn set_points(&self, points: [PointHandle; 3]) {
        let mut this = self.get_mut().unwrap();

        this.vertices =
            [points[0].index().into(), points[1].index().into(), points[2].index().into()]
    }

    pub(super) fn neighbours(&self) -> SmallVec<[DelaunayTriangleHandle<'arena>; 3]> {
        let neighbour_ids = self.get().unwrap().neighbours;
        neighbour_ids
            .into_iter()
            .filter(|neighbour_id| *neighbour_id != -1)
            .map(|neighbour_id| self.arena().handle(neighbour_id.into(), self.points))
            .collect()
    }

    pub(super) fn set_neighbours(
        &self,
        new_neighbours: SmallVec<[DelaunayTriangleHandle<'arena>; 3]>,
    ) {
        let mut neighbours = &mut self.get_mut().unwrap().neighbours;
        *neighbours = [-1; 3];

        neighbours
            .iter_mut()
            .zip(new_neighbours.into_iter())
            .for_each(|(neighbour, new_neighbour)| *neighbour = new_neighbour.index().into());
    }

    pub(super) fn is_neighbour(&self, neighbour: &DelaunayTriangleHandle<'arena>) -> bool {
        self.neighbours().contains(neighbour)
    }

    pub(super) fn replace_neighbour(
        &self,
        index: Index,
        new_neighbour: DelaunayTriangleHandle<'arena>,
    ) {
        let mut neighbours = self.neighbours();
        let position = neighbours.iter().position(|n| n.index() == index);

        if let Some(position) = position {
            neighbours[position] = new_neighbour;
            self.set_neighbours(neighbours);
        }
    }

    pub(super) fn surrounding(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> SmallVec<[DelaunayTriangleHandle<'arena>; 6]> {
        let mut neighbours: SmallVec<[_; 6]> = self.neighbours().into_iter().collect();

        let mut other_neighbours: SmallVec<[_; 3]> = other
            .neighbours()
            .into_iter()
            .filter(|other_neighbour| !neighbours.contains(&other_neighbour))
            .collect();

        neighbours.append(&mut other_neighbours);

        neighbours
    }

    pub(super) fn is_counterclockwise(&self) -> bool {
        let this = self.get().unwrap();

        this.is_counterclockwise(self.points)
    }

    pub(super) fn make_counterclockwise(&mut self) {
        let mut this = self.get_mut().unwrap();

        this.make_counterclockwise(self.points);
    }

    pub(super) fn shared_points_with(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> SmallVec<[PointHandle; 2]> {
        let mut shared_points = SmallVec::<[PointHandle; 2]>::new();

        for other_point in other.points() {
            for point in self.points() {
                if point == other_point {
                    shared_points.push(point);
                }
            }
        }

        shared_points
    }

    pub(super) fn opposite_points_with(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> SmallVec<[PointHandle; 2]> {
        let shared_points = self.shared_points_with(other);
        self.points()
            .into_iter()
            .chain(other.points().into_iter())
            .filter(|p| !shared_points.contains(&p))
            .collect()
    }

    pub(super) fn is_flippable_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let shared_points = self.shared_points_with(other);
        let opposite_points = self.opposite_points_with(other);

        let is_shared_edge_connected_to_bounds = (shared_points.len() == 2
            && opposite_points.len() == 2)
            && (shared_points[0].is_bounding() || shared_points[1].is_bounding())
            && !(opposite_points[0].is_bounding() || opposite_points[1].is_bounding());

        let are_by_the_same_side_after_flip = (shared_points.len() == 2
            && opposite_points.len() == 2)
            && shared_points[0]
                .cross_product(&opposite_points[0], &opposite_points[1])
                .signum()
                != shared_points[1]
                    .cross_product(&opposite_points[0], &opposite_points[1])
                    .signum();

        is_shared_edge_connected_to_bounds && are_by_the_same_side_after_flip
    }

    pub(super) fn flip_with(&mut self, other: &mut DelaunayTriangleHandle<'arena>) -> bool {
        if self.is_flippable_with(other) {
            let mut neighbours = self.surrounding(other);

            {
                let shared_points = self.shared_points_with(other);
                let opposite_points = self.opposite_points_with(other);

                self.set_points([
                    shared_points[0].clone(),
                    opposite_points[0].clone(),
                    opposite_points[1].clone(),
                ]);
                other.set_points([
                    shared_points[1].clone(),
                    opposite_points[0].clone(),
                    opposite_points[1].clone(),
                ]);
            }

            self.make_counterclockwise();
            other.make_counterclockwise();

            let triangles = [self, other];

            for i in 0..triangles.len() {
                let triangle = &*triangles[i];
                let other_triangle = &*triangles[(i + 1) % 2];

                let mut new_neighbours: SmallVec<[_; 3]> = neighbours
                    .iter()
                    .cloned()
                    .filter(|neighbour| triangle.shared_points_with(&neighbour).len() == 2)
                    .map(|neighbour| {
                        if neighbour.is_neighbour(other_triangle) {
                            neighbour.replace_neighbour(other_triangle.index(), triangle.clone());
                        }

                        neighbour
                    })
                    .collect();

                triangle.set_neighbours(new_neighbours);
            }

            true
        } else {
            false
        }
    }
}

impl<'arena> Handle<'arena> for DelaunayTriangleHandle<'arena> {
    type Type = DelaunayTriangle;
    type Userdata = &'arena Arena<Point>;

    fn from_raw(raw: RawHandle<'arena, Self::Type>, userdata: Self::Userdata) -> Self {
        Self { raw, points: userdata }
    }

    fn as_raw(&self) -> &RawHandle<'arena, Self::Type> {
        &self.raw
    }
}

impl PartialEq for DelaunayTriangleHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl Eq for DelaunayTriangleHandle<'_> {}

pub struct Triangle;
