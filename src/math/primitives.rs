/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 * 
 * Quadric and cubic bezier tesselation adapted from stb_truetype: https://github.com/nothings/stb
 */

use super::FLATNESS;

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

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y)
    }
}

pub fn point(x: f32, y: f32) -> Point {
    Point { x, y }
}

pub trait Curve {
    fn tesselate(self, lines: &mut Vec<Line>);
}

#[derive(Debug, Default, Clone)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,

    pub dx: f32,
    pub dy: f32,

    pub dir: i8,
}

impl Curve for Line {
    fn tesselate(self, lines: &mut Vec<Line>) {
        lines.push(self);
    }
}

impl std::cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.p0 == other.p0
    }
}

impl std::cmp::PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.p0.partial_cmp(&other.p0)
    }
}

pub fn line(mut p0: Point, mut p1: Point) -> Line {
    let dx = (p1.x - p0.x) / (p1.y - p0.y);
    let dy = (p1.y - p0.y) / (p1.x - p0.x);
    let mut dir = 1;

    if p0.y > p1.y {
        (p0, p1) = (p1, p0);
        dir = -1;
    }

    Line { p0, p1, dx, dy, dir }
}

pub struct QuadricCurve {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
}

impl Curve for QuadricCurve {
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

pub fn quadric(p0: Point, p1: Point, p2: Point) -> QuadricCurve {
    QuadricCurve { p0, p1, p2 }
}

pub struct CubicCurve {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Curve for CubicCurve {
    fn tesselate(self, lines: &mut Vec<Line>) {
        let dp0 = self.p1 - self.p0;
        let dp1 = self.p2 - self.p1;
        let dp2 = self.p3 - self.p2;
        let dp = self.p3 - self.p0;

        let longlen = (dp0.x.powi(2) + dp0.y.powi(2)).sqrt() + 
            (dp1.x.powi(2) + dp1.y.powi(2)).sqrt() + (dp2.x.powi(2) + dp2.y.powi(2)).sqrt();
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

pub fn cubic(p0: Point, p1: Point, p2: Point, p3: Point) -> CubicCurve {
    CubicCurve { p0, p1, p2, p3 }
}