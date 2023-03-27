mod bounds;
mod factory;
mod point;
mod triangle;

pub use factory::DelaunayFactory;
pub use point::*;
pub use triangle::*;
//pub(crate) use bounds::*;

use arena_system::{Arena, Handle};

pub struct Delaunay {
    dim: usize,

    points: Arena<DelaunayPoint>,
    triangles: Arena<DelaunayTriangle>,
}

impl Delaunay {
    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn points(&self) -> &Arena<DelaunayPoint> {
        &self.points
    }

    pub fn image(&self) -> Vec<u8> {
        let mut bitmap = vec![0.0; self.dim * self.dim];

        let mut i: i64 = 0;
        let mut tri = self
            .triangles
            .handle::<DelaunayTriangleHandle>(i.into(), &self.points)
            .get();

        while let Ok(t) = tri {
            let t = &t.unwrap();

            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                self.points
                    .handle::<DelaunayPointHandle>(t.vertices[0].into(), ())
                    .get()
                    .unwrap()
                    .unwrap(),
                self.points
                    .handle::<DelaunayPointHandle>(t.vertices[1].into(), ())
                    .get()
                    .unwrap()
                    .unwrap(),
            );

            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                self.points
                    .handle::<DelaunayPointHandle>(t.vertices[1].into(), ())
                    .get()
                    .unwrap()
                    .unwrap(),
                self.points
                    .handle::<DelaunayPointHandle>(t.vertices[2].into(), ())
                    .get()
                    .unwrap()
                    .unwrap(),
            );

            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                self.points
                    .handle::<DelaunayPointHandle>(t.vertices[0].into(), ())
                    .get()
                    .unwrap()
                    .unwrap(),
                self.points
                    .handle::<DelaunayPointHandle>(t.vertices[2].into(), ())
                    .get()
                    .unwrap()
                    .unwrap(),
            );

            i += 1;
            tri = self
                .triangles
                .handle::<DelaunayTriangleHandle>(i.into(), &self.points)
                .get();
        }

        bitmap.into_iter().flat_map(|a| [0, 0, 0, (255.0 * a) as u8]).collect()
    }

    fn new(dim: usize, points: Arena<DelaunayPoint>, triangles: Arena<DelaunayTriangle>) -> Self {
        Self { dim, points, triangles }
    }
}
