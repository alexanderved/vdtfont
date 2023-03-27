#![allow(unused)]

use crate::delaunay::{bounds::Bounds, DelaunayPoint, PointId};

use arena_system::{Arena, Handle, RawHandle};

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
        Self {
            vertices,
            neighbours: [-1; 3],
        }
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

    pub(super) fn points(&self) -> [DelaunayPointHandle; 3] {
        let this = self.get().unwrap();

        [
            self.points.handle(this.vertices[0].into(), ()),
            self.points.handle(this.vertices[1].into(), ()),
            self.points.handle(this.vertices[2].into(), ()),
        ]
    }

    pub(super) fn set_points(&mut self, points: [DelaunayPointHandle; 3]) {
        let mut this = self.get_mut().unwrap();

        this.vertices = [
            points[0].index().into(),
            points[1].index().into(),
            points[2].index().into(),
        ]
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
    ) -> [PointId; 2] {
        let mut shared_points = [-1; 2];
        let mut last_shared_point = 0;

        let point_ids = self.point_ids();
        let other_point_ids = other.point_ids();

        for point_id in point_ids {
            for other_point_id in other_point_ids {
                if point_id == other_point_id {
                    shared_points[last_shared_point] = point_id;
                    last_shared_point += 1;
                }
            }
        }

        shared_points
    }

    pub(super) fn opposite_points_with(&self, other: &DelaunayTriangleHandle) -> [PointId; 2] {
        let mut opposite_points = [-1; 2];
        let shared_points = self.shared_points_with(other);

        let point_ids = self.point_ids();
        let other_point_ids = other.point_ids();

        for (i, point_ids) in [point_ids, other_point_ids].into_iter().enumerate() {
            for point_id in point_ids {
                if !shared_points.contains(&point_id) {
                    opposite_points[i] = point_id;
                    break;
                }
            }
        }

        opposite_points
    }

    pub(super) fn is_flippable_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let shared_points = self.shared_points_with(other);
        let is_shared_edge_connected_to_bounds = self
            .points
            .handle::<DelaunayPointHandle>(shared_points[0].into(), ())
            .is_bounding()
            || self
                .points
                .handle::<DelaunayPointHandle>(shared_points[1].into(), ())
                .is_bounding();

        is_shared_edge_connected_to_bounds
    }

    pub(super) fn flip_with(&mut self, other: &mut DelaunayTriangleHandle) {
        if self.is_flippable_with(other) {
            let shared_points = self.shared_points_with(other);
            let opposite_points = self.opposite_points_with(other);

            {
                let mut this = self.get_mut().unwrap();
                let mut other = other.get_mut().unwrap();

                this.vertices = [shared_points[0], opposite_points[0], opposite_points[1]];
                other.vertices = [shared_points[1], opposite_points[0], opposite_points[1]];
            }

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

    fn as_mut_raw(&mut self) -> &mut RawHandle<'arena, Self::Type> {
        &mut self.raw
    }
}

pub struct Triangle;
