VDTFont
[![crates.io](https://img.shields.io/crates/v/vdtfont.svg)](https://crates.io/crates/vdtfont)
[![Documentation](https://docs.rs/vdtfont/badge.svg)](https://docs.rs/vdtfont)
==============
A novel library for converting glyphs into triangulations which can be used right in the Graphical APIs.

```rust
use vdtfont::{*, delaunay::*};

// Create a font
let font_data = include_bytes!("/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf");
let mut font = Font::from_vec(font_data.to_vec())?;

// Obtain a glyph
let glyph = font.glyph('a');
// Outline the glyph
let outlined_glyph = font.outline_glyph(glyph);
// Triangulate th glyph
let triangulated_glyph = font.triangulate_glyph(outlined_glyph)?;

// Use the resulting triangulation
triangulated_glyph
    .triangles()
    .handle_iter::<DelaunayTriangleHandle>(triangulated_glyph.points())
    .for_each(|triangle_handle| {
        // ...
    })
```

## What's new?

The original font_rasterizer wasn't competetive so it was decided to almost fully rewrite it.

A new library VDTFont doesn't use classical method of rasterizing every pixel, but
triangulates glyphs using GPU.

## Example

To run the example use the following command:

```bash
cargo run --release --example simple
```