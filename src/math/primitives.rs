/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Quadric and cubic bezier tesselation adapted from stb_truetype: https://github.com/nothings/stb
 */

use super::FLATNESS;

/// A point with (`x`, `y`) coordinates.
#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        point(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        point(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        point(rhs.x * self, rhs.y * self)
    }
}

impl std::ops::Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        point(self.x / rhs.x, self.y / rhs.y)
    }
}

impl std::ops::Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        point(self.x / rhs, self.y / rhs)
    }
}

/// Constructs [`Point`].
/// 
/// ```
/// let p = point(1.2, 5.3);
/// assert!(p.x == 1.2 && p.y == 5.3);
/// ```
pub fn point(x: f32, y: f32) -> Point {
    Point { x, y }
}

/// Operations on curves.
pub trait Curve {
    /// Divides a curve into lines and pushes them to specified [`Vec`].
    fn tesselate(self, lines: &mut Vec<Line>);
}

/// A straight line from `p0` to `p1`.
#[derive(Debug, Default, Clone)]
pub struct Line {
    p0: Point,
    p1: Point,

    dx: f32,
    dy: f32,

    dir: i8,
}

impl Line {
    /// Returns the reference to the starting point of a line.
    #[inline]
    pub fn p0(&self) -> &Point {
        &self.p0
    }

    /// Returns the reference to the end point of a line.
    #[inline]
    pub fn p1(&self) -> &Point {
        &self.p1
    }

    /// Returns the delta x of a line.
    #[inline]
    pub fn dx(&self) -> f32 {
        self.dx
    }

    /// Returns the delta y of a line.
    #[inline]
    pub fn dy(&self) -> f32 {
        self.dy
    }

    /// Returns the direction of a line.
    #[inline]
    pub fn dir(&self) -> i8 {
        self.dir
    }

    /// Applies specified function to end points of a line.
    /// 
    /// ```
    /// let mut l = line(point(0.0, 0.0), point(10.0, 10.0));
    /// let increment_x = |p: &mut Point| p.x += 1.0;
    /// l.transform(increment_x);
    /// 
    /// assert!(l.p0().x == 1.0 && l.p1().x == 11.0);
    /// ```
    pub fn transform<F>(&mut self, f: F)
    where
        F: Fn(&mut Point),
    {
        if self.dir == -1 {
            std::mem::swap(&mut self.p0, &mut self.p1);
        }

        f(&mut self.p0);
        f(&mut self.p1);

        *self = line(self.p0, self.p1);
    }
}

impl Curve for Line {
    /// Pushes line to specifed [`Vec`] because it is simple enough.
    #[inline]
    fn tesselate(self, lines: &mut Vec<Line>) {
        lines.push(self);
    }
}

/// Constructs [`Line`].
/// 
/// ```
/// let l = line(point(1.2, 5.3), point(7.4, 9.8));
/// ```
pub fn line(mut p0: Point, mut p1: Point) -> Line {
    let dx = (p1.x - p0.x) / (p1.y - p0.y);
    let dy = (p1.y - p0.y) / (p1.x - p0.x);
    let mut dir = 1;

    if p0.y > p1.y {
        (p0, p1) = (p1, p0);
        dir = -1;
    }

    Line {
        p0,
        p1,
        dx,
        dy,
        dir,
    }
}

/// A quadric Bezier curve.
pub struct QuadricCurve {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
}

impl Curve for QuadricCurve {
    /// Recursively divides a quadric curve into lines.
    fn tesselate(self, lines: &mut Vec<Line>) {
        let mid_p = (self.p0 + 2.0 * self.p1 + self.p2) / 4.0;
        let dp = (self.p0 + self.p2) / 2.0 - mid_p;

        if dp.x * dp.x + dp.y * dp.y > FLATNESS * FLATNESS {
            quadric(self.p0, (self.p0 + self.p1) / 2.0, mid_p).tesselate(lines);
            quadric(mid_p, (self.p1 + self.p2) / 2.0, self.p2).tesselate(lines);
        } else {
            lines.push(line(self.p0, self.p2));
        }
    }
}

/// Constructs [`QuadricCurve`].
/// 
/// ```
/// let q = quadric(point(1.2, 5.3), point(7.4, 9.8), point(15.4, 6.9));
/// ```
#[inline]
pub fn quadric(p0: Point, p1: Point, p2: Point) -> QuadricCurve {
    QuadricCurve { p0, p1, p2 }
}

/// A cubic Bezier curve.
pub struct CubicCurve {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Curve for CubicCurve {
    /// Recursively divides a cubic curve into lines.
    fn tesselate(self, lines: &mut Vec<Line>) {
        let dp0 = self.p1 - self.p0;
        let dp1 = self.p2 - self.p1;
        let dp2 = self.p3 - self.p2;
        let dp = self.p3 - self.p0;

        let longlen = (dp0.x.powi(2) + dp0.y.powi(2)).sqrt()
            + (dp1.x.powi(2) + dp1.y.powi(2)).sqrt()
            + (dp2.x.powi(2) + dp2.y.powi(2)).sqrt();
        let shortlen = (dp.x.powi(2) + dp.y.powi(2)).sqrt();
        let flatness_squared = longlen.powi(2) - shortlen.powi(2);

        if flatness_squared > FLATNESS * FLATNESS {
            let p01 = (self.p0 + self.p1) / 2.0;
            let p12 = (self.p1 + self.p2) / 2.0;
            let p23 = (self.p2 + self.p3) / 2.0;

            let p012 = (p01 + p12) / 2.0;
            let p123 = (p12 + p23) / 2.0;

            let mid_p = (p012 + p123) / 2.0;

            cubic(self.p0, p01, p012, mid_p).tesselate(lines);
            cubic(mid_p, p123, p23, self.p3).tesselate(lines);
        } else {
            lines.push(line(self.p0, self.p3))
        }
    }
}

/// Constructs [`CubicCurve`].
/// 
/// ```
/// let c = cubic(point(1.2, 5.3), point(7.4, 9.8), point(10.1, 8.5), point(15.4, 6.9));
/// ```
#[inline]
pub fn cubic(p0: Point, p1: Point, p2: Point, p3: Point) -> CubicCurve {
    CubicCurve { p0, p1, p2, p3 }
}
