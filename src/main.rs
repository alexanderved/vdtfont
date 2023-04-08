use rand::Rng;

use vdtfont::font::*;
use vdtfont::point::Point;
use vdtfont::font::Font;

pub const IMG_DIM: usize = 2048;
pub const IMG_LEN: usize = IMG_DIM * IMG_DIM * 4;

#[allow(unused)]
fn generate_random_points(dim: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let len: usize = rng.gen_range(128..=128); //=dim.min(512));

    let res = (0..len)
        .into_iter()
        .map(|_| {
            let x = rng.gen_range(0..dim as i32) as f32;
            let y = rng.gen_range(0..dim as i32) as f32;

            let x_fract = rng.gen_range(0..100) as f32 / 100.0;
            let y_fract = rng.gen_range(0..100) as f32 / 100.0;

            Point::new(x + x_fract, y + y_fract)
        })
        .collect::<Vec<Point>>();

    res
}

fn save(glyph: &TriangulatedGlyph, name: &str) -> anyhow::Result<()> {
    let dim = glyph.dim();

    let glyph_img_data = glyph.image();

    image::save_buffer(name, &glyph_img_data, dim as u32, dim as u32, image::ColorType::Rgba8)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    #[rustfmt::skip]
    let font =
        include_bytes!("/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf");
        //include_bytes!(
        //    "/home/alex/projects/.deprecated/font_rasterizer/examples/fonts/DejaVuSansMono.ttf");

    let mut f = Font::from_vec(font.to_vec())?;

    for i in 0..26 {
        let c = char::from_u32('a' as u32 + i as u32).unwrap();
        println!("{}", c);

        let g = f.glyph(c);
        let og = f.outline_glyph(g);
        let tg = f.triangulate_glyph(og)?;

        save(&tg, "glyph.png")?;

        let mut t = "".to_string();
        let _ = std::io::stdin().read_line(&mut t);
        println!("{t}");
    }

    Ok(())
}
