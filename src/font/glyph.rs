use crate::Point;
use crate::delaunay::DelaunayTriangle;

use std::convert;

use arena_system::Arena;

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

impl convert::Into<u16> for Glyph {
    fn into(self) -> u16 {
        self.0
    }
}

impl convert::Into<ttfp::GlyphId> for Glyph {
    fn into(self) -> ttfp::GlyphId {
        ttfp::GlyphId(self.0)
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

    points: Arena<Point>,
    triangles: Arena<DelaunayTriangle>,
}

impl TriangulatedGlyph {
    pub(super) fn new(
        glyph: Glyph,
        points: Arena<Point>,
        triangles: Arena<DelaunayTriangle>
    ) -> Self {
        Self { glyph, points, triangles }
    }

    pub fn points(&self) -> &Arena<Point> {
        &self.points
    }

    pub fn triangles(&self) -> &Arena<DelaunayTriangle> {
        &self.triangles
    }

    pub fn into_raw_parts(self) -> (Glyph, Arena<Point>, Arena<DelaunayTriangle>) {
        (self.glyph, self.points, self.triangles)
    }
}