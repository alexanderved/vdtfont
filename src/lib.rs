pub mod delaunay;
pub mod font;
pub mod opencl;
pub mod point;
pub mod voronoi;

pub extern crate arena_system;
pub extern crate ocl;

use std::{mem, time};

use point::Point;

fn bench(f: impl FnOnce() -> anyhow::Result<()>) -> time::Duration {
    let now = std::time::Instant::now();

    std::hint::black_box(f().unwrap());

    now.elapsed()
}

pub fn test(name: &str, f: impl FnOnce() -> anyhow::Result<()>) {
    let dur = crate::bench(f);
    println!("{}: {}μs, {}ms", name, dur.as_micros(), dur.as_millis());
}

pub fn profile(name: &str, event_list: ocl::EventList) -> anyhow::Result<()> {
    let mut elapsed = 0;

    event_list.wait_for()?;

    for event in event_list {
        let start = event.profiling_info(ocl::enums::ProfilingInfo::Start)?.time()?;
        let end = event.profiling_info(ocl::enums::ProfilingInfo::End)?.time()?;

        elapsed += end - start;
    }

    println!("{}: {}μs, {}ms", name, elapsed / 1000, elapsed / 1000000);

    Ok(())
}

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
