mod primitives;

pub use primitives::{
    Point, point,
    Curve,
    Line, line,
    QuadricCurve, quadric,
    CubicCurve, cubic
};

pub const FLATNESS: f32 = 0.35;