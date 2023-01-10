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
        self.lines.msort();

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
                    
                        hits.push(x as usize);
                    }
                }
            }

            hits.sort();

            for xs in hits.chunks_exact(2) {
                for x in xs[0]..xs[1] {
                    scanline[x + 1] = 1.0;
                }
            }

            hits.clear();
        }

        canvas
    }
}