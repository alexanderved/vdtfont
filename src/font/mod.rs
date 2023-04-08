pub mod curve;
pub mod glyph;
pub mod outliner;

pub use glyph::*;

use ttfp::AsFaceRef;

pub struct Font(ttfp::PreParsedSubtables<'static, ttfp::OwnedFace>);

impl Font {
    #[inline]
    pub fn from_vec(data: Vec<u8>) -> anyhow::Result<Self> {
        Self::from_vec_and_index(data, 0)
    }

    #[inline]
    pub fn from_vec_and_index(data: Vec<u8>, index: u32) -> anyhow::Result<Self> {
        Ok(Self(ttfp::PreParsedSubtables::from(ttfp::OwnedFace::from_vec(data, index)?)))
    }

    #[inline]
    pub fn units_per_em(&self) -> Option<f32> {
        Some(self.0.as_face_ref().units_per_em().into())
    }

    #[inline]
    pub fn ascender(&self) -> f32 {
        self.0.as_face_ref().ascender().into()
    }

    #[inline]
    pub fn descender(&self) -> f32 {
        self.0.as_face_ref().descender().into()
    }

    #[inline]
    pub fn line_gap(&self) -> f32 {
        self.0.as_face_ref().line_gap().into()
    }

    #[inline]
    pub fn glyph_id(&self, c: char) -> Glyph {
        let index = self.0.glyph_index(c).map(|id| id.0).unwrap_or(0);
        Glyph(index)
    }

    #[inline]
    pub fn hor_advance(&self, id: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_hor_advance(id.into())
            .expect("Invalid glyph_hor_advance")
            .into()
    }

    #[inline]
    pub fn hor_side_bearing(&self, id: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_hor_side_bearing(id.into())
            .expect("Invalid glyph_hor_side_bearing")
            .into()
    }

    #[inline]
    pub fn ver_advance(&self, id: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_ver_advance(id.into())
            .expect("Invalid glyph_ver_advance")
            .into()
    }

    #[inline]
    pub fn ver_side_bearing(&self, id: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_ver_side_bearing(id.into())
            .expect("Invalid glyph_ver_side_bearing")
            .into()
    }

    #[inline]
    pub fn hor_kerning(&self, first: Glyph, second: Glyph) -> f32 {
        self.0
            .glyphs_hor_kerning(first.into(), second.into())
            .map(f32::from)
            .unwrap_or_default()
    }

    #[inline]
    pub fn glyph_count(&self) -> usize {
        self.0.as_face_ref().number_of_glyphs() as _
    }
}
