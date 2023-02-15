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

        if (p1.x - p0.x).abs() >= (p1.y - p0.y).abs() {
            if p0.x == p1.x {
                return;
            }

            if p0.x > p1.x {
                mem::swap(&mut p0, &mut p1);
            }

            let x0 = p0.x.round();
            let x1 = p1.x.round();

            let mut prev_x = p0.x;
            let mut y = p0.y;

            for x in x0 as usize..x1 as usize + 1 {
                y += dy * (x as f32 - prev_x);

                self.plot(x, y as usize, y.rfract());
                self.plot(x, y as usize + 1, y.fract());

                prev_x = x as f32;
            }
        } else if (p1.x - p0.x).abs() < (p1.y - p0.y).abs() {
            if p0.y == p1.y {
                return;
            }

            if p0.y > p1.y {
                mem::swap(&mut p0, &mut p1);
            }

            let y0 = p0.y.round();
            let y1 = p1.y.round();

            let mut prev_y = p0.y;
            let mut x = p0.x;

            for y in y0 as usize..y1 as usize + 1 {
                x += dx * (y as f32 - prev_y);

                self.plot(x as usize, y, x.rfract());
                self.plot(x as usize + 1, y, x.fract());

                prev_y = y as f32;
            }
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

    lines: Vec<Line>,
    curves: Vec<Curve>,

    transform: fn(Point) -> Point,
}

impl CanvasBuilder {
    /// Creates new [`CanvasBuilder`] with specified width and height.
    ///
    /// ```
    /// let canvas_builder = CanvasBuilder::new(width, height);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            lines: vec![],
            curves: vec![],
            transform: |p| p,
        }
    }

    /// Sets `width` for [`Canvas`].
    ///
    /// ```
    /// canvas_builder.width(600);
    /// ```
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;

        self
    }

    /// Sets `height` for [`Canvas`].
    ///
    /// ```
    /// canvas_builder.height(800);
    /// ```
    pub const fn height(mut self, height: usize) -> Self {
        self.height = height;

        self
    }

    pub fn transform(mut self, transform: fn(Point) -> Point) -> Self {
        self.transform = transform;

        self
    }

    /// Tesselates a curve with lines and stores them.
    ///
    /// ```
    /// let canvas_builder = canvas_builder
    ///     .curve(Curve::linear(l0, l1))
    ///     .curve(Curve::quadric(q0, q1, q2))
    ///     .curve(Curve::cubic(c0, c1, c2, c3));
    /// ```
    pub fn curve(mut self, curve: Curve) -> Self {
        curve.tesselate(&mut self.lines);

        self
    }

    /// Stores `line` in [`CanvasBuilder`].
    ///
    /// ```
    /// let canvas_builder = canvas_builder.line(Line::new(l0, l1))
    /// ```
    pub fn line(mut self, line: Line) -> Self {
        self.lines.push(line);

        self
    }

    /// Builds [`Canvas`] with stored lines.
    ///
    /// ```
    /// let canvas = canvas_builder.build();
    /// ```
    pub fn build(mut self) -> Canvas {
        let mut canvas = Canvas {
            width: self.width,
            height: self.height,
            bitmap: vec![0.0; self.width * self.height],
        };

        self.curves
            .into_iter()
            .for_each(|curve| curve.tesselate(&mut self.lines));
        self.lines.iter().for_each(|line| canvas.draw_line(line));
        self.lines
            .sort_by(|left, right| left.p0().y.partial_cmp(&right.p0().y).unwrap());

        let mut hits: Vec<(usize, i8)> = Vec::with_capacity(8);
        for scanline_y in 0..canvas.height {
            self.lines
                .iter()
                .filter_map(|line| {
                    // Find the intersection of line and scanline
                    if line.p0().y <= scanline_y as f32 && line.p1().y > scanline_y as f32 {
                        let x = line.p0().x + line.dx() * (scanline_y as f32 - line.p0().y);

                        Some((x as usize + 1, line.dir()))
                    } else {
                        None
                    }
                })
                .for_each(|hit| {
                    let i = hits.partition_point(|(x, _)| *x < hit.0);

                    hits.insert(i, hit);
                });

            hits.drain(..)
                .fold((0, 0), |(mut start, mut w), (hit, dir)| {
                    // If `w` equals 0, we have filled all previous intervals.
                    // Now we need to find starting point of the new interval.
                    if w == 0 {
                        start = hit;
                    }

                    w += dir;

                    // If `w` equals 0 after adding dir, we have reached the end point
                    // of the current interval. Now we need to fill it.
                    if w == 0 {
                        for x in start..hit {
                            canvas.plot(x, scanline_y, 1.0);
                        }
                    }

                    (start, w)
                });
        }

        canvas
    }
}
