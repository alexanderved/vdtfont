use font_rasterizer::*;

fn main() {
    let canvas = build_iota_canvas();
    let mut data = vec![];

    for i in 0..canvas.width * canvas.height {
        let a = canvas.bitmap[i];

        data.push(0);
        data.push(0);
        data.push(255);
        data.push((255.0 * a.abs()) as u8);
    }

    let _ = image::save_buffer(
        &std::path::Path::new("examples/images/iota_60px.png"),
        &data,
        canvas.width as u32,
        canvas.height as u32,
        image::ColorType::Rgba8,
    );
}

fn build_iota_canvas() -> Canvas {
    let canvas_builder = CanvasBuilder::new(15, 39)
        .add_curve(quadric(
            point(6.1964865, 34.482967),
            point(7.2291145, 34.482967),
            point(9.186806, 33.92363),
        ))
        .add_curve(line(point(9.186806, 33.92363), point(9.186806, 36.69882)))
        .add_curve(quadric(
            point(9.186806, 36.69882),
            point(8.5199, 36.978485),
            point(7.455002, 37.204376),
        ))
        .add_curve(quadric(
            point(7.455002, 37.204376),
            point(6.3901043, 37.430264),
            point(5.572607, 37.430264),
        ))
        .add_curve(quadric(
            point(5.572607, 37.430264),
            point(2.9695234, 37.430264),
            point(1.6034422, 36.150234),
        ))
        .add_curve(quadric(
            point(1.6034422, 36.150234),
            point(0.2373612, 34.870205),
            point(0.2373612, 32.33166),
        ))
        .add_curve(quadric(
            point(0.2373612, 32.33166),
            point(0.2373612, 31.621727),
            point(0.4417355, 30.470778),
        ))
        .add_curve(quadric(
            point(0.4417355, 30.470778),
            point(0.6461098, 29.319828),
            point(4.0021515, 13.421656),
        ))
        .add_curve(line(
            point(4.0021515, 13.421656),
            point(7.594837, 13.421656),
        ))
        .add_curve(line(point(7.594837, 13.421656), point(4.2603087, 29.25529)))
        .add_curve(quadric(
            point(4.2603087, 29.25529),
            point(3.894586, 30.890284),
            point(3.894586, 31.92291),
        ))
        .add_curve(quadric(
            point(3.894586, 31.92291),
            point(3.894586, 34.482967),
            point(6.1964865, 34.482967),
        ))
        .add_curve(line(
            point(6.1964865, 34.482967),
            point(6.1964865, 34.482967),
        ))
        .add_curve(quadric(
            point(5.615633, 6.645033),
            point(7.89602, 2.6220856),
            point(8.993188, 0.08354187),
        ))
        .add_curve(line(
            point(8.993188, 0.08354187),
            point(13.18824, 0.08354187),
        ))
        .add_curve(line(
            point(13.18824, 0.08354187),
            point(13.18824, 0.5138016),
        ))
        .add_curve(quadric(
            point(13.18824, 0.5138016),
            point(12.112585, 1.9982071),
            point(10.456078, 3.837574),
        ))
        .add_curve(quadric(
            point(10.456078, 3.837574),
            point(8.79957, 5.6769447),
            point(7.2291145, 7.1183205),
        ))
        .add_curve(line(
            point(7.2291145, 7.1183205),
            point(5.615633, 7.1183205),
        ))
        .add_curve(line(point(5.615633, 7.1183205), point(5.615633, 6.645033)))
        .add_curve(line(point(5.615633, 6.645033), point(5.615633, 6.645033)))
        .add_curve(quadric(
            point(0.94729304, 7.354965),
            point(0.94729304, 6.300823),
            point(1.560416, 5.612406),
        ))
        .add_curve(quadric(
            point(1.560416, 5.612406),
            point(2.1735392, 4.9239845),
            point(3.1416278, 4.9239845),
        ))
        .add_curve(quadric(
            point(3.1416278, 4.9239845),
            point(4.8196487, 4.9239845),
            point(4.8196487, 6.645033),
        ))
        .add_curve(quadric(
            point(4.8196487, 6.645033),
            point(4.8196487, 7.720688),
            point(4.1635, 8.441376),
        ))
        .add_curve(quadric(
            point(4.1635, 8.441376),
            point(3.5073504, 9.162064),
            point(2.6898532, 9.162064),
        ))
        .add_curve(quadric(
            point(2.6898532, 9.162064),
            point(1.9368951, 9.162064),
            point(1.4420941, 8.688776),
        ))
        .add_curve(quadric(
            point(1.4420941, 8.688776),
            point(0.94729304, 8.215488),
            point(0.94729304, 7.354965),
        ))
        .add_curve(line(
            point(0.94729304, 7.354965),
            point(0.94729304, 7.354965),
        ))
        .add_curve(quadric(
            point(9.617067, 7.354965),
            point(9.617067, 6.365364),
            point(10.219434, 5.6446743),
        ))
        .add_curve(quadric(
            point(10.219434, 5.6446743),
            point(10.8218, 4.9239845),
            point(11.811402, 4.9239845),
        ))
        .add_curve(quadric(
            point(11.811402, 4.9239845),
            point(13.489423, 4.9239845),
            point(13.489423, 6.645033),
        ))
        .add_curve(quadric(
            point(13.489423, 6.645033),
            point(13.489423, 7.699175),
            point(12.854787, 8.43062),
        ))
        .add_curve(quadric(
            point(12.854787, 8.43062),
            point(12.220151, 9.162064),
            point(11.381141, 9.162064),
        ))
        .add_curve(quadric(
            point(11.381141, 9.162064),
            point(10.628182, 9.162064),
            point(10.122625, 8.688776),
        ))
        .add_curve(quadric(
            point(10.122625, 8.688776),
            point(9.617067, 8.215488),
            point(9.617067, 7.354965),
        ))
        .add_curve(line(point(9.617067, 7.354965), point(9.617067, 7.354965)));

    canvas_builder.build()
}