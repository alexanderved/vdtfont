mod outliner;
mod example;

use crate::example::*;

fn main() {
    let examples: [&dyn Example; 5] = [
        &W,
        &Iota,
        &TailedE,
        &Biohazard,
        &Ichi
    ];

    for example in examples {
        example.create_image();
    }
}