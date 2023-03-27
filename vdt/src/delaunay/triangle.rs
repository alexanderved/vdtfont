#![allow(unused)]

use crate::delaunay::{bounds::Bounds, DelaunayPoint, PointId};

use arena_system::{Arena, Handle, RawHandle};

use super::DelaunayPointHandle;

pub(super) type TriangleId = i64;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(super) struct DelaunayTriangle {
    pub(super) vertices: [PointId; 3],
}

impl DelaunayTriangle {
    pub(super) fn new(vertices: [PointId; 3]) -> Self {
        Self { vertices }
    }

    pub(super) fn is_counterclockwise(&self, points: &Arena<DelaunayPoint>) -> bool {
        points.handle::<DelaunayPointHandle>(self.vertices[1].into(), ()).cross_product(
            &points.handle(self.vertices[0].into(), ()),
            &points.handle(self.vertices[2].into(), ()),
        ) < 0.0
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

    pub(super) fn shared_points_with(
        &self,
        other_id: TriangleId,
        triangles: &Vec<DelaunayTriangle>,
    ) -> [PointId; 2] {
        let mut shared_points = [-1; 2];
        let mut last_shared_point = 0;

        for point_id in self.vertices {
            for other_point_id in triangles[other_id as usize].vertices {
                if point_id == other_point_id {
                    shared_points[last_shared_point] = point_id;
                    last_shared_point += 1;
                }
            }
        }

        shared_points
    }

    pub(super) fn opposite_points_with(
        &self,
        other_id: TriangleId,
        triangles: &Vec<DelaunayTriangle>,
    ) -> [PointId; 2] {
        let mut opposite_points = [-1; 2];
        let shared_points = self.shared_points_with(other_id, triangles);

        for (i, triangle) in [self, &triangles[other_id as usize]].into_iter().enumerate() {
            for point_id in triangle.vertices {
                if !shared_points.contains(&point_id) {
                    opposite_points[i] = point_id;
                    break;
                }
            }
        }

        opposite_points
    }

    pub(super) fn is_flippable_with(
        &self,
        other_id: TriangleId,
        triangles: &Vec<DelaunayTriangle>,
        bounds: &Bounds,
    ) -> bool {
        let shared_points = self.shared_points_with(other_id, triangles);
        let is_shared_edge_connected_to_bounds =
            bounds.contains(shared_points[0]) || bounds.contains(shared_points[1]);

        is_shared_edge_connected_to_bounds
    }

    pub(super) fn flip_with(
        &mut self,
        other_id: TriangleId,
        triangles: &mut Vec<DelaunayTriangle>,
        points: &Arena<DelaunayPoint>,
        bounds: &Bounds,
    ) {
        if self.is_flippable_with(other_id, triangles, bounds) {
            let shared_points = self.shared_points_with(other_id, triangles);
            let opposite_points = self.opposite_points_with(other_id, triangles);

            self.vertices = [shared_points[0], opposite_points[0], opposite_points[1]];
            triangles[other_id as usize].vertices =
                [shared_points[1], opposite_points[0], opposite_points[1]];

            self.make_counterclockwise(points);
            triangles[other_id as usize].make_counterclockwise(points);
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
