use crate::Point;

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

    bounds: ttfp::Rect,
    points: Arena<Point>,
}

impl OutlinedGlyph {
    pub(super) fn new(glyph: Glyph, bounds: ttfp::Rect, points: Arena<Point>) -> Self {
        Self { glyph, bounds, points }
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

    pub fn into_raw_parts(self) -> (Glyph, ttfp::Rect, Arena<Point>) {
        (self.glyph, self.bounds, self.points)
    }
}
