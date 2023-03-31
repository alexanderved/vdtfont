use crate::point::{Point, PointHandle, PointId};

use arena_system::{Arena, Handle, Index, RawHandle};
use smallvec::SmallVec;

pub type TriangleId = i64;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DelaunayTriangle {
    pub vertices: [PointId; 3],
    pub neighbours: [TriangleId; 3],
    pub neighbours_number: i32,
}

impl DelaunayTriangle {
    pub fn new(vertices: [PointId; 3]) -> Self {
        Self { vertices, neighbours: [-1; 3], neighbours_number: 0 }
    }

    pub fn is_counterclockwise(&self, points: &Arena<Point>) -> bool {
        points.handle::<PointHandle>(self.vertices[1].into(), ()).skew_product(
            &points.handle(self.vertices[0].into(), ()),
            &points.handle(self.vertices[2].into(), ()),
        ) < 0.0
    }

    pub fn make_counterclockwise(&mut self, points: &Arena<Point>) {
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
pub struct DelaunayTriangleHandle<'arena> {
    raw: RawHandle<'arena, DelaunayTriangle>,
    points: &'arena Arena<Point>,
}

impl<'arena> DelaunayTriangleHandle<'arena> {
    pub fn points(&self) -> [PointHandle<'arena>; 3] {
        let vertices = self.get().unwrap().vertices;

        [
            self.points.handle(vertices[0].into(), ()),
            self.points.handle(vertices[1].into(), ()),
            self.points.handle(vertices[2].into(), ()),
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

    pub fn is_neighbour(&self, neighbour: &DelaunayTriangleHandle<'arena>) -> bool {
        self.neighbours().contains(neighbour)
    }

    pub fn replace_neighbour(&self, index: Index, new_neighbour: DelaunayTriangleHandle<'arena>) {
        let mut neighbours = self.neighbours();
        let position = neighbours.iter().position(|n| n.index() == index);

        if let Some(position) = position {
            neighbours[position] = new_neighbour;
            self.set_neighbours(neighbours);
        }
    }

    pub fn remove_neighbour(&self, index: Index) {
        let mut neighbours = self.neighbours();
        let position = neighbours.iter().position(|n| n.index() == index);

        if let Some(position) = position {
            neighbours[position] = self
                .arena()
                .handle::<DelaunayTriangleHandle>(<i64 as Into<Index>>::into(-1i64), self.points);
            self.set_neighbours(neighbours);
        }
    }

    pub fn surrounding(
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

        self.points().into_iter().filter(|p| other_points.contains(p)).collect()
    }

    pub fn opposite_points_with(
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

    pub fn is_in_circle_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let s = self.shared_points_with(other);
        let o = self.opposite_points_with(other);

        let s = [
            *s[0].get().unwrap(),
            *s[1].get().unwrap(),
        ];
        let o = [
            *o[0].get().unwrap(),
            *o[1].get().unwrap(),
        ];
        let p = [o[0], s[1], o[1], s[0]];

        let center = p[3];
        let mut points = p[0..3]
            .into_iter()
            .map(|p| Point::new(p.x() - center.x(), p.y() - center.y(), false, -1))
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
            return false;
        }

        let is_shared_edge_connected_to_bounds =
            shared_points[0].is_bounding() || shared_points[1].is_bounding();

        let by_the_same_side_after_flip = shared_points[0]
            .skew_product(&opposite_points[0], &opposite_points[1])
            .signum()
            == shared_points[1]
                .skew_product(&opposite_points[0], &opposite_points[1])
                .signum();

        let is_contour = shared_points[0].previous_in_outline() == shared_points[1]
            || shared_points[1].previous_in_outline() == shared_points[0];

        let satisfies_delaunay_condition = !self.is_in_circle_with(other);

        if by_the_same_side_after_flip {
            return false;
        }

        if is_contour {
            return false;
        }

        if is_shared_edge_connected_to_bounds { 
            return true;
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
            let neighbours = self.surrounding(other);

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

            let triangles = [&*self, &*other];

            for i in 0..triangles.len() {
                let triangle = triangles[i];
                let other_triangle = triangles[(i + 1) % 2];

                let new_neighbours: SmallVec<[_; 3]> = neighbours
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

            deep -= 1;

            self.neighbours().into_iter().filter(|n| *n != *other).for_each(|mut n| {
                self.flip_with(&mut n, deep);
            });

            other.neighbours().into_iter().filter(|n| *n != *self).for_each(|mut n| {
                other.flip_with(&mut n, deep);
            });
        }

        is_flippable
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
