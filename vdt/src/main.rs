use rand::Rng;

use vdt::delaunay::{Delaunay, DelaunayFactory};
//use vdt::point::Point;
use vdt::voronoi::{Site, VoronoiImage, VoronoiImageFactory};

pub const IMG_DIM: usize = 2048;
pub const IMG_LEN: usize = IMG_DIM * IMG_DIM * 4;

fn generate_random_points(dim: usize) -> Vec<Site> {
    let mut rng = rand::thread_rng();
    let len: usize = rng.gen_range(30..=30); //=dim.min(512));

    let res = (0..len)
        .into_iter()
        .map(|_| {
            let x = rng.gen_range(0..dim as i32) as f32;
            let y = rng.gen_range(0..dim as i32) as f32;

            let x_fract = rng.gen_range(0..100) as f32 / 100.0;
            let y_fract = rng.gen_range(0..100) as f32 / 100.0;

            Site::new(x + x_fract, y + y_fract)
        })
        .collect::<Vec<Site>>();

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
    let random = generate_random_points(dim);

    //let now = std::time::Instant::now();

    let mut voronoi_image_factory = VoronoiImageFactory::new(queue.clone(), IMG_DIM)?;
    let mut delaunay_factory = DelaunayFactory::new(queue.clone())?;

    let now = std::time::Instant::now();

    let voronoi_image = voronoi_image_factory.construct_borrowed(random.clone(), dim)?;
    let delaunay = delaunay_factory.construct(&voronoi_image)?;

    let dur = now.elapsed();
    println!("Overall time: {}μs, {}ms", dur.as_micros(), dur.as_millis());

    save(&voronoi_image, &delaunay, "voronoi.png")?;

    Ok(())
}
