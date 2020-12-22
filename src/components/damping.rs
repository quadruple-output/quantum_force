use std::{ops::Mul, time::Duration};

#[derive(Clone, Copy)]
pub struct Damping(f32);

impl Damping {
    pub fn half_live(half_live: f32) -> Self {
        Damping(f32::ln(2.0) / half_live)
    }
}

impl Mul<Duration> for Damping {
    type Output = f32;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.0 * rhs.as_secs_f32()
    }
}
