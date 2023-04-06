mod bounds;
mod edge;
mod factory;
mod triangle;
mod triangle_fan;

pub(crate) use bounds::*;
pub use edge::*;
pub use factory::DelaunayFactory;
pub use triangle::*;

use crate::point::*;

use std::ops::ControlFlow;

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

        let t = edge.points()[0].triangle_fan().into_iter().try_for_each(|t| {
            let opposite_edge = t.opposite_edge_to(edge.points()[0]);

            // println!("Edge: {:?}", edge);
            // println!("Opposite edge: {:?}", opposite_edge);

            if opposite_edge.contains(edge.points()[1]) {
                return ControlFlow::Break(t);
            }

            if opposite_edge.intersects(&edge) {
                let tri_edge = opposite_edge;
                let prev = t;
                let next = prev.neighbour_on_edge(tri_edge);

                println!("Tri edge: {:?}", tri_edge);

                let edges = next.edges();
                println!("{:?}", edges);

                if edges[0].intersects(&edge) && edges[0] != tri_edge {
                    println!("1 {:?}", edges[0]);
                }
                if edges[1].intersects(&edge) && edges[1] != tri_edge {
                    println!("2 {:?}", edges[1]);
                }
                if edges[2].intersects(&edge) && edges[2] != tri_edge {
                    println!("3 {:?}", edges[2]);
                }

                return ControlFlow::Break(t);
            }

            ControlFlow::Continue(())
        });

        let t = match t {
            ControlFlow::Break(t) => t,
            _ => panic!("Triangle not found"),
        };
        println!("{:?}", t.index());
    }

    pub fn image(&self) -> Vec<u8> {
        let mut bitmap = vec![0.0; self.dim * self.dim];

        let mut i: i64 = 0;
        let mut tri = self
            .triangles
            .handle::<DelaunayTriangleHandle>(i.into(), &self.points)
            .get();

        while let Ok(t) = tri {
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
