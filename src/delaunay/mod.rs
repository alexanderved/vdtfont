mod bounds;
mod factory;
mod triangle;
mod triangle_fan;
mod util;

pub(crate) use bounds::*;
pub use factory::DelaunayFactory;
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
