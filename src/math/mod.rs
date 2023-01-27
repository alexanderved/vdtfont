/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod primitives;
mod float;

pub use primitives::{
    Point, point,
    Curve,
    Line, line,
    QuadricCurve, quadric,
    CubicCurve, cubic
};
pub use float::ReverseFract;

pub const FLATNESS: f32 = 0.35;