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

        let (edge_track, triangle_track) = edge.find_triangle_track();
        let mut polygon: Polygon = Polygon::from(&triangle_track)
            .points()
            .iter()
            .filter(|p| !edge.points().contains(p))
            .map(|p| *p)
            .collect::<Vec<_>>()
            .into();
        polygon.sort_by_angle(edge.points()[0]);

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

        let mut tri0 = self.triangulate_hole(contour0);
        tri0
            .iter_mut()
            .for_each(|t| t.set_is_visible(true));


        let mut tri1 = self.triangulate_hole(contour1);
        tri1
            .iter_mut()
            .for_each(|t| t.set_is_visible(true));

        for t in tri0 {
            self.triangles.add(t);
        }

        for t in tri1 {
            self.triangles.add(t);
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

            //if t.is_counterclockwise(self.points()) {
                let r = t.circumcircle_radius(self.points());

                if r < smallest_circle {
                    smallest_circle = r;
                    smallest_triangle = Some(t);
                    middle_vertex = i + 1;
                }
            //}
        }

        contour.remove(middle_vertex);

        let mut res = vec![smallest_triangle.unwrap()];

        if contour.len() >= 3 {
            res.append(&mut self.triangulate_hole(contour));
        }

        res
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
