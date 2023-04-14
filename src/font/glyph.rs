use crate::delaunay::DelaunayTriangle;
use crate::Point;

use std::convert;

use arena_system::Arena;

/// A glyph with id which correspondes to one of the characters in the font.
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

/// An outlined glyph.
pub struct OutlinedGlyph {
    glyph: Glyph,
    dim: usize,

    bounds: ttfp::Rect,
    points: Arena<Point>,
}

impl OutlinedGlyph {
    /// Returns a glyph.
    pub fn glyph(&self) -> Glyph {
        self.glyph
    }

    /// Returns a dimension of the glyph.
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Returns bounds of the glyph.
    pub fn bounds(&self) -> ttfp::Rect {
        self.bounds
    }

    /// Returns points which the outline of the glyph consists of.
    pub fn points(&self) -> &Arena<Point> {
        &self.points
    }

    /// Converts [`OutlinedGlyph`] into raw parts: a glyph, a dimension, bounds and points.
    pub fn into_raw_parts(self) -> (Glyph, usize, ttfp::Rect, Arena<Point>) {
        (self.glyph, self.dim, self.bounds, self.points)
    }

    /// Creates a new [`OutlinedGlyph`].
    ///
    /// The validity of the given parameters is ensured by [`Font`].
    pub(super) fn new(glyph: Glyph, dim: usize, bounds: ttfp::Rect, points: Arena<Point>) -> Self {
        Self { glyph, dim, bounds, points }
    }
}

/// A triangulated glyph.
pub struct TriangulatedGlyph {
    glyph: Glyph,
    dim: usize,

    points: Arena<Point>,
    triangles: Arena<DelaunayTriangle>,
}

impl TriangulatedGlyph {
    /// Returns a glyph.
    pub fn glyph(&self) -> Glyph {
        self.glyph
    }

    /// Returns a dimension of the glyph.
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Returns points which the outline of the glyph consists of.
    pub fn points(&self) -> &Arena<Point> {
        &self.points
    }

    /// Returns triangles which the triangulation of the glyph consists of.
    pub fn triangles(&self) -> &Arena<DelaunayTriangle> {
        &self.triangles
    }

    /// Converts [`TriangulatedGlyph`] into raw parts: a glyph, a dimension, points and triangles
    pub fn into_raw_parts(self) -> (Glyph, usize, Arena<Point>, Arena<DelaunayTriangle>) {
        (self.glyph, self.dim, self.points, self.triangles)
    }

    /// Creates a new [`TriangulatedGlyph`].
    ///
    /// The validity of the given parameters is ensured by [`Font`].
    /// 
    /// [`Font`]: crate::font::Font
    pub(super) fn new(
        glyph: Glyph,
        dim: usize,
        points: Arena<Point>,
        triangles: Arena<DelaunayTriangle>,
    ) -> Self {
        Self { glyph, dim, points, triangles }
    }
}
