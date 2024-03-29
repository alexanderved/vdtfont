use super::{edge::Edge, Polygon};

use crate::point::{Point, PointHandle, PointId};

use std::{fmt, iter, hash, collections::HashSet};

use arena_system::{Arena, Handle, Index, RawHandle};
use smallvec::SmallVec;

pub type TriangleId = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum Visibility {
    Unknown = -1,
    Invisible = 0,
    Visible = 1,
}

/// A triangle which is used in [`Delaunay`].
/// 
/// [`Delaunay`]: crate::delaunay::Delaunay
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct DelaunayTriangle {
    pub vertices: [PointId; 3],
    pub neighbours: [TriangleId; 3],
    pub neighbours_number: i32,
    pub visibility: Visibility,
}

impl DelaunayTriangle {
    /// Creates a new [`DelaunayTriangle`] with the given `vertices`.
    pub fn new(vertices: [PointId; 3]) -> Self {
        Self {
            vertices,
            neighbours: [-1; 3],
            neighbours_number: 0,
            visibility: Visibility::Unknown,
        }
    }

    /// Returns the visibiity of the triangle.
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Sets the visibility of the triangle.
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    /// Checks if the triangle is counterclockwise.
    pub fn is_counterclockwise(&self, points: &Arena<Point>) -> bool {
        points.handle::<PointHandle>(self.vertices[1].into(), None).skew_product(
            &points.handle(self.vertices[0].into(), None),
            &points.handle(self.vertices[2].into(), None),
        ) < 0.0
    }

    /// Makes the triangle counterclockwise.
    pub fn make_counterclockwise(&mut self, points: &Arena<Point>) {
        if !self.is_counterclockwise(points) {
            let (vertices0, vertices1) = self.vertices[1..].split_at_mut(1);

            std::mem::swap(&mut vertices0[0], &mut vertices1[0]);
        }
    }

    /// Returns the radius of the circumcircle around the triangle.
    #[allow(non_snake_case)]
    pub fn circumcircle_radius(&self, points: &Arena<Point>) -> f32 {
        let A = points.handle::<PointHandle>(self.vertices[0].into(), None);
        let B = points.handle::<PointHandle>(self.vertices[1].into(), None);
        let C = points.handle::<PointHandle>(self.vertices[2].into(), None);

        let mut a = [0.0; 4];
        let mut b = [0.0; 2];
        a[0] = (A.x() - B.x()) * 2.0;
        a[1] = (A.y() - B.y()) * 2.0;
        a[2] = (B.x() - C.x()) * 2.0;
        a[3] = (B.y() - C.y()) * 2.0;
        b[0] = A.x() * A.x() + A.y() * A.y() - B.x() * B.x() - B.y() * B.y();
        b[1] = B.x() * B.x() + B.y() * B.y() - C.x() * C.x() - C.y() * C.y();

        let det = a[0] * a[3] - a[1] * a[2];
        if libm::fabsf(det) <= f32::EPSILON {
            return -1.0;
        }

        let mut center = [0.0; 2];
        center[0] = (b[0] * a[3] - a[1] * b[1]) / det;
        center[1] = (a[0] * b[1] - b[0] * a[2]) / det;

        let dx = A.x() - center[0];
        let dy = A.y() - center[1];

        libm::sqrtf(dx * dx + dy * dy)
    }
}

impl std::default::Default for DelaunayTriangle {
    fn default() -> Self {
        Self::new([-1; 3])
    }
}

unsafe impl ocl::traits::OclPrm for DelaunayTriangle {}

/// A handle of the [`DelaunayTriangle`] which is used by [`Arena`].
#[derive(Clone, Copy)]
pub struct DelaunayTriangleHandle<'arena> {
    raw: RawHandle<'arena, DelaunayTriangle>,
    points: &'arena Arena<Point>,
}

impl<'arena> DelaunayTriangleHandle<'arena> {
    /// Returns [`PointHandle`]s for the vertices of the triangle.
    pub fn points(&self) -> [PointHandle<'arena>; 3] {
        let vertices = self.get().unwrap().vertices;

        [
            self.points.handle(vertices[0].into(), Some(self.arena())),
            self.points.handle(vertices[1].into(), Some(self.arena())),
            self.points.handle(vertices[2].into(), Some(self.arena())),
        ]
    }

    /// Sets the given `points` as the vertices of the triangle.
    pub fn set_points(&self, points: [PointHandle; 3]) {
        let mut this = self.get_mut().unwrap();

        this.vertices =
            [points[0].index().into(), points[1].index().into(), points[2].index().into()]
    }

    /// Returns [`DelaunayTriangleHandle`]s for the neighbours of the triangle.
    pub fn neighbours(&self) -> SmallVec<[DelaunayTriangleHandle<'arena>; 3]> {
        let neighbour_ids = self.get().unwrap().neighbours;
        neighbour_ids
            .into_iter()
            .filter(|neighbour_id| *neighbour_id != -1)
            .map(|neighbour_id| self.arena().handle(neighbour_id.into(), self.points))
            .collect()
    }

    /// Sets the `new_neighbours` as the neighbours of the triangle.
    pub fn set_neighbours(&self, new_neighbours: SmallVec<[DelaunayTriangleHandle<'arena>; 3]>) {
        let neighbours = &mut self.get_mut().unwrap().neighbours;
        *neighbours = [-1; 3];

        neighbours
            .iter_mut()
            .zip(new_neighbours.into_iter())
            .for_each(|(neighbour, new_neighbour)| *neighbour = new_neighbour.index().into());
    }

    /// Checks if the triangle is visible.
    pub fn visibility(&self) -> Visibility {
        self.get_mut().unwrap().visibility
    }

    /// Sets the visibility of the triangle.
    pub fn set_visibiity(&self, visibility: Visibility) {
        self.get_mut().unwrap().visibility = visibility;
    }

    /// Returns the edges of the triangle.
    pub fn edges(&self) -> SmallVec<[Edge<'arena>; 3]> {
        let vertices = self.points();
        (0..3)
            .map(|i| i as usize)
            .map(|i| [vertices[i], vertices[(i + 1) % 3]])
            .map(|e| e.into())
            .collect::<SmallVec<[Edge<'arena>; 3]>>()
    }

    /// Returns the edges of the triangle except the `exception` edge.
    pub fn edges_except(&self, exception: Edge<'arena>) -> SmallVec<[Edge<'arena>; 2]> {
        self.edges().into_iter().filter(|edge| edge != &exception).collect()
    }

    /// Checks if the triangle is the neighbour of the other one.
    pub fn is_neighbour(&self, neighbour: &DelaunayTriangleHandle<'arena>) -> bool {
        self.neighbours().contains(neighbour)
    }

    /// Checks if the triangle is connected to the other one.
    pub fn is_connected(&self, other: &DelaunayTriangleHandle<'arena>) -> bool {
        self.shared_points_with(other).len() == 2
    }

    /// Replaces the neighbour with the index `index` with `new_neighbour` if it exists
    /// in the neighbour list of the triangle.
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

    /// Adds `new_neighbour` to the neighbour list of the triangle if it has free space.
    pub fn try_add_neighbour(&self, new_neighbour: DelaunayTriangleHandle<'arena>) -> bool {
        let neighbour_ids = &mut self.get_mut().unwrap().neighbours;
        if neighbour_ids.contains(&new_neighbour.index().into()) {
            return false;
        }

        let position = neighbour_ids.iter().position(|n| *n == -1);

        if let Some(position) = position {
            neighbour_ids[position] = new_neighbour.index().into();

            true
        } else {
            false
        }
    }

    /// Removes the neighbour with the index `index` from the neighbour list
    /// of the triangle if it exists there.
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

    /// Checks if the triangle is counterclockwise.
    pub fn is_counterclockwise(&self) -> bool {
        let this = self.get().unwrap();

        this.is_counterclockwise(self.points)
    }

    /// Makes the triangle counterclockwise.
    pub fn make_counterclockwise(&mut self) {
        let mut this = self.get_mut().unwrap();

        this.make_counterclockwise(self.points);
    }

    /// Returns points which the triangle shares with the other one.
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

    /// Returns an edge which the triangle shares with the other one.
    pub fn shared_edge_with(&self, other: &DelaunayTriangleHandle<'arena>) -> Edge {
        self.shared_points_with(other).into()
    }

    /// Returns points which the triangle doesn't share with the other one.
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

    /// Returns the edge of the triangle which is opposite to the given `vertex`.
    pub fn opposite_edge_to(&self, vertex: PointHandle) -> Edge<'arena> {
        self.points()
            .into_iter()
            .filter(|point| *point != vertex)
            .collect::<SmallVec<[PointHandle; 2]>>()
            .into()
    }

    /// Returns the neighbour of the triangle on the given `edge`.
    pub fn neighbour_on_edge(&self, edge: Edge) -> DelaunayTriangleHandle<'arena> {
        self.neighbours()
            .into_iter()
            .find(|n| edge == self.shared_points_with(n).into())
            .expect("No neighbour which shares the specified edge")
    }

    /// Checks if the triangle is fully in the circle of the other triangle.
    pub fn is_in_circle_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let s = self.shared_points_with(other);
        let o = self.opposite_points_with(other);

        let origin = s[0];
        let mut polygon = Polygon::new([o[0], s[1], o[1]].to_vec());

        polygon.sort_by_angle(origin);

        let p = polygon
            .points()
            .iter()
            .map(|p| Point::new(p.x() - origin.x(), p.y() - origin.y()))
            .rev()
            .collect::<Vec<Point>>();

        let abdet = p[0].x() * p[1].y() - p[1].x() * p[0].y();
        let bcdet = p[1].x() * p[2].y() - p[2].x() * p[1].y();
        let cadet = p[2].x() * p[0].y() - p[0].x() * p[2].y();

        let alift = p[0].x() * p[0].x() + p[0].y() * p[0].y();
        let blift = p[1].x() * p[1].x() + p[1].y() * p[1].y();
        let clift = p[2].x() * p[2].x() + p[2].y() * p[2].y();

        alift * bcdet + blift * cadet + clift * abdet > 0.0
    }

    /// Checks if the triangle can be flipped with the other one.
    pub fn is_flippable_with(&self, other: &DelaunayTriangleHandle) -> bool {
        let shared_points = self.shared_points_with(other);
        let opposite_points = self.opposite_points_with(other);

        if !(shared_points.len() == 2 && opposite_points.len() == 2) {
            return false;
        }

        // Can't be flipped if the shared edge will be connected to the bounds after flip.
        let is_opposite_point_on_bounds =
            opposite_points[0].is_bounding() || opposite_points[1].is_bounding();

        // Can't be flipped if the triangles will intersect after flip.
        let sp0 = shared_points[0].skew_product(&opposite_points[0], &opposite_points[1]);
        let sp1 = shared_points[1].skew_product(&opposite_points[0], &opposite_points[1]);
        let by_the_same_side_after_flip = sp0.signum() == sp1.signum();

        // Can't be flipped if the shared edge is in the contour.
        let has_contour_edge = Edge::from(shared_points).is_contour();

        // Can't be flipped if the triangles satisfies Delaunay condition.
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

    /// Flips the triangle with the other one.
    pub fn flip_with(
        &mut self,
        other: &mut DelaunayTriangleHandle<'arena>,
        mut deep: usize,
    ) -> bool {
        let is_flippable = self.is_flippable_with(other) && deep != 0;
        if is_flippable {
            self.flip_edge(other);

            let neighbourhood = self.neighbourhood(*other);
            self.update_neighbours(neighbourhood.clone());
            other.update_neighbours(neighbourhood);

            deep -= 1;

            self.flip_with_neighbours_except(Some(*other), deep);
            other.flip_with_neighbours_except(Some(*self), deep);
        }

        is_flippable
    }

    // Flips the shared edge of the triangles.
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

    // Gets all neighbours of the two triangles.
    fn neighbourhood(
        &self,
        other: DelaunayTriangleHandle<'arena>,
    ) -> Vec<DelaunayTriangleHandle<'arena>> {
        let neighbourhood = self.neighbours()
            .into_iter()
            .chain(other.neighbours())
            .collect::<HashSet<DelaunayTriangleHandle>>()
            .into_iter()
            .collect::<Vec<DelaunayTriangleHandle>>();

        neighbourhood
    }

    // Update the neighbours of the triangle with `supposed_neighbours`.
    fn update_neighbours(&self, mut supposed_neighbours: Vec<DelaunayTriangleHandle<'arena>>) {
        let mut neighbours = self.neighbours().to_vec();
        neighbours.append(&mut supposed_neighbours);

        // Remove all neighbours which are not connected to each other.
        for neighbour in iter::once(self).chain(neighbours.iter()) {
            neighbour
                .neighbours()
                .into_iter()
                .filter(|n| !neighbour.is_connected(n))
                .for_each(|n| {
                    neighbour.try_remove_neighbour(n.index());
                });
        }

        // Make connected triangles neighbours.
        neighbours
            .into_iter()
            .filter(|neighbour| self.is_connected(neighbour))
            .for_each(|neighbour| {
                neighbour.try_add_neighbour(*self);
                self.try_add_neighbour(neighbour);
            });
    }

    /// Flips the triangle with its neighbours except the `exception` triangle.
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

impl hash::Hash for DelaunayTriangleHandle<'_> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let index: i64 = self.index().into();
        index.hash(state);
    }
}