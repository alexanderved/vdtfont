#[allow(unused)]
use arena_system::Arena;

use owned_ttf_parser::{self as ttfp, AsFaceRef};
use rand::Rng;

use vdtfont::delaunay::{Delaunay, DelaunayFactory};
use vdtfont::font::*;
use vdtfont::point::{Point, PointHandle};
use vdtfont::voronoi::{VoronoiImage, VoronoiImageFactory};

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

fn save(voronoi_image: &VoronoiImage, delaunay: &Delaunay, name: &str) -> anyhow::Result<()> {
    let image = voronoi_image.image();
    let dim = voronoi_image.dim();
    let len = dim * dim * 4;

    let mut img_data = vec![0; len];
    image.read(&mut img_data)?;

    let img_data = img_data
        .chunks(4)
        .flat_map(|p| {
            let mut res = [255u8; 4];

            res[0] = (p[0] as f32 * 255.0 / dim as f32).min(255.0) as u8;
            res[1] = (p[1] as f32 * 255.0 / dim as f32).min(255.0) as u8;
            res[2] = (p[2] as f32 * 255.0 / dim as f32).min(255.0) as u8;

            res
        })
        .collect::<Vec<u8>>();

    let delaunay_img_data = delaunay.image();
    let img_data = img_data
        .chunks(4)
        .zip(delaunay_img_data.chunks(4))
        .flat_map(|(v, d)| if d[3] > 0 { [0, 0, 0, d[3]] } else { [v[0], v[1], v[2], v[3]] })
        .collect::<Vec<u8>>();

    image::save_buffer(name, &img_data, dim as u32, dim as u32, image::ColorType::Rgba8)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let platform = ocl::Platform::default();
    let device = ocl::Device::list(platform, Some(ocl::DeviceType::GPU))?[0];
    let context = ocl::Context::builder().platform(platform).devices(device).build()?;
    let queue =
        ocl::Queue::new(&context, device, Some(ocl::CommandQueueProperties::PROFILING_ENABLE))?;

    let dim = IMG_DIM / 2;

    let font = include_bytes!("../../../.deprecated/font_rasterizer/.fonts/times.ttf");

    let owned_face = ttfp::OwnedFace::from_vec(font.to_vec(), 0).unwrap();
    let parsed_face = ttfp::PreParsedSubtables::from(owned_face);

    let glyph_id = parsed_face.glyph_index('r').unwrap();

    let mut outliner = outliner::Outliner::new();
    let rect = parsed_face.as_face_ref().outline_glyph(glyph_id, &mut outliner).unwrap();

    let height: f32 =
        (parsed_face.as_face_ref().ascender() - parsed_face.as_face_ref().descender()).into();
    let h_factor = dim as f32 / height;
    let v_factor = dim as f32 / height;

    let bounds = ttfp::Rect {
        x_min: (rect.x_min as f32 * h_factor) as i16,
        x_max: (rect.x_max as f32 * h_factor) as i16,
        y_min: (rect.y_min as f32 * v_factor) as i16,
        y_max: (rect.y_max as f32 * v_factor) as i16,
    };

    println!("The number of points: {}", outliner.points.len());
    println!("The shortest distance: {}", outliner.shortest_distance * h_factor);
    println!("The height of a glyph: {}", height);

    (0..outliner.points.len()).into_iter().for_each(|i| {
        let p = outliner.points.handle::<PointHandle>(i.into(), None);
        let new_x = p.x() * h_factor;
        let new_y = bounds.height() as f32 - p.y() * v_factor;

        let mut point = outliner.points.handle::<PointHandle>(i.into(), None);
        point.set_coords(ocl::prm::Float2::new(new_x, new_y));
    });

    let _random = generate_random_points(dim).into_iter().collect::<Arena<Point>>();

    //let now = std::time::Instant::now();

    let now = std::time::Instant::now();

    let mut voronoi_image_factory = VoronoiImageFactory::new(queue.clone(), IMG_DIM)?;
    let mut delaunay_factory = DelaunayFactory::new(queue.clone())?;

    let voronoi_image = voronoi_image_factory.construct_borrowed(outliner.points, dim)?;
    let mut delaunay = delaunay_factory.construct(&voronoi_image)?;

    delaunay.insert_edge([0, 100]);

    let dur = now.elapsed();
    println!("Overall time: {}μs, {}ms", dur.as_micros(), dur.as_millis());

    save(&voronoi_image, &delaunay, "voronoi.png")?;

    Ok(())
}
