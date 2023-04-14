mod common;

use vdtfont::*;
use std::io::Cursor;

const OPENSANS_REGULAR: &'static [u8] =
    include_bytes!("/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf");

#[test]
fn test_opensans_regular_r() {
    let mut font = Font::from_vec(OPENSANS_REGULAR.to_vec()).unwrap();

    let glyph = font.glyph('r');
    let outlined_glyph = font.outline_glyph(glyph);
    let triangulated_glyph = font.triangulate_glyph(outlined_glyph).unwrap();

    let glyph = common::rasterize_glyph(&triangulated_glyph);
    let reference_glyph = image::load(
        Cursor::new(include_bytes!("../reference_glyphs/opensans_regular_r.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .as_bytes()
    .to_vec();

    for (g, rg) in glyph.into_iter().zip(reference_glyph) {
        assert_eq!(g, rg);
    }
}
