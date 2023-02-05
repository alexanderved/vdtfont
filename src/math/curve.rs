/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Quadric and cubic bezier tesselation adapted from stb_truetype: https://github.com/nothings/stb
 */

use super::line::*;
use super::point::*;

const OBJSPACE_FLATNESS: f32 = 0.35;

/// Operations on Bezier curves.
pub trait Curve: Sized {
    /// Splits a curve into two curves in its midpoint.
    fn split(self) -> (Self, Self);

    /// Divides a curve into lines and pushes them to specified [`Vec`].
    fn tesselate(self, lines: &mut Vec<Line>);
}

#[derive(Debug, Default, Clone)]
pub struct Linear(pub Point, pub Point);

impl Curve for Linear {
    /// Splits a linear curve into two linear curves.
    fn split(self) -> (Self, Self) {
        let midpoint = self.0.midpoint(&self.1);

        (Linear(self.0, midpoint), Linear(midpoint, self.1))
    }

    /// Converts [`Linear`] into [`Line`].
    #[inline]
    fn tesselate(self, lines: &mut Vec<Line>) {
        lines.push(Line::new(self.0, self.1));
    }
}

/// A quadric Bezier curve.
#[derive(Debug, Default, Clone)]
pub struct Quadric(pub Point, pub Point, pub Point);

impl Curve for Quadric {
    /// Splits a quadric curve into two quadric curves.
    fn split(self) -> (Self, Self) {
        let mp01 = self.0.midpoint(&self.1);
        let mp12 = self.1.midpoint(&self.2);
        let midpoint = mp01.midpoint(&mp12);

        let q0 = Quadric(self.0, mp01, midpoint);
        let q1 = Quadric(midpoint, mp12, self.2);

        (q0, q1)
    }

    /// Recursively divides a quadric curve into lines.
    fn tesselate(self, lines: &mut Vec<Line>) {
        let mp01 = self.0.midpoint(&self.1);
        let mp12 = self.1.midpoint(&self.2);
        let midpoint = mp01.midpoint(&mp12);
        let distance_squared = self.0.midpoint(&self.2).distance_squared(&midpoint);

        if distance_squared > OBJSPACE_FLATNESS * OBJSPACE_FLATNESS {
            let (q0, q1) = self.split();
            q0.tesselate(lines);
            q1.tesselate(lines);
        } else {
            lines.push(Line::new(self.0, self.2));
        }
    }
}

/// A cubic Bezier curve.
#[derive(Debug, Default, Clone)]
pub struct Cubic(pub Point, pub Point, pub Point, pub Point);

impl Curve for Cubic {
    /// Splits a cubic curve into two cubic curves.
    fn split(self) -> (Self, Self) {
        let mp01 = self.0.midpoint(&self.1);
        let mp12 = self.1.midpoint(&self.2);
        let mp23 = self.2.midpoint(&self.3);

        let mp012 = mp01.midpoint(&mp12);
        let mp123 = mp12.midpoint(&mp23);

        let midpoint = mp012.midpoint(&mp123);

        let c0 = Cubic(self.0, mp01, mp012, midpoint);
        let c1 = Cubic(midpoint, mp123, mp23, self.3);

        (c0, c1)
    }

    /// Recursively divides a cubic curve into lines.
    fn tesselate(self, lines: &mut Vec<Line>) {
        let longlen =
            self.0.distance(&self.1) + self.1.distance(&self.2) + self.2.distance(&self.3);
        let shortlen = self.0.distance(&self.3);
        let flatness_squared = longlen.powi(2) - shortlen.powi(2);

        if flatness_squared > OBJSPACE_FLATNESS * OBJSPACE_FLATNESS {
            let (c0, c1) = self.split();
            c0.tesselate(lines);
            c1.tesselate(lines);
        } else {
            lines.push(Line::new(self.0, self.3))
        }
    }
}
