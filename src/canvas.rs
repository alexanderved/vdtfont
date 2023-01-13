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
    fn draw_line(&mut self, line: Line) {
        if (line.p1.x - line.p0.x).abs() >= (line.p1.y - line.p0.y).abs() {
            self.interpolate();
        }
    }

    fn interpolate(&mut self) {

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

        /*let mut hits = vec![];

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
        }*/

        /*self.lines.msort_by(&|left, right| left.p0.x.partial_cmp(&right.p0.x).unwrap());

        for scanline_x in 0..self.width {
            for line in self.lines.iter() {
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




        for line in self.lines.iter_mut() {
            if line.p0.x == line.p1.x { continue; }

            let mut p0 = line.p0;
            let mut p1 = line.p1;
            if p0.x > p1.x {
                (p0, p1) = (p1, p0);
            }

            let mut y = p0.y;
            let k = (p1.y - p0.y) / (p1.x - p0.x);
            for x in p0.x as usize..canvas.width.min(p1.x.ceil() as usize) {
                let row = y as usize * canvas.width;

                canvas.bitmap[row + x] = 1.0 - (y - y.floor());
                canvas.bitmap[row + canvas.width + x] = y - y.floor();

                let dx = 1.0_f32.min(canvas.width as f32 - p1.x);

                y += k * dx;
            }
        }

        /*for line in self.lines.iter_mut() {
            if line.p0.y == line.p1.y { continue; }

            if line.p0 > line.p1 {
                (line.p0, line.p1) = (line.p1, line.p0);
            }

            let mut x = line.p0.x;
            let k = (line.p1.x - line.p0.x) / (line.p1.y - line.p0.y);
            for y in line.p0.y as usize..canvas.height.min(line.p1.y.ceil() as usize) {
                let row = y * canvas.width;

                canvas.bitmap[row + x as usize] = 1.0 - (x - x.floor());
                canvas.bitmap[row + x as usize + 1] = x - x.floor();



                let dy = 1.0_f32.min(canvas.height as f32 - line.p1.y);

                x += k * dy
            }
            
        }*/

        



        self.lines.sort_by(|left, right|
            left.p0.y.partial_cmp(&right.p0.y).unwrap()
        );

        let mut hits = vec![];

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
                for x in xs[0] as usize..xs[1] as usize + 1 {
                    scanline[x] = 1.0;
                }
            }

            hits.clear();
        }



        canvas
    }
}