/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Additional operations on floats.
pub trait FloatExt {
    fn rfract(self) -> Self;
}

impl FloatExt for f32 {
    fn rfract(self) -> Self {
        1.0 - self.fract()
    }
}
