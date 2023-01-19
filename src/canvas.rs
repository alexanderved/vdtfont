use std::{
    mem, slice, vec,
    iter::IntoIterator,
};
use crate::{
    Line,
    Curve,
};

pub struct Canvas {
    width: usize,
    height: usize,
    bitmap: Vec<f32>,
}

impl Canvas {
    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn draw_line(&mut self, line: &Line) {
        let mut p0 = line.p0;
        let mut p1 = line.p1;

        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            if p0.x == p1.x { return; }

            if p0.x > p1.x {
                mem::swap(&mut p0, &mut p1);
            }

            let x0 = p0.x.round();
            let x1 = p1.x.round().min(self.width as f32);
            let mut y = p0.y + line.dy * (x0 - p0.x);

            for x in x0 as usize..x1 as usize + 1 {
                self.plot(x, y as usize, 1.0 - (y - y.floor()));
                self.plot(x, (y as usize + 1).min(self.height - 1), y - y.floor());

                let dx = 1.0_f32.min(self.width as f32 - p1.x);

                y += line.dy * dx;
            }
        } else if (p1.x - p0.x).abs() <= (p1.y - p0.y).abs() {
            if p0.y == p1.y { return; }

            if p0.y > p1.y {
                mem::swap(&mut p0, &mut p1);
            }

            let y0 = p0.y.round();
            let y1 = p1.y.round().min(self.height as f32);
            let mut x = p0.x + line.dx * (y0 - p0.y);

            for y in y0 as usize..y1 as usize + 1 {
                self.plot(x as usize, y, 1.0 - (x - x.floor()));
                self.plot((x as usize + 1).min(self.width - 1), y, x - x.floor());

                let dy = 1.0_f32.min(self.height as f32 - p1.y);

                x += line.dx * dy
            }
        }
    }

    #[inline(always)]
    pub fn plot(&mut self, x: usize, y: usize, c: f32) {
        if y * self.width + x < self.bitmap.len() {
            self.bitmap[y * self.width + x] = c;
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &'_ f32> {
        self.bitmap.iter()
    }

    #[inline(always)]
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
    pub fn new(width: usize, height: usize,) -> CanvasBuilder {
        CanvasBuilder {
            width,
            height,
            lines: Vec::new()
        }
    }

    pub fn add_curve(mut self, curve: impl Curve) -> CanvasBuilder {
        curve.to_lines_with_vec(&mut self.lines);

        self
    }

    pub fn build(mut self) -> Canvas {
        let mut canvas = Canvas {
            width: self.width,
            height: self.height,
            bitmap: vec![0.0; self.width * self.height]
        };

        self.lines.iter().for_each(|line| canvas.draw_line(line));

        self.lines.sort_by(|left, right|
            left.p0.y.partial_cmp(&right.p0.y).unwrap()
        );

        for scanline_y in 0..self.height {
            let mut hits = self.lines.iter()
                .filter_map(|line| {
                    if line.p0.y <= scanline_y as f32 && line.p1.y > scanline_y as f32 {
                        let x = line.p0.x + line.dx * (scanline_y as f32 - line.p0.y);

                        Some(x as usize + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            hits.sort_by(|a, b| a.cmp(b));

            hits.chunks_exact(2)
                .flat_map(|xs| xs[0]..xs[1])
                .for_each(|x| canvas.plot(x, scanline_y, 1.0));
        }

        canvas
    }
}