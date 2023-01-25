pub trait ReverseFract {
    fn rfract(self) -> Self;
}

impl ReverseFract for f32 {
    fn rfract(self) -> Self {
        1.0 - self.fract()
    }
}