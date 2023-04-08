mod bounds;
mod edge;
mod factory;
mod polygon;
mod triangle;
mod triangle_fan;

pub(crate) use bounds::*;
pub use edge::*;
pub use factory::DelaunayFactory;
pub use polygon::Polygon;
pub use triangle::*;

use crate::point::*;

use arena_system::{Arena, Handle, Index};

pub struct Delaunay {
    dim: usize,

    points: Arena<Point>,
    triangles: Arena<DelaunayTriangle>,

    bounds: Bounds,
}

impl Delaunay {
    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn points(&self) -> &Arena<Point> {
        &self.points
    }

    pub fn triangles(&self) -> &Arena<DelaunayTriangle> {
        &self.triangles
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    pub fn insert_edge(&mut self, edge: [PointId; 2]) {
        let edge: Edge = [
            self.points().handle(edge[0].into(), Some(self.triangles())),
            self.points().handle(edge[1].into(), Some(self.triangles())),
        ]
        .into();

        let (edge_track, triangle_track) = edge.find_triangle_track();
        let mut neighbours = triangle_track
            .iter()
            .flat_map(|t| t.neighbours())
            .filter(|n| !triangle_track.contains(&n))
            .map(|n| n.index())
            .collect::<Vec<_>>();

        let mut contour0 = vec![edge.points()[0], edge_track[0].points()[0]];
        for e in edge_track[1..].iter() {
            let last = contour0.last().unwrap();
            let d0 = last.distance(&e.points()[0]);
            let d1 = last.distance(&e.points()[1]);

            if d0 == 0.0 || d1 == 0.0 {
                continue;
            }

            if d0 < d1 {
                contour0.push(e.points()[0]);
            } else {
                contour0.push(e.points()[1]);
            }
        }
        contour0.push(edge.points()[1]);
        //let contour0 = contour0.into_iter().rev().collect::<Vec<_>>();

        let mut contour1 = vec![edge.points()[0], edge_track[0].points()[1]];
        for e in edge_track[1..].iter() {
            let last = contour1.last().unwrap();
            let d0 = last.distance(&e.points()[0]);
            let d1 = last.distance(&e.points()[1]);

            if d0 == 0.0 || d1 == 0.0 {
                continue;
            }

            if d0 < d1 {
                contour1.push(e.points()[0]);
            } else {
                contour1.push(e.points()[1]);
            }
        }
        contour1.push(edge.points()[1]);
        let contour1 = contour1.into_iter().rev().collect::<Vec<_>>();

        println!("Contour0: {:?}", contour0);
        println!("Contour1: {:?}", contour1);

        let tri0 = self.triangulate_hole(contour0);
        let tri1 = self.triangulate_hole(contour1);

        let triangle_indices_to_remove = triangle_track
            .into_iter()
            .map(|t| t.index())
            .collect::<Vec<_>>();
        triangle_indices_to_remove
            .into_iter()
            .for_each(|t| self.remove_triangle(t));

        for t in tri0 {
            let triangle_index = self.insert_triangle(t, &neighbours);
            neighbours.push(triangle_index);
        }

        for t in tri1 {
            let triangle_index = self.insert_triangle(t, &neighbours);
            neighbours.push(triangle_index);
        }
    }

    pub fn triangulate_hole(&self, mut contour: Vec<PointHandle>) -> Vec<DelaunayTriangle> {
        let mut middle_vertex = 0;
        let mut smallest_triangle = None;
        let mut smallest_circle = f32::MAX;
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

        let mut res = vec![];
        if let Some(smallest_triangle) = smallest_triangle {
            contour.remove(middle_vertex);
            res.push(smallest_triangle);

            if contour.len() >= 3 {
                res.append(&mut self.triangulate_hole(contour));
            }
        }

        res
    }

    pub fn insert_triangle(
        &mut self,
        triangle: DelaunayTriangle,
        supposed_neighbours: &Vec<Index>
    ) -> Index {
        let triangle_index = self.triangles.add(triangle);
        let triangle_handle =
            self.triangles.handle::<DelaunayTriangleHandle>(triangle_index, self.points());

        supposed_neighbours
            .iter()
            .copied()
            .map(|neighbour_index| {
                self.triangles.handle::<DelaunayTriangleHandle>(neighbour_index, self.points())
            })
            .filter(|neigbour| triangle_handle.shared_points_with(&neigbour).len() == 2)
            .for_each(|neighbour| {
                neighbour.try_add_neighbour(triangle_handle);
                triangle_handle.try_add_neighbour(neighbour);
            });

        triangle_handle.points()
            .into_iter()
            .for_each(|p| {
                p.add_triangle_to_fan(triangle_handle);
            });    

        triangle_index
    }

    pub fn remove_triangle(&mut self, triangle_index: Index) {
        let triangle =
            self.triangles().handle::<DelaunayTriangleHandle>(triangle_index, self.points());

        triangle.neighbours()
            .into_iter()
            .for_each(|n| {
                n.try_remove_neighbour(triangle.index());
            });

        triangle.points()
            .into_iter()
            .for_each(|p| {
                p.remove_triangle_from_fan(triangle_index);
            });

        self.triangles.remove(triangle_index).unwrap();
    }

    pub fn image(&self) -> Vec<u8> {
        let mut bitmap = vec![0.0; self.dim * self.dim];

        self
            .triangles
            .handle_iter::<DelaunayTriangleHandle>(&self.points)
            .for_each(|t| {
                if let Ok(t) = t.get() {
                    if t.is_visible {
                        crate::draw_line(
                            &mut bitmap,
                            self.dim,
                            self.dim,
                            (*self
                                .points
                                .handle::<PointHandle>(t.vertices[0].into(), None)
                                .get()
                                .unwrap())
                            .clone(),
                            (*self
                                .points
                                .handle::<PointHandle>(t.vertices[1].into(), None)
                                .get()
                                .unwrap())
                            .clone(),
                        );
        
                        crate::draw_line(
                            &mut bitmap,
                            self.dim,
                            self.dim,
                            (*self
                                .points
                                .handle::<PointHandle>(t.vertices[1].into(), None)
                                .get()
                                .unwrap())
                            .clone(),
                            (*self
                                .points
                                .handle::<PointHandle>(t.vertices[2].into(), None)
                                .get()
                                .unwrap())
                            .clone(),
                        );
        
                        crate::draw_line(
                            &mut bitmap,
                            self.dim,
                            self.dim,
                            (*self
                                .points
                                .handle::<PointHandle>(t.vertices[0].into(), None)
                                .get()
                                .unwrap())
                            .clone(),
                            (*self
                                .points
                                .handle::<PointHandle>(t.vertices[2].into(), None)
                                .get()
                                .unwrap())
                            .clone(),
                        );
                    }
                }
            });

        bitmap.into_iter().flat_map(|a| [0, 0, 0, (255.0 * a) as u8]).collect()
    }

    fn new(
        dim: usize,
        points: Arena<Point>,
        triangles: Arena<DelaunayTriangle>,
        bounds: Bounds,
    ) -> Self {
        Self { dim, points, triangles, bounds }
    }
}
