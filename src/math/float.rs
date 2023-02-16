/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Additional operations on floats.
pub trait FloatExt {
    #[cfg(feature = "no_std")]
    fn abs(self) -> Self;
    #[cfg(feature = "no_std")]
    fn powi(self, n: i32) -> Self;
    #[cfg(feature = "no_std")]
    fn sqrt(self) -> Self;
    #[cfg(feature = "no_std")]
    fn round(self) -> Self;
    #[cfg(feature = "no_std")]
    fn fract(self) -> Self;

    fn rfract(self) -> Self;
}

impl FloatExt for f32 {
    #[cfg(feature = "no_std")]
    fn abs(self) -> Self {
        libm::fabsf(self)
    }

    #[cfg(feature = "no_std")]
    fn powi(self, n: i32) -> Self {
        libm::powf(self, n as f32)
    }

    #[cfg(feature = "no_std")]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    #[cfg(feature = "no_std")]
    fn round(self) -> Self {
        libm::roundf(self)
    }

    #[cfg(feature = "no_std")]
    fn fract(self) -> Self {
        self - libm::floorf(self)
    }

    fn rfract(self) -> Self {
        1.0 - self.fract()
    }
}
