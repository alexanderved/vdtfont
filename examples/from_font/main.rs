/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod example;
mod outliner;

use crate::example::*;

fn main() {
    let examples: [&dyn Example; 6] = [&W, &Iota, &TailedE, &Biohazard, &Ichi, &StressedE];

    for example in examples {
        example.create_image();
    }
}
