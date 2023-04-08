use crate::Point;
use crate::delaunay::{DelaunayTriangle, DelaunayTriangleHandle};
use crate::point::PointHandle;

use std::convert;

use arena_system::{Arena, Handle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Glyph(pub u16);

impl convert::From<u16> for Glyph {
    fn from(value: u16) -> Self {
        Glyph(value)
    }
}

impl convert::From<ttfp::GlyphId> for Glyph {
    fn from(value: ttfp::GlyphId) -> Self {
        Glyph(value.0)
    }
}

impl convert::From<Glyph> for u16 {
    fn from(glyph: Glyph) -> Self {
        glyph.0
    }
}

impl convert::From<Glyph> for ttfp::GlyphId {
    fn from(glyph: Glyph) -> Self {
        Self(glyph.0)
    }
}

pub struct OutlinedGlyph {
    glyph: Glyph,
    dim: usize,

    bounds: ttfp::Rect,
    points: Arena<Point>,
}

impl OutlinedGlyph {
    pub(super) fn new(glyph: Glyph, dim: usize, bounds: ttfp::Rect, points: Arena<Point>) -> Self {
        Self { glyph, dim, bounds, points }
    }

    pub fn glyph(&self) -> Glyph {
        self.glyph
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn bounds(&self) -> ttfp::Rect {
        self.bounds
    }

    pub fn points(&self) -> &Arena<Point> {
        &self.points
    }

    pub fn into_raw_parts(self) -> (Glyph, usize, ttfp::Rect, Arena<Point>) {
        (self.glyph, self.dim, self.bounds, self.points)
    }
}

pub struct TriangulatedGlyph {
    glyph: Glyph,
    dim: usize,

    points: Arena<Point>,
    triangles: Arena<DelaunayTriangle>,
}

impl TriangulatedGlyph {
    pub(super) fn new(
        glyph: Glyph,
        dim: usize,
        points: Arena<Point>,
        triangles: Arena<DelaunayTriangle>
    ) -> Self {
        Self { glyph, dim, points, triangles }
    }

    pub fn glyph(&self) -> Glyph {
        self.glyph
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn points(&self) -> &Arena<Point> {
        &self.points
    }

    pub fn triangles(&self) -> &Arena<DelaunayTriangle> {
        &self.triangles
    }

    pub fn into_raw_parts(self) -> (Glyph, usize, Arena<Point>, Arena<DelaunayTriangle>) {
        (self.glyph, self.dim, self.points, self.triangles)
    }

    pub fn image(&self) -> Vec<u8> {
        let mut bitmap = vec![0.0; self.dim * self.dim];

        self.triangles
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

        bitmap.into_iter().flat_map(|a| [0, 0, 0, (255.0 * (1.0 - a)) as u8]).collect()
    }
}