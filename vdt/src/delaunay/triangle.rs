#![allow(unused)]

use crate::delaunay::{DelaunayPoint, PointId};

use arena_system::{Arena, Handle, RawHandle};
use smallvec::SmallVec;

use super::DelaunayPointHandle;

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

    pub(super) fn is_counterclockwise(&self, points: &Arena<DelaunayPoint>) -> bool {
        points
            .handle::<DelaunayPointHandle>(self.vertices[1].into(), ())
            .cross_product(
                &points.handle(self.vertices[0].into(), ()),
                &points.handle(self.vertices[2].into(), ()),
            )
            < 0.0
    }

    pub(super) fn make_counterclockwise(&mut self, points: &Arena<DelaunayPoint>) {
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
    points: &'arena Arena<DelaunayPoint>,
}

impl<'arena> DelaunayTriangleHandle<'arena> {
    pub(super) fn points(&self) -> [DelaunayPointHandle<'arena>; 3] {
        let vertices = self.get().unwrap().vertices;

        [
            self.points.handle(vertices[0].into(), ()),
            self.points.handle(vertices[1].into(), ()),
            self.points.handle(vertices[2].into(), ()),
        ]
    }

    pub(super) fn set_points(&self, points: [DelaunayPointHandle; 3]) {
        let mut this = self.get_mut().unwrap();

        this.vertices =
            [points[0].index().into(), points[1].index().into(), points[2].index().into()]
    }

    pub(super) fn neighbours(&self) -> SmallVec<[DelaunayTriangleHandle<'arena>; 3]> {
        let neighbour_ids = self.get().unwrap().neighbours;
        let mut neighbour_handles = SmallVec::new();

        for neighbour_id in neighbour_ids {
            if neighbour_id != -1 {
                neighbour_handles.push(self.arena().handle(neighbour_id.into(), self.points))
            }
        }

        neighbour_handles
    }

    pub(super) fn set_neghbours(&self, neighbours: SmallVec<[DelaunayTriangleHandle<'arena>; 3]>) {
        let mut this = self.get_mut().unwrap();

        let mut new_neighbours = [-1; 3];
        for (i, neighbour) in neighbours.into_iter().enumerate() {
            new_neighbours[i] = neighbour.index().into();
        }

        this.neighbours = new_neighbours;
    }

    pub(super) fn surrounding(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> SmallVec<[DelaunayTriangleHandle<'arena>; 4]> {
        let mut neighbours: SmallVec<[_; 4]> = self
            .neighbours()
            .into_iter()
            .filter(|n| n.index() != other.index())
            .collect();

        for other_neighbour in other.neighbours() {
            if !neighbours.contains(&other_neighbour) && other_neighbour != *self {
                neighbours.push(other_neighbour);
            }
        }

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
    ) -> SmallVec<[DelaunayPointHandle; 2]> {
        let mut shared_points = SmallVec::<[DelaunayPointHandle; 2]>::new();

        let points = self.points();
        let other_points = other.points();

        for point in points {
            for other_point in other_points.iter() {
                if point == *other_point {
                    shared_points.push(point);
                    break;
                }
            }
        }

        shared_points
    }

    pub(super) fn opposite_points_with(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> SmallVec<[DelaunayPointHandle; 2]> {
        let mut opposite_points = SmallVec::<[DelaunayPointHandle; 2]>::new();
        let shared_points = self.shared_points_with(other);

        let points = self.points();
        let other_points = other.points();

        for points in [points, other_points].into_iter() {
            for point in points {
                if !shared_points.contains(&point) {
                    opposite_points.push(point);
                    break;
                }
            }
        }

        opposite_points
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

            neighbours.append(&mut SmallVec::from([self.clone(), other.clone()]));

            for triangle in [self, other] {
                let mut new_neighbours = SmallVec::<[DelaunayTriangleHandle; 3]>::new();
                for neighbour in neighbours.iter().cloned() {
                    if triangle.shared_points_with(&neighbour).len() == 2 {
                        new_neighbours.push(neighbour);
                    }
                }

                println!(
                    "{:?} new: {:?}",
                    triangle.index(),
                    new_neighbours.iter().map(|n| n.index().into()).collect::<Vec<i64>>()
                );

                triangle.set_neghbours(new_neighbours);
            }

            true
        } else {
            false
        }
    }
}

impl<'arena> Handle<'arena> for DelaunayTriangleHandle<'arena> {
    type Type = DelaunayTriangle;
    type Userdata = &'arena Arena<DelaunayPoint>;

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
