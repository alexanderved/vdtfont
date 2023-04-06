mod bounds;
mod factory;
mod triangle;
mod triangle_fan;
mod util;
mod edge;

pub(crate) use bounds::*;
pub use factory::DelaunayFactory;
pub use triangle::*;
pub use edge::*;

use crate::point::*;

use std::ops::ControlFlow;

use arena_system::{Arena, Handle};
use smallvec::SmallVec;

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
        let edge = edge
            .into_iter()
            .map(|p| self.points().handle(p.into(), Some(&self.triangles)))
            .collect::<SmallVec<[PointHandle; 2]>>()
            .into_inner()
            .unwrap();

        let t = edge[0]
            .triangle_fan()
            .into_iter()
            .try_for_each(|t| {
                let opposite_edge = t.opposite_edge_to(edge[0]);

                if opposite_edge.contains(&edge[1]) {
                    return ControlFlow::Break(t);
                }

                if util::lines_intersect(opposite_edge, edge) {
                    let tri_edge = opposite_edge;
                    let prev = t;
                    let next = prev.neighbour_on_edge(tri_edge);

                    println!("Tri edge: {:?}", tri_edge);

                    let vertices = next.points();
                    let e0 = [vertices[0], vertices[1]];
                    let e1 = [vertices[1], vertices[2]];
                    let e2 = [vertices[2], vertices[0]];

                    if util::lines_intersect(e0, edge)
                        && !util::are_lines_equal(e0, tri_edge)
                    {
                        println!("1 {:?}", e0);
                    }
                    if util::lines_intersect(e1, edge)
                        && !util::are_lines_equal(e1, tri_edge)
                    {
                        println!("2 {:?}", e1);
                    }
                    if util::lines_intersect(e2, edge)
                        && !util::are_lines_equal(e2, tri_edge)
                    {
                        println!("3 {:?}", e2);
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
                (*self.points.handle::<PointHandle>(t.vertices[0].into(), None).get().unwrap())
                    .clone(),
                (*self.points.handle::<PointHandle>(t.vertices[1].into(), None).get().unwrap())
                    .clone(),
            );

            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                (*self.points.handle::<PointHandle>(t.vertices[1].into(), None).get().unwrap())
                    .clone(),
                (*self.points.handle::<PointHandle>(t.vertices[2].into(), None).get().unwrap())
                    .clone(),
            );

            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                (*self.points.handle::<PointHandle>(t.vertices[0].into(), None).get().unwrap())
                    .clone(),
                (*self.points.handle::<PointHandle>(t.vertices[2].into(), None).get().unwrap())
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
