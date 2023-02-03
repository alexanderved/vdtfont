/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Rasterization for linear, quadric and cubic Bezier curves.
//!
//! ```
//! use font_rasterizer::CanvasBuilder;
//!
//! // Add outline curves to CanvasBuilder
//! let canvas_builder = CanvasBuilder::new(width, height)
//!     .add_curve(line(l0, l1))
//!     .add_curve(quadric(q0, q1, q2))
//!     .add_curve(cubic(c0, c1, c2, c3));
//!
//! // Draw outlines and fill them
//! let canvas = canvas_builder.build();
//!
//! // Iterate over resultant pixel alphas
//! canvas.iter()
//!     .for_each(|alpha| {
//!         // ...
//!     })
//! ```

mod canvas;
mod math;

pub use canvas::*;
pub use math::*;
