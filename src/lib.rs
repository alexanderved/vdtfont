//! A novel library for converting glyphs into triangulations which can be
//! used to simplify text rendering in Game and Application Interfaces.
//!
//! ```
//! use vdtfont::{*, delaunay::*};
//!
//! // Create a font
//! let font_data = include_bytes!("/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf");
//! let mut font = Font::from_vec(font_data.to_vec()).unwrap();
//!
//! // Obtain a glyph
//! let glyph = font.glyph('a');
//! // Outline the glyph
//! let outlined_glyph = font.outline_glyph(glyph);
//! // Triangulate th glyph
//! let triangulated_glyph = font.triangulate_glyph(outlined_glyph).unwrap();
//!
//! // Use the resulting triangulation
//! triangulated_glyph
//!     .triangles()
//!     .handle_iter::<DelaunayTriangleHandle>(triangulated_glyph.points())
//!     .for_each(|triangle_handle| {
//!         // ...
//!     })
//! ```

pub extern crate arena_system;
pub extern crate ocl;
pub extern crate owned_ttf_parser as ttfp;

pub mod delaunay;
pub mod font;
pub mod opencl;
pub mod point;
pub mod voronoi;

pub use font::{Font, Glyph, OutlinedGlyph, TriangulatedGlyph};
pub use point::{Point, PointHandle, PointId};
