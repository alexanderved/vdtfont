/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    mem, slice, vec,
    iter::IntoIterator,
};
use crate::{
    Line,
    Curve,
    math::*,
};

pub struct Canvas {
    width: usize,
    height: usize,
    bitmap: Vec<f32>,
}

impl Canvas {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn draw_line(&mut self, line: &Line) {
        let mut p0 = line.p0;
        let mut p1 = line.p1;

        if (p1.x - p0.x).abs() >= (p1.y - p0.y).abs() {
            if p0.x == p1.x { return; }

            if p0.x > p1.x {
                mem::swap(&mut p0, &mut p1);
            }

            let x0 = p0.x.round();
            let x1 = p1.x.round().min(self.width as f32);
            let mut y = p0.y + line.dy * (x0 - p0.x);

            for x in x0 as usize..x1 as usize + 1 {
                self.plot(x, y as usize, y.rfract());
                self.plot(x, y as usize + 1, y.fract());

                let dx = 1.0_f32.min(self.width as f32 - p1.x);

                y += line.dy * dx;
            }
        } else if (p1.x - p0.x).abs() < (p1.y - p0.y).abs() {
            if p0.y == p1.y { return; }

            if p0.y > p1.y {
                mem::swap(&mut p0, &mut p1);
            }

            let y0 = p0.y.round();
            let y1 = p1.y.round().min(self.height as f32);
            let mut x = p0.x + line.dx * (y0 - p0.y);

            for y in y0 as usize..y1 as usize + 1 {
                self.plot(x as usize, y, x.rfract());
                self.plot(x as usize + 1, y, x.fract());

                let dy = 1.0_f32.min(self.height as f32 - p1.y);

                x += line.dx * dy
            }
        }
    }

    #[inline]
    pub fn plot(&mut self, x: usize, y: usize, c: f32) {
        if y * self.width + x < self.bitmap.len() {
            self.bitmap[y * self.width + x] = c.max(self.bitmap[y * self.width + x]);
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'_ f32> {
        self.bitmap.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut f32> {
        self.bitmap.iter_mut()
    }
}

impl IntoIterator for Canvas {
    type Item = f32;
    type IntoIter = vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.bitmap.into_iter()
    }
}

impl<'a> IntoIterator for &'a Canvas {
    type Item = &'a f32;
    type IntoIter = slice::Iter<'a, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.bitmap.iter()
    }
}

impl<'a> IntoIterator for &'a mut Canvas {
    type Item = &'a mut f32;
    type IntoIter = slice::IterMut<'a, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.bitmap.iter_mut()
    }
}

pub struct CanvasBuilder {
    width: usize,
    height: usize,
    pub lines: Vec<Line>,
}

impl CanvasBuilder {
    pub fn new(width: usize, height: usize) -> CanvasBuilder {
        CanvasBuilder {
            width,
            height,
            lines: Vec::new()
        }
    }

    pub fn add_curve(mut self, curve: impl Curve) -> CanvasBuilder {
        curve.tesselate(&mut self.lines);

        self
    }

    pub fn build(mut self) -> Canvas {
        let mut canvas = Canvas {
            width: self.width,
            height: self.height,
            bitmap: vec![0.0; self.width * self.height]
        };

        self.lines.iter().for_each(|line| canvas.draw_line(line));

        self.lines.sort_by(|left, right| {
            left.p0.y.partial_cmp(&right.p0.y).unwrap()
        });

        let mut hits: Vec<(usize, i8)> = Vec::with_capacity(8);
        for scanline_y in 0..canvas.height {
            self.lines.iter()
                .filter_map(|line| {
                    if line.p0.y <= scanline_y as f32 && line.p1.y > scanline_y as f32 {
                        let x = line.p0.x + line.dx * (scanline_y as f32 - line.p0.y);

                        Some((x as usize + 1, line.dir))
                    } else {
                        None
                    }
                })
                .for_each(|hit| hits.push(hit));
            hits.sort_by(|a, b| a.0.cmp(&b.0));

            hits.drain(..)
                .fold((0, 0), |(mut x0, mut w), (hit, dir)| {
                    if w == 0 {
                        x0 = hit;
                    }
                    w += dir;
                    if w == 0 {
                        for x in x0..hit {
                            canvas.plot(x, scanline_y, 1.0);
                        }
                    }

                    (x0, w)
                });
        }

        canvas
    }
}