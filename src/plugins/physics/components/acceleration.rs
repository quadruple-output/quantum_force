use super::Velocity;
use bevy::{prelude::*, utils::Duration};
use std::ops::Mul;

#[derive(Copy, Clone, Default)]
pub struct Acceleration(pub Vec3);

impl Mul<Duration> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        Self::Output::from(self.0 * rhs.as_secs_f32())
    }
}
