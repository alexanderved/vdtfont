use font_rasterizer::*;

fn main() {
    let canvas = build_e_canvas();
    let mut data = vec![];

    for i in 0..canvas.width * canvas.height {
        let a = canvas.bitmap[i];

        data.push(0);
        data.push(0);
        data.push(255);
        data.push((255.0 * a.abs()) as u8);
    }

    let _ = image::save_buffer(
        &std::path::Path::new("examples/images/e_300px.png"),
        &data,
        canvas.width as u32,
        canvas.height as u32,
        image::ColorType::Rgba8,
    );
}

fn build_e_canvas() -> Canvas {
    let canvas_builder = CanvasBuilder::new(106, 183)
        .add_curve(cubic(
            point(103.0, 163.5),
            point(86.25, 169.25),
            point(77.0, 165.0),
            point(82.25, 151.5),
        ))
        .add_curve(cubic(
            point(82.25, 151.5),
            point(86.75, 139.75),
            point(94.0, 130.75),
            point(102.0, 122.0),
        ))
        .add_curve(line(point(102.0, 122.0), point(100.25, 111.25)))
        .add_curve(cubic(
            point(100.25, 111.25),
            point(89.0, 112.75),
            point(72.75, 114.25),
            point(58.5, 114.25),
        ))
        .add_curve(cubic(
            point(58.5, 114.25),
            point(30.75, 114.25),
            point(18.5, 105.25),
            point(16.75, 72.25),
        ))
        .add_curve(line(point(16.75, 72.25), point(77.0, 72.25)))
        .add_curve(cubic(
            point(77.0, 72.25),
            point(97.0, 72.25),
            point(105.25, 60.25),
            point(104.75, 38.5),
        ))
        .add_curve(cubic(
            point(104.75, 38.5),
            point(104.5, 13.5),
            point(89.0, 0.75),
            point(54.25, 0.75),
        ))
        .add_curve(cubic(
            point(54.25, 0.75),
            point(16.0, 0.75),
            point(0.0, 16.75),
            point(0.0, 64.0),
        ))
        .add_curve(cubic(
            point(0.0, 64.0),
            point(0.0, 110.5),
            point(16.0, 128.0),
            point(56.5, 128.0),
        ))
        .add_curve(cubic(
            point(56.5, 128.0),
            point(66.0, 128.0),
            point(79.5, 127.0),
            point(90.0, 125.0),
        ))
        .add_curve(cubic(
            point(90.0, 125.0),
            point(78.75, 135.25),
            point(73.25, 144.5),
            point(70.75, 152.0),
        ))
        .add_curve(cubic(
            point(70.75, 152.0),
            point(64.5, 169.0),
            point(75.5, 183.0),
            point(105.0, 170.5),
        ))
        .add_curve(line(point(105.0, 170.5), point(103.0, 163.5)))
        .add_curve(cubic(
            point(55.0, 14.5),
            point(78.5, 14.5),
            point(88.5, 21.75),
            point(88.75, 38.75),
        ))
        .add_curve(cubic(
            point(88.75, 38.75),
            point(89.0, 50.75),
            point(85.75, 59.75),
            point(73.5, 59.75),
        ))
        .add_curve(line(point(73.5, 59.75), point(16.5, 59.75)))
        .add_curve(cubic(
            point(16.5, 59.75),
            point(17.25, 25.5),
            point(27.0, 14.5),
            point(55.0, 14.5),
        ))
        .add_curve(line(point(55.0, 14.5), point(55.0, 14.5)));

    canvas_builder.build()
}
