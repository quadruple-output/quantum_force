use core::time::Duration;
use std::ops::{AddAssign, Mul, MulAssign, SubAssign};

use bevy::prelude::Vec3;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Velocity(pub Vec3);

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl MulAssign<f32> for Velocity {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

impl Mul<Duration> for Velocity {
    type Output = Vec3;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.0 * rhs.as_secs_f32()
    }
}

impl SubAssign for Velocity {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl PartialEq<f32> for Velocity {
    fn eq(&self, other: &f32) -> bool {
        self.0.length() == *other
    }
}

impl PartialOrd<f32> for Velocity {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.0.length().partial_cmp(other)
    }
}
