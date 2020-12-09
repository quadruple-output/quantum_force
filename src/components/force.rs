use crate::components::{Mass, Velocity};
use bevy::prelude::*;
use core::time::Duration;
use std::ops::{AddAssign, Div, Mul};

#[derive(Default, Copy, Clone)]
pub struct Force(pub Vec3);

impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, rhs: Mass) -> Self::Output {
        Acceleration(self.0 / rhs.0)
    }
}

impl AddAssign for Force {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(Default, Copy, Clone)]
pub struct Acceleration(pub Vec3);

impl Mul<Duration> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        Velocity(self.0 * rhs.as_secs_f32())
    }
}
