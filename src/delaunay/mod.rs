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

use arena_system::{Arena, Handle};

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

        let triangle_track = edge.find_triangle_track();
        let _polygon = Polygon::from(&triangle_track);

        //println!("{:?}", polygon);
    }

    pub fn image(&self) -> Vec<u8> {
        let mut bitmap = vec![0.0; self.dim * self.dim];

        let mut i: i64 = 0;
        let mut tri = self
            .triangles
            .handle::<DelaunayTriangleHandle>(i.into(), &self.points)
            .get();

        while let Ok(t) = tri {
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

            i += 1;
            tri = self
                .triangles
                .handle::<DelaunayTriangleHandle>(i.into(), &self.points)
                .get();
        }

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
