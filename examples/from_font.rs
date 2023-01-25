use font_rasterizer::*;
use owned_ttf_parser as ttfp;
use ttfp::AsFaceRef;

fn main() {
    let canvas = build_canvas_from_font();

    let data = canvas.iter()
        .flat_map(|a| [0, 0, 255, (255.0 * a.abs()) as u8])
        .collect::<Vec<u8>>();

    let _ = image::save_buffer(
        &std::path::Path::new("examples/images/from_font.png"),
        &data,
        canvas.width() as u32,
        canvas.height() as u32,
        image::ColorType::Rgba8,
    );
}

fn build_canvas_from_font() -> Canvas {
    let owned_font_data =
        include_bytes!("../../../../ab-glyph/dev/fonts/DejaVuSansMono.ttf");
        //include_bytes!("../.fonts/mingliu.ttc");

    let owned_face = ttfp::OwnedFace::from_vec(
        owned_font_data.to_vec(), 0).unwrap();
    let faster_face = ttfp::PreParsedSubtables::from(owned_face);

    let glyph_id = faster_face.glyph_index('☣').unwrap(); //☣ 一

    let mut outliner = Outliner::default();
    let rect = faster_face.as_face_ref()
        .outline_glyph(glyph_id, &mut outliner).unwrap();

    let mut canvas_builder = CanvasBuilder::new(
        (rect.width() + rect.x_min + 10) as usize,
        (rect.height() + rect.y_min + 10) as usize);

    let scale = 0.05;
    let scale_up = |p: Point| {
        point(p.x * scale, (rect.height() as f32 - p.y) * scale)
    };
    
    for linee in outliner.outline {
        canvas_builder = canvas_builder.add_curve(line(
            scale_up(linee.p0),
            scale_up(linee.p1)
        ));
    }

    canvas_builder.build()
}

#[derive(Default)]
struct Outliner {
    last: Point,
    last_move: Option<Point>,
    outline: Vec<Line>
}

impl ttfp::OutlineBuilder for Outliner {
    fn move_to(&mut self, x: f32, y: f32) {
        // eprintln!("M {x} {y}");
        self.last = point(x, y);
        self.last_move = Some(self.last);
    }

    fn line_to(&mut self, x1: f32, y1: f32) {
        // eprintln!("L {x1} {y1}");
        let p1 = point(x1, y1);
        self.outline.push(line(self.last, p1));
        self.last = p1;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        // eprintln!("Q {x1} {y1}");
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        quadric(self.last, p1, p2).to_lines_with_vec(&mut self.outline);
        self.last = p2;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        // eprintln!("C {x1} {y1} {x3} {y3}");
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        let p3 = point(x3, y3);

        cubic(self.last, p1, p2, p3).to_lines_with_vec(&mut self.outline);
        self.last = p3;
    }

    fn close(&mut self) {
        // eprintln!("Z");
        if let Some(m) = self.last_move.take() {
            self.outline.push(line(self.last, m));
        }
    }
}