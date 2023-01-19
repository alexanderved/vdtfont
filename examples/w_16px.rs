use font_rasterizer::*;

fn main() {
    let canvas = build_w_canvas();
    
    let data = canvas.iter()
        .flat_map(|a| [0, 0, 255, (255.0 * a.abs()) as u8])
        .collect::<Vec<u8>>();

    let _ = image::save_buffer(
        &std::path::Path::new("examples/images/w_16px.png"),
        &data,
        canvas.width() as u32,
        canvas.height() as u32,
        image::ColorType::Rgba8,
    );
}

fn build_w_canvas() -> Canvas {
    let canvas_builder = CanvasBuilder::new(10, 10)
        .add_curve(line(
            point(0.0, 0.48322153),
            point(1.2214766, 0.48322153)
        ))
        .add_curve(line(
            point(1.2214766, 0.48322153),
            point(2.5302014, 6.557047),
        ))
        .add_curve(line(
            point(2.5302014, 6.557047),
            point(3.6040268, 2.6778522),
        ))
        .add_curve(line(
            point(3.6040268, 2.6778522),
            point(4.657718, 2.6778522),
        ))
        .add_curve(line(
            point(4.657718, 2.6778522),
            point(5.7449665, 6.557047)
        ))
        .add_curve(line(
            point(5.7449665, 6.557047),
            point(7.0536914, 0.48322153),
        ))
        .add_curve(line(
            point(7.0536914, 0.48322153), point(8.275167, 0.48322153)))
        .add_curve(line(
            point(8.275167, 0.48322153), point(6.5167785, 8.0)))
        .add_curve(line(
            point(6.5167785, 8.0), point(5.3355703, 8.0)))
        .add_curve(line(
            point(5.3355703, 8.0), point(4.134228, 3.8791947)))
        .add_curve(line(
            point(4.134228, 3.8791947), point(2.9395974, 8.0)))
        .add_curve(line(
            point(2.9395974, 8.0), point(1.7583892, 8.0)))
        .add_curve(line(
            point(1.7583892, 8.0), point(0.0, 0.48322153)))
        .add_curve(line(
            point(0.0, 0.48322153), point(0.0, 0.48322153)));

    canvas_builder.build()
}