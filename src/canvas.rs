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

        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            if p0.x == p1.x { return; }

            if p0.x > p1.x {
                mem::swap(&mut p0, &mut p1);
            }

            let x0 = p0.x.floor();
            let x1 = p1.x.ceil().min(self.width as f32);

            let k = (p1.y - p0.y) / (p1.x - p0.x);
            let mut y = p0.y + k * (x0 - p0.x);

            for x in x0 as usize..x1 as usize {
                self.plot(x, y as usize, 1.0 - (y - y.floor()));
                self.plot(x, (y as usize + 1).min(self.height - 1), y - y.floor());

                y += k;
            }
        } else if (p1.x - p0.x).abs() <= (p1.y - p0.y).abs() {
            if p0.y == p1.y { return; }

            if p0.y > p1.y {
                mem::swap(&mut p0, &mut p1);
            }

            let y0 = p0.y.floor();
            let y1 = p1.y.ceil().min(self.height as f32);

            let k = (p1.x - p0.x) / (p1.y - p0.y);
            let mut x = p0.x + k * (y0 - p0.y);

            for y in y0 as usize..y1 as usize {
                self.plot(x as usize, y, 1.0 - (x - x.floor()));
                self.plot((x as usize + 1).min(self.width - 1), y, x - x.floor());

                let dy = 1.0_f32.min(self.height as f32 - p1.y);

                x += k * dy
            }
        }
    }

    #[inline(always)]
    fn plot(&mut self, x: usize, y: usize, c: f32) {
        if y * self.width + x < self.bitmap.len() {
            self.bitmap[y * self.width + x] = c;
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

        self.lines.iter().for_each(|line| canvas.draw_line(line));

        self.lines.sort_by(|left, right|
            left.p0.y.partial_cmp(&right.p0.y).unwrap()
        );

        for scanline_y in 0..self.height {
            let mut hits = self.lines.iter()
                .filter_map(|line| {
                    if line.p0.y <= scanline_y as f32 && line.p1.y > scanline_y as f32 {
                        let k = (line.p1.x - line.p0.x) / (line.p1.y - line.p0.y);
                        let x = line.p0.x + k * (scanline_y as f32 - line.p0.y);

                        Some(x as usize + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            hits.sort_by(|a, b| a.cmp(b));

            for xs in hits.chunks_exact(2) {
                for x in xs[0]..xs[1] {
                    canvas.plot(x, scanline_y, 1.0);
                }
            }
        }

        canvas
    }
}
