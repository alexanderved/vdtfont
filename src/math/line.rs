/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Quadric and cubic bezier tesselation adapted from stb_truetype: https://github.com/nothings/stb
 */

use super::point::*;
use std::{mem, ops};

/// A straight line from `p0` to `p1`.
///
/// If `p0.y` is greater than `p1.y`, `p0` and `p1` are swapped and
/// `dir` is set `-1`. Otherwise, `dir` equals `1`.
#[derive(Debug, Clone)]
pub struct Line {
    p0: Point,
    p1: Point,

    dx: f32,
    dy: f32,

    dir: i8,
}

impl Line {
    /// Constructs [`Line`].
    ///
    /// ```
    /// let l = Line::new(point(1.2, 5.3), point(7.4, 9.8));
    /// ```
    pub fn new(mut p0: Point, mut p1: Point) -> Line {
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

    /// Decomposes a [`Line`] to its raw parts.
    #[inline]
    pub fn to_raw_parts(&self) -> (Point, Point, f32, f32, i8) {
        let Line {
            p0,
            p1,
            dx,
            dy,
            dir,
        } = *self;
        (p0, p1, dx, dy, dir)
    }
}

impl ops::Add<f32> for Line {
    type Output = Line;

    fn add(mut self, rhs: f32) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 + rhs, self.p1 + rhs)
    }
}

impl ops::Add<Line> for f32 {
    type Output = Line;

    fn add(self, mut rhs: Line) -> Self::Output {
        if rhs.dir == -1 {
            mem::swap(&mut rhs.p0, &mut rhs.p1);
        }

        Line::new(self + rhs.p0, self + rhs.p1)
    }
}

impl ops::Add<Point> for Line {
    type Output = Line;

    fn add(mut self, rhs: Point) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 + rhs, self.p1 + rhs)
    }
}

impl ops::Add<Line> for Point {
    type Output = Line;

    fn add(self, mut rhs: Line) -> Self::Output {
        if rhs.dir == -1 {
            mem::swap(&mut rhs.p0, &mut rhs.p1);
        }

        Line::new(self + rhs.p0, self + rhs.p1)
    }
}

impl ops::AddAssign<f32> for Line {
    fn add_assign(&mut self, rhs: f32) {
        *self = self.clone() + rhs;
    }
}

impl ops::AddAssign<Point> for Line {
    fn add_assign(&mut self, rhs: Point) {
        *self = self.clone() + rhs;
    }
}

impl ops::Sub<f32> for Line {
    type Output = Line;

    fn sub(mut self, rhs: f32) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 - rhs, self.p1 - rhs)
    }
}

impl ops::Sub<Point> for Line {
    type Output = Line;

    fn sub(mut self, rhs: Point) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 - rhs, self.p1 - rhs)
    }
}

impl ops::SubAssign<f32> for Line {
    fn sub_assign(&mut self, rhs: f32) {
        *self = self.clone() - rhs;
    }
}

impl ops::SubAssign<Point> for Line {
    fn sub_assign(&mut self, rhs: Point) {
        *self = self.clone() - rhs;
    }
}

impl ops::Mul<f32> for Line {
    type Output = Line;

    fn mul(mut self, rhs: f32) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 * rhs, self.p1 * rhs)
    }
}

impl ops::Mul<Line> for f32 {
    type Output = Line;

    fn mul(self, mut rhs: Line) -> Self::Output {
        if rhs.dir == -1 {
            mem::swap(&mut rhs.p0, &mut rhs.p1);
        }

        Line::new(self * rhs.p0, self * rhs.p1)
    }
}

impl ops::Mul<Point> for Line {
    type Output = Line;

    fn mul(mut self, rhs: Point) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 * rhs, self.p1 * rhs)
    }
}

impl ops::Mul<Line> for Point {
    type Output = Line;

    fn mul(self, mut rhs: Line) -> Self::Output {
        if rhs.dir == -1 {
            mem::swap(&mut rhs.p0, &mut rhs.p1);
        }

        Line::new(self * rhs.p0, self * rhs.p1)
    }
}

impl ops::MulAssign<f32> for Line {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.clone() * rhs;
    }
}

impl ops::MulAssign<Point> for Line {
    fn mul_assign(&mut self, rhs: Point) {
        *self = self.clone() * rhs;
    }
}

impl ops::Div<f32> for Line {
    type Output = Line;

    fn div(mut self, rhs: f32) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 / rhs, self.p1 / rhs)
    }
}

impl ops::Div<Point> for Line {
    type Output = Line;

    fn div(mut self, rhs: Point) -> Self::Output {
        if self.dir == -1 {
            mem::swap(&mut self.p0, &mut self.p1);
        }

        Line::new(self.p0 / rhs, self.p1 / rhs)
    }
}

impl ops::DivAssign<f32> for Line {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.clone() / rhs;
    }
}

impl ops::DivAssign<Point> for Line {
    fn div_assign(&mut self, rhs: Point) {
        *self = self.clone() / rhs;
    }
}