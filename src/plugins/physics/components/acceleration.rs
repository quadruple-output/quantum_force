use crate::common::components::Velocity;
use bevy::prelude::*;
use bevy::utils::Duration;
use std::ops::Mul;

#[derive(Copy, Clone, Default)]
pub struct Acceleration(pub Vec3);

impl Mul<Duration> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        Velocity(self.0 * rhs.as_secs_f32())
    }
}
