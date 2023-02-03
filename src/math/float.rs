/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub trait ReverseFract {
    fn rfract(self) -> Self;
}

impl ReverseFract for f32 {
    fn rfract(self) -> Self {
        1.0 - self.fract()
    }
}
