/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::math::*;
use std::{iter::IntoIterator, mem, slice, vec};

/// An object that contains pixel alphas of drawn curves.
pub struct Canvas {
    width: usize,
    height: usize,
    bitmap: Vec<f32>,
}

impl Canvas {
    /// Constructs new [`Canvas`].
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            bitmap: vec![0.0; width * height],
        }
    }

    /// Returns the width of [`Canvas`].
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of [`Canvas`].
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Draws line in [`Canvas`] with
    /// [Xiaolin Wu's line algorithm](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm).
    ///
    /// ```
    /// canvas.draw_line(&line(l0, l1));
    /// ```
    pub fn draw_line(&mut self, line: &Line) {
        let (mut p0, mut p1, dx, dy, _) = line.to_raw_parts();
        let steep = (p1.x - p0.x).abs() < (p1.y - p0.y).abs();
        let delta = if steep { dx } else { dy };
        let boundary = if steep { self.height } else { self.width };

        if steep {
            mem::swap(&mut p0.x, &mut p0.y);
            mem::swap(&mut p1.x, &mut p1.y);
        }

        if p0.x == p1.x {
            return;
        }

        if p0.x > p1.x {
            mem::swap(&mut p0, &mut p1);
        }

        let i0 = p0.x.round();
        let i1 = p1.x.round();

        let mut prev_i = p0.x;
        let mut j = p0.y;

        for i in i0 as usize..boundary.min(i1 as usize + 1) {
            j += delta * (i as f32 - prev_i);

            if steep {
                self.plot(j as usize, i, j.rfract());
                self.plot(j as usize + 1, i, j.fract());
            } else {
                self.plot(i, j as usize, j.rfract());
                self.plot(i, j as usize + 1, j.fract());
            }

            prev_i = i as f32;
        }
    }

    /// Plots one pixel on [`Canvas`] if it's inside the bounds.
    ///
    /// ```
    /// canvas.plot(x, y, alpha);
    /// ```
    pub fn plot(&mut self, x: usize, y: usize, c: f32) {
        if x < self.width && y < self.height {
            // SAFETY: `x` and `y` are inside bounds which makes this function safe.
            unsafe {
                let pixel = self.bitmap.get_unchecked_mut(x + y * self.width);
                *pixel = c.max(*pixel);
            }
        }
    }

    /// Returns an iterator over pixel alphas in [`Canvas`].
    ///
    /// ```
    /// canvas.iter()
    ///     .for_each(|alpha| {
    ///         // ...
    ///     })
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'_ f32> {
        self.bitmap.iter()
    }

    /// Returns an iterator over pixel alphas in [`Canvas`] that allows modify them.
    ///
    /// ```
    /// canvas.iter_mut()
    ///     .for_each(|alpha| {
    ///         *alpha = some_value;
    ///
    ///         // ...
    ///     })
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut f32> {
        self.bitmap.iter_mut()
    }
}

impl IntoIterator for Canvas {
    type Item = f32;
    type IntoIter = vec::IntoIter<f32>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bitmap.into_iter()
    }
}

impl<'a> IntoIterator for &'a Canvas {
    type Item = &'a f32;
    type IntoIter = slice::Iter<'a, f32>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bitmap.iter()
    }
}

impl<'a> IntoIterator for &'a mut Canvas {
    type Item = &'a mut f32;
    type IntoIter = slice::IterMut<'a, f32>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bitmap.iter_mut()
    }
}

/// An object that stores outlines and allows building [`Canvas`] with them.
pub struct CanvasBuilder {
    width: usize,
    height: usize,

    curves: Vec<Curve>,

    transform: TransformFn,
}

impl CanvasBuilder {
    /// Creates new [`CanvasBuilder`].
    ///
    /// ```
    /// let canvas_builder = CanvasBuilder::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            curves: vec![],
            transform: Box::new(|p| p),
        }
    }

    /// Sets `width` for [`Canvas`].
    ///
    /// ```
    /// canvas_builder.width(600);
    /// ```
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = width;

        self
    }

    /// Sets `height` for [`Canvas`].
    ///
    /// ```
    /// canvas_builder.height(800);
    /// ```
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.height = height;

        self
    }

    pub fn transform(&mut self, transform: TransformFn) -> &mut Self {
        self.transform = transform;

        self
    }

    /// Stores a curve.
    ///
    /// ```
    /// canvas_builder
    ///     .curve(Curve::linear(l0, l1))
    ///     .curve(Curve::quadric(q0, q1, q2))
    ///     .curve(Curve::cubic(c0, c1, c2, c3));
    /// ```
    pub fn curve(&mut self, curve: Curve) -> &mut Self {
        self.curves.push(curve);

        self
    }

    /// Builds [`Canvas`] with stored lines.
    ///
    /// ```
    /// let canvas = canvas_builder.build();
    /// ```
    pub fn build(self) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);
        let mut lines = vec![];

        self.curves.into_iter().for_each(|mut curve| {
            curve.transform(&self.transform);
            curve.tesselate(&mut lines);
        });
        lines.iter().for_each(|line| canvas.draw_line(line));
        lines.sort_by(|left, right| left.p0().y.partial_cmp(&right.p0().y).unwrap());

        let mut hits_down = Vec::with_capacity(4);
        let mut hits_up = Vec::with_capacity(4);
        for scanline_y in 0..canvas.height {
            lines
                .iter()
                .filter(|line| line.p0().y <= scanline_y as f32)
                .filter(|line| line.p1().y > scanline_y as f32)
                .for_each(|line| {
                    // Find the intersection of line and scanline
                    let x =
                        (line.p0().x + line.dx() * (scanline_y as f32 - line.p0().y)) as usize + 1;

                    // Choose the vec to add the hit depending on the direction of a line.
                    let hits = match line.dir() {
                        Direction::Down => &mut hits_down,
                        Direction::Up => &mut hits_up,
                    };

                    let i = hits.partition_point(|hit| *hit < x);
                    hits.insert(i, x);
                });

            hits_down
                .drain(..)
                .zip(hits_up.drain(..))
                .map(|(x0, x1)| if x0 > x1 { (x1, x0) } else { (x0, x1) })
                .for_each(|(x0, x1)| {
                    for x in x0..x1 {
                        canvas.plot(x, scanline_y, 1.0);
                    }
                });
        }

        canvas
    }
}
