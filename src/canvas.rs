/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{math::*, Curve, Line};
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
    #[inline]
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
}

impl CanvasBuilder {
    /// Creates new [`CanvasBuilder`] with specified width and height.
    /// 
    /// ```
    /// let canvas_builder = CanvasBuilder::new(width, height);
    /// ```
    pub const fn new() -> CanvasBuilder {
        CanvasBuilder {
            width: 0,
            height: 0,
            lines: Vec::new(),
        }
    }

    /// Sets `width` for [`Canvas`].
    ///
    /// ```
    /// canvas_builder.width(600);
    /// ```
    pub const fn width(mut self, width: usize) -> CanvasBuilder {
        self.width = width;

        self
    }

    /// Sets `height` for [`Canvas`].
    ///
    /// ```
    /// canvas_builder.height(800);
    /// ```
    pub const fn height(mut self, height: usize) -> CanvasBuilder {
        self.height = height;

        self
    }

    /// Tesselates a curve with lines and stores them.
    /// 
    /// ```
    /// let canvas_builder = canvas_builder
    ///     .curve(line(l0, l1))
    ///     .curve(quadric(q0, q1, q2))
    ///     .curve(cubic(c0, c1, c2, c3));
    /// ```
    pub fn curve(mut self, curve: impl Curve) -> CanvasBuilder {
        curve.tesselate(&mut self.lines);

        self
    }

    /// Stores `line` in [`CanvasBuilder`].
    /// 
    /// ```
    /// let canvas_builder = canvas_builder.line(Line::new(l0, l1))
    /// ```
    pub fn line(mut self, line: Line) -> CanvasBuilder {
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

        self.lines.iter().for_each(|line| canvas.draw_line(line));
        self.lines
            .sort_by(|left, right| left.p0().y.partial_cmp(&right.p0().y).unwrap());

        let mut hits: Vec<(usize, i8)> = Vec::with_capacity(8);
        for scanline_y in 0..canvas.height {
            self.lines
                .iter()
                .filter_map(|line| { // Find the intersection of line and scanline
                    if line.p0().y <= scanline_y as f32 && line.p1().y > scanline_y as f32 {
                        let x = line.p0().x + line.dx() * (scanline_y as f32 - line.p0().y);

                        Some((x as usize + 1, line.dir()))
                    } else {
                        None
                    }
                })
                .for_each(|hit| {
                    let i = if !hits.is_empty() {
                        // Using binary search to find place to insert element,
                        // where the previous one is less and the next one is greater.
                        let mut low = 0;
                        let mut high = hits.len() - 1;
                        loop {
                            if high <= low {
                                break if hit.0 > hits[low].0 {
                                    low + 1
                                } else {
                                    low
                                };
                            }

                            let mid = (low + high) / 2;
                            // SAFETY: 0 >= `low` >= `mid` and `mid` <= `high` < hits.len(),
                            // so it's safe.
                            let mid_hit = unsafe { hits.get_unchecked(mid) };

                            if hit.0 == mid_hit.0 {
                                break mid + 1;
                            } else if hit.0 > mid_hit.0 {
                                low = mid.saturating_add(1);
                            } else if hit.0 < mid_hit.0 {
                                high = mid.saturating_sub(1);
                            }
                        }
                    } else {
                        // If hits are empty, insert element at the begining.
                        0
                    };

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
