use std::mem;
use crate::{
    Line,
    Curve,
};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<f32>,
}

impl Canvas {
    fn draw_line(&mut self, line: &Line) {
        let mut p0 = line.p0;
        let mut p1 = line.p1;

        if (p1.x - p0.x).abs() >= (p1.y - p0.y).abs() {
            if p0.x == p1.x { return; }

            if p0.x > p1.x {
                mem::swap(&mut p0, &mut p1);
            }

            let mut y = p0.y;
            let k = (p1.y - p0.y) / (p1.x - p0.x);
            for x in p0.x as usize..self.width.min(p1.x.ceil() as usize) {
                let row = y as usize * self.width;

                self.bitmap[row + x] = 1.0 - (y - y.floor());
                self.bitmap[row + self.width + x] = y - y.floor();

                let dx = 1.0_f32.min(self.width as f32 - p1.x);

                y += k * dx;
            }
        } else if (p1.x - p0.x).abs() < (p1.y - p0.y).abs() {
            if p0.y == p1.y { return; }

            if p0.y > p1.y {
                mem::swap(&mut p0, &mut p1);
            }

            let mut x = p0.x;
            let k = (p1.x - p0.x) / (p1.y - p0.y);
            for y in p0.y as usize..self.height.min(p1.y.ceil() as usize) {
                let row = y * self.width;

                self.bitmap[row + x as usize] = 1.0 - (x - x.floor());
                self.bitmap[row + x as usize + 1] = x - x.floor();

                let dy = 1.0_f32.min(self.height as f32 - p1.y);

                x += k * dy
            }
        }
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

        self.lines.iter().for_each(|line| {
            canvas.draw_line(line);
        });

        self.lines.sort_by(|left, right|
            left.p0.y.partial_cmp(&right.p0.y).unwrap()
        );

        let mut hits = Vec::with_capacity(16);

        for scanline_y in 0..self.height {
            let scanline = &mut canvas.bitmap[
                            scanline_y * canvas.width..(scanline_y + 1) * canvas.width];

            for line in self.lines.iter() {
                if line.p0.y <= scanline_y as f32 {
                    if line.p1.y > scanline_y as f32 {
                        let k = (line.p1.x - line.p0.x) / (line.p1.y - line.p0.y);
                        let x = line.p0.x + k * (scanline_y as f32 - line.p0.y);

                        hits.push(x);
                    }
                }
            }

            hits.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for xs in hits.chunks_exact(2) {
                for x in xs[0] as usize + 1..xs[1] as usize + 1 {
                    scanline[x] = 1.0;
                }
            }

            hits.clear();
        }

        canvas
    }
}