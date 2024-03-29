use anyhow::Context;
use vdtfont::{Point, PointHandle, Font, TriangulatedGlyph};
use vdtfont::delaunay::{DelaunayTriangleHandle, Visibility};

use std::mem;

use arena_system::Handle;

pub fn plot(bitmap: &mut Vec<f32>, width: usize, height: usize, x: usize, y: usize, c: f32) {
    if x < width && y < height {
        unsafe {
            let pixel = bitmap.get_unchecked_mut(x + y * width);
            *pixel = c.max(*pixel);
        }
    }
}

pub fn draw_line(bitmap: &mut Vec<f32>, width: usize, height: usize, p0: Point, p1: Point) {
    let mut x0 = p0.x();
    let mut y0 = p0.y();
    let mut x1 = p1.x();
    let mut y1 = p1.y();

    let steep = (x1 - x0).abs() < (y1 - y0).abs();
    let delta = if steep { (x1 - x0) / (y1 - y0) } else { (y1 - y0) / (x1 - x0) };
    let boundary = if steep { height } else { width };

    if steep {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
    }

    if x0 == x1 {
        return;
    }

    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let i0 = x0.round();
    let i1 = x1.round();

    let mut prev_i = x0;
    let mut j = y0;

    let c = if p0.is_bounding() || p1.is_bounding() { 0.5 } else { 1.0 };

    for i in i0 as usize..boundary.min(i1 as usize + 1) {
        j += delta * (i as f32 - prev_i);

        if steep {
            plot(bitmap, width, height, j as usize, i, c);
        } else {
            plot(bitmap, width, height, i, j as usize, c);
        }

        prev_i = i as f32;
    }
}

#[allow(unused)]
pub fn rasterize_outline(glyph: &TriangulatedGlyph) -> Vec<u8> {
    let mut bitmap = vec![0.0; glyph.dim() * glyph.dim()];

    glyph
        .points()
        .handle_iter::<PointHandle>(None)
        .for_each(|p| {
            let pp = p.previous_in_outline();

            if p.exists() && pp.exists() {
                draw_line(
                    &mut bitmap,
                    glyph.dim(),
                    glyph.dim(),
                    (*p
                        .get()
                        .unwrap())
                        .clone(),
                    (*pp
                        .get()
                        .unwrap())
                        .clone(),
                );
            }
        });

    bitmap.into_iter().flat_map(|a| {
        if a > 0.0 {
            [255, 255, 255, (255.0 * a) as u8]
        } else {
            [0, 0, 0, 255]
        }
    }).collect()
}

pub fn rasterize_glyph(glyph: &TriangulatedGlyph) -> Vec<u8> {
    let mut bitmap = vec![0.0; glyph.dim() * glyph.dim()];

    glyph.triangles()
        .handle_iter::<DelaunayTriangleHandle>(&glyph.points())
        .for_each(|t| {
            if let Ok(t) = t.get() {
                if matches!(t.visibility(), Visibility::Visible) {
                    draw_line(
                        &mut bitmap,
                        glyph.dim(),
                        glyph.dim(),
                        (*glyph
                            .points()
                            .handle::<PointHandle>(t.vertices[0].into(), None)
                            .get()
                            .unwrap())
                            .clone(),
                        (*glyph
                            .points()
                            .handle::<PointHandle>(t.vertices[1].into(), None)
                            .get()
                            .unwrap())
                            .clone(),
                    );

                    draw_line(
                        &mut bitmap,
                        glyph.dim(),
                        glyph.dim(),
                        (*glyph
                            .points()
                            .handle::<PointHandle>(t.vertices[1].into(), None)
                            .get()
                            .unwrap())
                            .clone(),
                        (*glyph
                            .points()
                            .handle::<PointHandle>(t.vertices[2].into(), None)
                            .get()
                            .unwrap())
                            .clone(),
                    );

                    draw_line(
                        &mut bitmap,
                        glyph.dim(),
                        glyph.dim(),
                        (*glyph
                            .points()
                            .handle::<PointHandle>(t.vertices[0].into(), None)
                            .get()
                            .unwrap())
                            .clone(),
                        (*glyph
                            .points()
                            .handle::<PointHandle>(t.vertices[2].into(), None)
                            .get()
                            .unwrap())
                            .clone(),
                    );
                }
            }
        });

    bitmap.into_iter().flat_map(|a| {
        if a > 0.0 {
            [255, 255, 255, (255.0 * a) as u8]
        } else {
            [0, 0, 0, 255]
        }
    }).collect()
} 


fn save(glyph: &TriangulatedGlyph, name: &str) -> anyhow::Result<()> {
    let dim = glyph.dim();
    let glyph_img_data = rasterize_glyph(glyph);
    image::save_buffer(name, &glyph_img_data, dim as u32, dim as u32, image::ColorType::Rgba8)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let font_data = include_bytes!("/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf");

    let mut font = Font::from_vec(font_data.to_vec())?;

    let mut s = "".to_string();
    loop {
        println!("Enter any symbol (CTRL + C to exit):");
        std::io::stdin().read_line(&mut s)?;
        let c = s.chars().nth(0).context("No character was provided")?;

        let glyph = font.glyph(c);
        let outlined_glyph = font.outline_glyph(glyph);
        let triangulated_glyph = font.triangulate_glyph(outlined_glyph)?;

        save(&triangulated_glyph, &format!("{c}.png"))?;
        println!("The image of the symbol was saved in {c}.png");

        s.clear();
    }
}
