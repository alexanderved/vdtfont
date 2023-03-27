#![allow(unused)]

use crate::delaunay::{bounds::Bounds, DelaunayPoint, PointId};

use arena_system::{Arena, Handle, RawHandle};
use smallvec::SmallVec;

use super::DelaunayPointHandle;

pub(super) type TriangleId = i64;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(super) struct DelaunayTriangle {
    pub(super) vertices: [PointId; 3],
    pub(super) neighbours: [TriangleId; 3],
}

impl DelaunayTriangle {
    pub(super) fn new(vertices: [PointId; 3]) -> Self {
        Self { vertices, neighbours: [-1; 3] }
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

pub(super) struct DelaunayTriangleHandle<'arena> {
    raw: RawHandle<'arena, DelaunayTriangle>,
    points: &'arena Arena<DelaunayPoint>,
}

impl<'arena> DelaunayTriangleHandle<'arena> {
    pub(super) fn point_ids(&self) -> [PointId; 3] {
        let this = self.get().unwrap();

        this.vertices
    }

    pub(super) fn points(&self) -> [DelaunayPointHandle<'arena>; 3] {
        let vertices = self.get().unwrap().vertices;
        let points = &*self.points;

        [
            points.handle(vertices[0].into(), ()),
            points.handle(vertices[1].into(), ()),
            points.handle(vertices[2].into(), ()),
        ]
    }

    pub(super) fn set_points(&self, points: [DelaunayPointHandle; 3]) {
        let mut this = self.get_mut().unwrap();

        this.vertices =
            [points[0].index().into(), points[1].index().into(), points[2].index().into()]
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
        other: &DelaunayTriangleHandle,
    ) -> [DelaunayPointHandle; 2] {
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

        shared_points.into_inner().unwrap()
    }

    pub(super) fn opposite_points_with(
        &self,
        other: &DelaunayTriangleHandle<'arena>,
    ) -> [DelaunayPointHandle; 2] {
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

        opposite_points.into_inner().unwrap()
    }

    pub(super) fn is_flippable_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let shared_points = self.shared_points_with(other);
        let is_shared_edge_connected_to_bounds =
            shared_points[0].is_bounding() || shared_points[1].is_bounding();

        is_shared_edge_connected_to_bounds
    }

    pub(super) fn flip_with(&mut self, other: &mut DelaunayTriangleHandle) {
        let is_flippable = self.is_flippable_with(other);
        if is_flippable {
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

            self.make_counterclockwise();
            other.make_counterclockwise();
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

pub struct Triangle;
