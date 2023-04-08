pub mod curve;
pub mod glyph;
pub mod outliner;

pub use glyph::*;

use crate::point::*;

use arena_system::Arena;
use ttfp::AsFaceRef;
use ocl::prm::Float2;

const MIN_GLYPH_HEIGHT: usize = 256;
const MAX_GLYPH_HEIGHT: usize = 2048;

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
    pub fn glyph(&self, c: char) -> Glyph {
        let index = self.0.glyph_index(c).map(|id| id.0).unwrap_or(0);
        Glyph(index)
    }

    #[inline]
    pub fn hor_advance(&self, glyph: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_hor_advance(glyph.into())
            .expect("Invalid glyph_hor_advance")
            .into()
    }

    #[inline]
    pub fn hor_side_bearing(&self, glyph: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_hor_side_bearing(glyph.into())
            .expect("Invalid glyph_hor_side_bearing")
            .into()
    }

    #[inline]
    pub fn ver_advance(&self, glyph: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_ver_advance(glyph.into())
            .expect("Invalid glyph_ver_advance")
            .into()
    }

    #[inline]
    pub fn ver_side_bearing(&self, glyph: Glyph) -> f32 {
        self.0
            .as_face_ref()
            .glyph_ver_side_bearing(glyph.into())
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

    pub fn outline_glyph(&self, glyph: Glyph) -> OutlinedGlyph {
        let mut outliner = outliner::Outliner::new();
        let rect = self.0.as_face_ref().outline_glyph(glyph.into(), &mut outliner).unwrap();

        let dim = nearest_power_of_two(
            (MAX_GLYPH_HEIGHT as f32 * 4.0 / outliner.shortest_distance) as usize,
        )
        .clamp(MIN_GLYPH_HEIGHT, MAX_GLYPH_HEIGHT);

        let height: f32 = self.ascender() - self.descender();
        let h_factor = dim as f32 / height;
        let v_factor = dim as f32 / height;

        let bounds = ttfp::Rect {
            x_min: (rect.x_min as f32 * h_factor) as i16,
            x_max: (rect.x_max as f32 * h_factor) as i16,
            y_min: (rect.y_min as f32 * v_factor) as i16,
            y_max: (rect.y_max as f32 * v_factor) as i16,
        };

        outliner.points
            .iter_mut()
            .for_each(|p| {
                let new_x = p.x() * h_factor - bounds.x_min as f32;
                let new_y = bounds.height() as f32 - p.y() * v_factor + bounds.y_min as f32;

                p.set_coords(Float2::new(new_x, new_y));
            });

        let points: Arena<Point> = outliner.points.into();

        OutlinedGlyph::new(glyph, bounds, points)
    }
}

fn is_power_of_two(n: usize) -> bool {
    2usize.pow(n.ilog2()) == n
}

fn nearest_power_of_two(n: usize) -> usize {
    if is_power_of_two(n) {
        return n;
    }

    2usize.pow(n.ilog2() + 1)
}
