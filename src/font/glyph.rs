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

    points: Arena<Point>,
}

impl OutlinedGlyph {
    pub(super) fn new(glyph: Glyph, points: Arena<Point>) -> Self {
        Self {
            glyph,
            points,
        }
    }
}