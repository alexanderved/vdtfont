mod factory;
mod point;
mod triangle;
mod bounds;

pub use factory::DelaunayFactory;
pub use point::*;
pub use triangle::*;
//pub(crate) use bounds::*;

use crate::list::List;

pub struct Delaunay {
    dim: usize,

    points: Vec<Point>,
    triangles: List<DelaunayTriangle>,
}

impl Delaunay {
    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn image(&self) -> Vec<u8> {
        let mut bitmap = vec![0.0; self.dim * self.dim];
        for t in self.triangles.data.iter().filter_map(|e| e.as_ref()) {
            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                self.points[t.vertices[0] as usize],
                self.points[t.vertices[1] as usize],
            );

            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                self.points[t.vertices[1] as usize],
                self.points[t.vertices[2] as usize],
            );

            crate::draw_line(
                &mut bitmap,
                self.dim,
                self.dim,
                self.points[t.vertices[0] as usize],
                self.points[t.vertices[2] as usize],
            );
        }

        bitmap.into_iter().flat_map(|a| [0, 0, 0, (255.0 * a) as u8]).collect()
    }

    fn new(dim: usize, points: Vec<Point>, triangles: List<DelaunayTriangle>) -> Self {
        Self { dim, points, triangles }
    }
}