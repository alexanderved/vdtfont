mod primitives;
mod float;

pub use primitives::{
    Point, point,
    Curve,
    Line, line,
    QuadricCurve, quadric,
    CubicCurve, cubic
};
pub use float::*;

pub const FLATNESS: f32 = 0.35;