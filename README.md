VDTFont
[![crates.io](https://img.shields.io/crates/v/vdtfont.svg)](https://crates.io/crates/vdtfont)
[![Documentation](https://docs.rs/vdtfont/badge.svg)](https://docs.rs/vdtfont)
==============
A novel library for converting glyphs into triangulations which can be used when rendering text
in Game and Application interfaces.

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

## Overview

VDTFont uses OpenCL to build Voronoi diagram and compute Delaunay triangulation with points from the glyph. The triangulation of the glyph can used for its rendering.

Full algorithm of triangulation is described in the paper ["Computing Two-dimensional Delaunay Triangulation Using Graphics Hardware"](https://www.comp.nus.edu.sg/%7Etants/delaunay/GPUDT.pdf).

## What's new?

The original font_rasterizer wasn't competetive so it was decided to almost fully rewrite it.

A new library VDTFont doesn't use classical method of rasterizing every pixel, but
triangulates glyphs using GPU.

## Dependencies

* ocl-icd-libopencl1
* opencl-headers
* OpenCL drivers for your GPU (e.g. intel-opencl-icd for Intel GPU)

## Usage

Run the following Cargo command in your Rust project directory:
```bash
$ cargo add vdtfont
```

Or add the following line to your Cargo.toml:
```
vdtfont = "0.3.0"
```

## Build

Run the following Cargo command in the project directory:
```bash
$ cargo build --release
```

## Example

Run the following command:

```bash
# build it
$ cargo build --release --example simple
# run it
$ ./target/release/examples/simple
```

## Roadmap

* Add C FFI
* Fix inserting edge algorithm
* Add function to fix self-intersecting outlines of glyphs