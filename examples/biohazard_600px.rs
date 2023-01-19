use font_rasterizer::*;

fn main() {
    let canvas = build_biohazard_canvas();

    let data = canvas.iter()
        .flat_map(|a| [0, 0, 255, (255.0 * a.abs()) as u8])
        .collect::<Vec<u8>>();

    let _ = image::save_buffer(
        &std::path::Path::new("examples/images/biohazard_600px.png"),
        &data,
        canvas.width() as u32,
        canvas.height() as u32,
        image::ColorType::Rgba8,
    );
}

fn build_biohazard_canvas() -> Canvas {
    let canvas_builder = CanvasBuilder::new(297, 272)
        .add_curve(quadric(
            point(176.22818, 196.51007),
            point(188.30872, 189.21141),
            point(195.22986, 175.74664),
        ))
        .add_curve(quadric(
            point(195.22986, 175.74664),
            point(202.151, 162.28189),
            point(200.89261, 148.4396),
        ))
        .add_curve(quadric(
            point(200.89261, 148.4396),
            point(207.43623, 145.92282),
            point(213.97986, 144.9161),
        ))
        .add_curve(quadric(
            point(213.97986, 144.9161),
            point(216.24495, 165.5537),
            point(204.41609, 184.55537),
        ))
        .add_curve(quadric(
            point(204.41609, 184.55537),
            point(192.58723, 203.55705),
            point(177.48657, 210.10068),
        ))
        .add_curve(quadric(
            point(177.48657, 210.10068),
            point(174.46643, 203.30537),
            point(176.22818, 196.51007),
        ))
        .add_curve(line(
            point(176.22818, 196.51007),
            point(176.22818, 196.51007),
        ))
        .add_curve(quadric(
            point(173.20804, 104.89934),
            point(160.87582, 98.104034),
            point(145.77516, 98.73323),
        ))
        .add_curve(quadric(
            point(145.77516, 98.73323),
            point(130.67448, 99.36243),
            point(119.60066, 107.164444),
        ))
        .add_curve(quadric(
            point(119.60066, 107.164444),
            point(113.81207, 103.13759),
            point(110.03691, 98.104034),
        ))
        .add_curve(quadric(
            point(110.03691, 98.104034),
            point(126.647644, 85.52014),
            point(148.92113, 86.40102),
        ))
        .add_curve(quadric(
            point(148.92113, 86.40102),
            point(171.19463, 87.28189),
            point(184.53355, 97.09732),
        ))
        .add_curve(quadric(
            point(184.53355, 97.09732),
            point(180.00334, 102.88591),
            point(173.20804, 104.89934),
        ))
        .add_curve(line(
            point(173.20804, 104.89934),
            point(173.20804, 104.89934),
        ))
        .add_curve(quadric(
            point(95.69127, 152.71812),
            point(95.69127, 166.56041),
            point(103.744965, 179.39598),
        ))
        .add_curve(quadric(
            point(103.744965, 179.39598),
            point(111.79865, 192.23154),
            point(124.38254, 198.02014),
        ))
        .add_curve(quadric(
            point(124.38254, 198.02014),
            point(123.879196, 204.81543),
            point(121.11073, 210.85571),
        ))
        .add_curve(quadric(
            point(121.11073, 210.85571),
            point(102.48657, 202.55034),
            point(91.66443, 182.79362),
        ))
        .add_curve(quadric(
            point(91.66443, 182.79362),
            point(80.84228, 163.03693),
            point(82.8557, 146.67786),
        ))
        .add_curve(quadric(
            point(82.8557, 146.67786),
            point(90.40604, 147.68457),
            point(95.69127, 152.71812),
        ))
        .add_curve(line(
            point(95.69127, 152.71812), point(95.69127, 152.71812)))
        .add_curve(quadric(
            point(262.302, 253.38927),
            point(242.9228, 268.23825),
            point(213.22482, 268.23825),
        ))
        .add_curve(quadric(
            point(213.22482, 268.23825),
            point(204.91945, 268.23825),
            point(196.11073, 266.97986),
        ))
        .add_curve(quadric(
            point(196.11073, 266.97986),
            point(166.66443, 260.68793),
            point(150.30536, 230.2349),
        ))
        .add_curve(line(
            point(150.30536, 230.2349),
            point(150.05368, 230.48657),
        ))
        .add_curve(quadric(
            point(150.05368, 230.48657),
            point(139.23154, 246.34229),
            point(117.83892, 258.6745),
        ))
        .add_curve(quadric(
            point(117.83892, 258.6745),
            point(101.22818, 266.97986),
            point(84.11409, 266.97986),
        ))
        .add_curve(quadric(
            point(84.11409, 266.97986),
            point(61.96644, 266.97986),
            point(39.063755, 253.64095),
        ))
        .add_curve(quadric(
            point(39.063755, 253.64095),
            point(64.73489, 265.72147),
            point(88.0151, 257.4161),
        ))
        .add_curve(quadric(
            point(88.0151, 257.4161),
            point(111.295296, 249.11073),
            point(121.86577, 231.74496),
        ))
        .add_curve(quadric(
            point(121.86577, 231.74496),
            point(136.71475, 202.55034),
            point(125.38925, 169.32886),
        ))
        .add_curve(line(
            point(125.38925, 169.32886),
            point(137.72147, 161.77853),
        ))
        .add_curve(quadric(
            point(137.72147, 161.77853),
            point(149.802, 170.33557),
            point(160.1208, 162.03021),
        ))
        .add_curve(line(
            point(160.1208, 162.03021),
            point(171.69798, 169.07718),
        ))
        .add_curve(quadric(
            point(171.69798, 169.07718),
            point(160.1208, 199.78189),
            point(176.98322, 226.71141),
        ))
        .add_curve(quadric(
            point(176.98322, 226.71141),
            point(188.05704, 247.09732),
            point(207.81375, 254.52182),
        ))
        .add_curve(quadric(
            point(207.81375, 254.52182),
            point(227.57047, 261.94632),
            point(262.302, 253.38927),
        ))
        .add_curve(line(
            point(262.302, 253.38927), point(262.302, 253.38927)))
        .add_curve(quadric(
            point(181.51341, 0.45303345),
            point(210.45636, 12.533569),
            point(227.06711, 51.29196),
        ))
        .add_curve(quadric(
            point(227.06711, 51.29196),
            point(235.87582, 79.73155),
            point(217.75502, 109.17786),
        ))
        .add_curve(line(
            point(217.75502, 109.17786),
            point(218.25838, 109.17786),
        ))
        .add_curve(quadric(
            point(218.25838, 109.17786),
            point(237.13422, 110.939606),
            point(258.52682, 123.02014),
        ))
        .add_curve(quadric(
            point(258.52682, 123.02014),
            point(293.51004, 146.1745),
            point(293.51004, 192.23154),
        ))
        .add_curve(line(
            point(293.51004, 192.23154),
            point(293.51004, 193.99329),
        ))
        .add_curve(quadric(
            point(293.51004, 193.99329),
            point(291.24496, 166.05705),
            point(272.49496, 149.698),
        ))
        .add_curve(quadric(
            point(272.49496, 149.698),
            point(253.74496, 133.33893),
            point(233.61073, 133.33893),
        ))
        .add_curve(quadric(
            point(233.61073, 133.33893),
            point(200.89261, 135.10068),
            point(177.48657, 161.52686),
        ))
        .add_curve(line(
            point(177.48657, 161.52686),
            point(165.15436, 154.2282),
        ))
        .add_curve(quadric(
            point(165.15436, 154.2282),
            point(166.41275, 139.63087),
            point(153.82886, 134.849),
        ))
        .add_curve(line(
            point(153.82886, 134.849), point(153.82886, 121.51007)))
        .add_curve(quadric(
            point(153.82886, 121.51007),
            point(186.29529, 115.97316),
            point(201.39597, 88.03693),
        ))
        .add_curve(quadric(
            point(201.39597, 88.03693),
            point(213.4765, 67.902695),
            point(210.07884, 47.139267),
        ))
        .add_curve(quadric(
            point(210.07884, 47.139267),
            point(206.6812, 26.375854),
            point(181.51341, 0.45303345),
        ))
        .add_curve(line(
            point(181.51341, 0.45303345),
            point(181.51341, 0.45303345),
        ))
        .add_curve(quadric(
            point(1.5637579, 197.01343),
            point(0.8087244, 192.7349),
            point(0.8087244, 188.45639),
        ))
        .add_curve(quadric(
            point(0.8087244, 188.45639),
            point(0.8087244, 161.77853),
            point(22.453018, 132.83557),
        ))
        .add_curve(quadric(
            point(22.453018, 132.83557),
            point(42.838924, 110.68793),
            point(77.31879, 111.69464),
        ))
        .add_curve(line(
            point(77.31879, 111.69464), point(77.06711, 111.44296)))
        .add_curve(quadric(
            point(77.06711, 111.44296),
            point(69.2651, 93.825516),
            point(68.76174, 69.412766),
        ))
        .add_curve(quadric(
            point(68.76174, 69.412766),
            point(71.781876, 21.090622),
            point(121.11073, 0.9563904),
        ))
        .add_curve(quadric(
            point(121.11073, 0.9563904),
            point(89.651, 19.580551),
            point(84.994965, 43.9933),
        ))
        .add_curve(quadric(
            point(84.994965, 43.9933),
            point(80.33892, 68.40605),
            point(90.15436, 86.27518),
        ))
        .add_curve(quadric(
            point(90.15436, 86.27518),
            point(108.27516, 113.45639),
            point(142.50334, 120.25168),
        ))
        .add_curve(line(
            point(142.50334, 120.25168),
            point(142.75502, 135.10068),
        ))
        .add_curve(quadric(
            point(142.75502, 135.10068),
            point(129.16443, 141.14095),
            point(131.17784, 154.2282),
        ))
        .add_curve(line(
            point(131.17784, 154.2282),
            point(119.85234, 160.77182),
        ))
        .add_curve(quadric(
            point(119.85234, 160.77182),
            point(98.7114, 135.35236),
            point(67.25167, 136.35907),
        ))
        .add_curve(quadric(
            point(67.25167, 136.35907),
            point(43.593956, 136.10739),
            point(27.486576, 149.44632),
        ))
        .add_curve(quadric(
            point(27.486576, 149.44632),
            point(11.379194, 162.78525),
            point(1.5637579, 197.01343),
        ))
        .add_curve(line(
            point(1.5637579, 197.01343),
            point(1.5637579, 197.01343),
        ));

    canvas_builder.build()
}
