/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use font_rasterizer::*;
use owned_ttf_parser as ttfp;
use ttfp::AsFaceRef;

use crate::outliner::*;

pub trait Example {
    fn name(&self) -> &str;
    fn letter(&self) -> char;
    fn size(&self) -> f32;
    fn font(&self) -> &'static [u8];

    fn build_canvas(&self) -> Canvas {
        let owned_face = ttfp::OwnedFace::from_vec(self.font().to_vec(), 0).unwrap();
        let parsed_face = ttfp::PreParsedSubtables::from(owned_face);

        let glyph_id = parsed_face.glyph_index(self.letter()).unwrap();

        let mut outliner = Outliner::default();
        let rect = parsed_face
            .as_face_ref()
            .outline_glyph(glyph_id, &mut outliner)
            .unwrap();

        let height: f32 =
            (parsed_face.as_face_ref().ascender() - parsed_face.as_face_ref().descender()).into();
        let h_factor = self.size() / height;
        let v_factor = self.size() / height;

        let bounds = ttfp::Rect {
            x_min: (rect.x_min as f32 * h_factor) as i16,
            x_max: (rect.x_max as f32 * h_factor) as i16,
            y_min: (rect.y_min as f32 * v_factor) as i16,
            y_max: (rect.y_max as f32 * v_factor) as i16,
        };

        let mut canvas_builder =
            CanvasBuilder::new(bounds.width() as usize + 2, bounds.height() as usize + 2);

        for mut line in outliner.outline {
            line *= point(h_factor, -v_factor);
            line -= point(bounds.x_min as f32, -bounds.y_min as f32);
            line += point(0.0, bounds.height() as f32);

            canvas_builder = canvas_builder.add_line(line);
        }

        canvas_builder.build()
    }

    fn create_image(&self) {
        let canvas = self.build_canvas();

        let data = canvas
            .iter()
            .flat_map(|a| [0, 0, 0, (255.0 * a.abs()) as u8])
            .collect::<Vec<u8>>();

        let filename = format!("examples/images/{}_{}px.png", self.name(), self.size());

        let _ = image::save_buffer(
            &std::path::Path::new(&filename),
            &data,
            canvas.width() as u32,
            canvas.height() as u32,
            image::ColorType::Rgba8,
        );
    }
}

macro_rules! example {
    ($example: ident($name: literal, $letter: literal, $size: literal, $font: literal)) => {
        pub struct $example;

        impl Example for $example {
            fn name(&self) -> &str {
                $name
            }
            fn letter(&self) -> char {
                $letter
            }
            fn size(&self) -> f32 {
                $size
            }
            fn font(&self) -> &'static [u8] {
                include_bytes!($font)
            }
        }
    };
}

example!(W("w", 'w', 16.0, "../fonts/DejaVuSansMono.ttf"));
example!(Iota("iota", 'ΐ', 60.0, "../fonts/OpenSans-Italic.ttf"));
example!(TailedE("tailed_e", 'ę', 300.0, "../fonts/Exo2-Light.ttf"));
example!(Biohazard(
    "biohazard",
    '☣',
    600.0,
    "../fonts/DejaVuSansMono.ttf"
));
example!(Ichi("ichi", '一', 100.0, "../fonts/mingliu.ttc"));
example!(StressedE(
    "stressed_e",
    'É',
    60.0,
    "../fonts/DejaVuSansMono.ttf"
));
