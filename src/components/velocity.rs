use core::time::Duration;
use std::ops::AddAssign;
use std::ops::Mul;

use bevy::prelude::Vec3;

#[derive(Debug, Default, Copy, Clone)]
pub struct Velocity(pub Vec3);

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul<Duration> for Velocity {
    type Output = Vec3;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.0 * rhs.as_secs_f32()
    }
}
