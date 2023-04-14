mod bounds;
mod edge;
mod factory;
mod polygon;
mod triangle;
mod triangle_fan;

pub use bounds::Bounds;
pub use edge::Edge;
pub use factory::DelaunayFactory;
pub use polygon::Polygon;
pub use triangle::{DelaunayTriangle, DelaunayTriangleHandle, TriangleId, Visibility};

use crate::point::*;

use arena_system::{Arena, Handle, Index};

/// A Delaunay triangulation.
pub struct Delaunay {
    dim: usize,

    points: Arena<Point>,
    triangles: Arena<DelaunayTriangle>,

    bounds: Bounds,
}

impl Delaunay {
    /// Returns a dimension of the triangulation.
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Returns a reference to the arena of points in the triangulation.
    pub fn points(&self) -> &Arena<Point> {
        &self.points
    }

    /// Returns a reference to the arena of triangles which the triangulation consits of.
    pub fn triangles(&self) -> &Arena<DelaunayTriangle> {
        &self.triangles
    }

    /// Returns bounds of the triangulation.
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    /// Converts triangulation into raw parts: a dimension, bounds, points and triangles.
    pub fn into_raw_parts(self) -> (usize, Arena<Point>, Arena<DelaunayTriangle>, Bounds) {
        (self.dim, self.points, self.triangles, self.bounds)
    }

    /// Inserts `triangle` into the triangulation and connects it to `supposed_neighbours`
    /// if it shares two points with them.
    pub fn insert_triangle(
        &mut self,
        triangle: DelaunayTriangle,
        supposed_neighbours: &[Index],
    ) -> Index {
        let triangle_index = self.triangles.add(triangle);
        let triangle_handle = self
            .triangles
            .handle::<DelaunayTriangleHandle>(triangle_index, self.points());

        // Connect `triangle` and `supposed_neighbours` to each other.
        supposed_neighbours
            .iter()
            .copied()
            .map(|neighbour_index| {
                self.triangles
                    .handle::<DelaunayTriangleHandle>(neighbour_index, self.points())
            })
            .filter(|neigbour| triangle_handle.is_connected(neigbour))
            .for_each(|neighbour| {
                neighbour.try_add_neighbour(triangle_handle);
                triangle_handle.try_add_neighbour(neighbour);
            });

        // Update triangle fans.
        triangle_handle.points().into_iter().for_each(|p| {
            p.add_triangle_to_fan(triangle_handle);
        });

        triangle_index
    }

    /// Removes a triangle with the index `triangle_index` from the triangulation.
    pub fn remove_triangle(&mut self, triangle_index: Index) {
        let triangle = self
            .triangles()
            .handle::<DelaunayTriangleHandle>(triangle_index, self.points());

        triangle.neighbours().into_iter().for_each(|n| {
            n.try_remove_neighbour(triangle.index());
        });

        triangle.points().into_iter().for_each(|p| {
            p.remove_triangle_from_fan(triangle_index);
        });

        self.triangles.remove(triangle_index).unwrap();
    }

    /// Inserts `edge` into the triangulation.
    pub fn insert_edge(&mut self, edge: [PointId; 2]) {
        let edge: Edge = [
            self.points().handle(edge[0].into(), Some(self.triangles())),
            self.points().handle(edge[1].into(), Some(self.triangles())),
        ]
        .into();

        // Find edges and triangles which are intersected by the given `edge`.
        let (edge_track, triangle_track) = edge.find_triangle_track();

        // Calculate contours around the `edge`.
        let contour0 = self.calculate_contour(edge, edge_track[0].points()[0], &edge_track);
        let contour1 = self.calculate_contour(edge, edge_track[0].points()[1], &edge_track);

        // Triangulate the contours.
        let triangulation0 = self.triangulate_hole(contour0);
        let triangulation1 = self.triangulate_hole(contour1);

        // Find triangles around the `triangle_track`.
        let mut neighbours = triangle_track
            .iter()
            .flat_map(|triangle| triangle.neighbours())
            .filter(|neighbour| !triangle_track.contains(neighbour))
            .map(|neighbour| neighbour.index())
            .collect::<Vec<Index>>();

        // Remove the triangles which are intersected by the given `edge`.
        let triangle_indices_to_remove =
            triangle_track.into_iter().map(|t| t.index()).collect::<Vec<_>>();
        triangle_indices_to_remove
            .into_iter()
            .for_each(|t| self.remove_triangle(t));

        // Insert the triangulations of the contours into the triangulation.
        triangulation0.into_iter().chain(triangulation1).for_each(|t| {
            let triangle_index = self.insert_triangle(t, &neighbours);
            neighbours.push(triangle_index);
        });
    }

    // Calculates a contour which starts from the first point of `base_line`, passes
    // through the `control_point` and ends at the second point of `base_line`.
    //
    // Other points of the contour are obtained from `edge_track`.
    //
    // TODO: Fix algorithm of searching the next point in the contour because
    // the current one may cause the creation of wrong contours.
    fn calculate_contour<'arena>(
        &self,
        base_line: Edge<'arena>,
        control_point: PointHandle<'arena>,
        edge_track: &[Edge<'arena>],
    ) -> Vec<PointHandle<'arena>> {
        let mut contour = vec![base_line.points()[0], control_point];
        for e in edge_track[1..].iter() {
            let last = contour.last().unwrap();
            let d0 = last.distance(&e.points()[0]);
            let d1 = last.distance(&e.points()[1]);

            // Move to the next point because one of the current points is
            // the same as the previous one
            if d0 == 0.0 || d1 == 0.0 {
                continue;
            }

            if d0 < d1 {
                contour.push(e.points()[0]);
            } else {
                contour.push(e.points()[1]);
            }
        }
        contour.push(base_line.points()[1]);

        contour
    }

    // Triangulate the given `contour`.
    //
    // TODO: Fix this algorithm because it can cause the creation of triangles
    // which intersects each other.
    fn triangulate_hole(&self, mut contour: Vec<PointHandle>) -> Vec<DelaunayTriangle> {
        let mut middle_vertex = 0;
        let mut smallest_triangle = None;
        let mut smallest_circle = f32::MAX;

        // Find the triangle with the smallest circumcircle.
        for (i, points) in contour.windows(3).enumerate() {
            let t = DelaunayTriangle::new([
                points[0].index().into(),
                points[1].index().into(),
                points[2].index().into(),
            ]);

            // t.is_counterclockwise(self.points()) {
            let r = t.circumcircle_radius(self.points());

            if r < smallest_circle {
                smallest_circle = r;
                smallest_triangle = Some(t);
                middle_vertex = i + 1;
            }
            // }
        }

        let mut triangulation = vec![];
        if let Some(smallest_triangle) = smallest_triangle {
            // Update contour after obtaining the triangle from it.
            contour.remove(middle_vertex);
            // Add the obtained triangle to the triangulation.
            triangulation.push(smallest_triangle);

            // Triangulate the new contour if it has 3 points at least.
            if contour.len() >= 3 {
                triangulation.append(&mut self.triangulate_hole(contour));
            }
        }

        triangulation
    }

    // Creates a new [`Delaunay`].
    //
    // The validity of `dim`, `points`, `triangles`, `bounds` is ensured by [`DelaunayFactory`].
    fn new(
        dim: usize,
        points: Arena<Point>,
        triangles: Arena<DelaunayTriangle>,
        bounds: Bounds,
    ) -> Self {
        Self { dim, points, triangles, bounds }
    }
}
