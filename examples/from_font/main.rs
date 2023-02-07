/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod example;
mod outliner;
 
use std::{time, hint};
use crate::example::*;
 
const BENCH_COUNT: usize = 10000;
 
fn bench(example: &dyn Example) -> time::Duration {
    let now = time::Instant::now();
    let f = || (0..BENCH_COUNT).into_iter()
        .for_each(|_| { example.build_canvas(); });
 
    hint::black_box(f());
 
    now.elapsed()
}

fn main() {
    let examples: [&dyn Example; 6] = [&W, &Iota, &TailedE, &Biohazard, &Ichi, &StressedE];
 
    for example in examples {
        let dur = bench(example);
        println!("{BENCH_COUNT} {}: {} s, {} ns, {} ms", example.name(),
            dur.as_secs(), dur.as_nanos(), dur.as_millis());
        println!("1 {}: {} ns, {} µs", example.name(), dur.as_nanos() / BENCH_COUNT as u128,
            dur.as_micros() as f32 / BENCH_COUNT as f32);
    }
} 