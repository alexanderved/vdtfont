use crate::{
    LinkedList,
    Line,
    Curve,
};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<f32>,
}

pub struct CanvasBuilder {
    width: usize,
    height: usize,
    pub lines: LinkedList<Line>,
}

impl CanvasBuilder {
    pub fn new(width: usize, height: usize,) -> CanvasBuilder {
        CanvasBuilder {
            width,
            height,
            lines: LinkedList::new()
        }
    }

    pub fn add_curve(mut self, curve: impl Curve) -> CanvasBuilder {
        self.lines.merge(curve.to_lines());

        self
    }

    pub fn build(mut self) -> Canvas {
        self.lines.msort_by(&|left, right| left.p0.y.partial_cmp(&right.p0.y).unwrap());

        let mut canvas = Canvas {
            width: self.width,
            height: self.height,
            bitmap: vec![0.0; self.width * self.height]
        };

        let mut hits = vec![];

        for scanline_y in 0..self.height {
            let scanline = &mut canvas.bitmap[
                            scanline_y * canvas.width..(scanline_y + 1) * canvas.width];

            for line in self.lines.iter() {
                if line.p0.y <= scanline_y as f32 + 0.5 {
                    if line.p1.y > scanline_y as f32 + 0.5 {
                        let k = (line.p1.x - line.p0.x) / (line.p1.y - line.p0.y);
                        let x = line.p0.x + k * (scanline_y as f32 + 0.5 - line.p0.y);

                        scanline[x as usize] = 1.0 - (x - x.floor());
                        if (x as usize + 1) < canvas.width {
                            scanline[x as usize + 1] = x - x.floor();
                        }
                    
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

        /*self.lines.msort_by(&|left, right| left.p0.x.partial_cmp(&right.p0.x).unwrap());

        for scanline_x in 0..self.width {
            for line in self.lines.iter() {
                println!("{line:?}");
                if line.p0.x <= scanline_x as f32 + 0.5 {
                    if line.p1.x > scanline_x as f32 + 0.5 {
                        let k = (line.p1.y - line.p0.y) / (line.p1.x - line.p0.x);
                        let y = line.p0.y + k * (scanline_x as f32 + 0.5 - line.p0.x);

                        canvas.bitmap[y as usize * canvas.width + scanline_x] =
                            1.0 - (y - y.floor());
                    }
                }
            }
        }*/

        canvas
    }
}