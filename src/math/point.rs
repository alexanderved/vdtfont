/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Quadric and cubic bezier tesselation adapted from stb_truetype: https://github.com/nothings/stb
 */

use std::ops;

pub type TransformFn = Box<dyn Fn(Point) -> Point>;

/// A point with (`x`, `y`) coordinates.
#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Constructs new [`Point`].
    ///
    /// ```
    /// let p = Point::new(1.2, 5.3);
    /// assert!(p.x == 1.2 && p.y == 5.3);
    /// ```
    #[inline]
    pub const fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    /// Calculates squared distance between two points.
    pub fn distance_squared(&self, other: &Point) -> f32 {
        let p = *self - *other;

        p.x.powi(2) + p.y.powi(2)
    }

    /// Calculates distance between two points.
    #[inline]
    pub fn distance(&self, other: &Point) -> f32 {
        self.distance_squared(other).sqrt()
    }

    /// Calculates midpoint between two points.
    #[inline]
    pub fn midpoint(&self, other: &Point) -> Point {
        (*self + *other) / 2.0
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Add<f32> for Point {
    type Output = Point;

    fn add(self, rhs: f32) -> Self::Output {
        point(self.x + rhs, self.y + rhs)
    }
}

impl ops::Add<Point> for f32 {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        point(rhs.x + self, rhs.y + self)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign<f32> for Point {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Sub<f32> for Point {
    type Output = Point;

    fn sub(self, rhs: f32) -> Self::Output {
        point(self.x - rhs, self.y - rhs)
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::SubAssign<f32> for Point {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl ops::Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        point(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        point(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        point(rhs.x * self, rhs.y * self)
    }
}

impl ops::MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        point(self.x / rhs.x, self.y / rhs.y)
    }
}

impl ops::Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        point(self.x / rhs, self.y / rhs)
    }
}

impl ops::DivAssign for Point {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl ops::DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

/// Constructs new [`Point`].
///
/// See [`Point::new`].
///
/// ```
/// let p = Point::new(1.2, 5.3);
/// assert!(p.x == 1.2 && p.y == 5.3);
/// ```
#[inline]
pub const fn point(x: f32, y: f32) -> Point {
    Point::new(x, y)
}
