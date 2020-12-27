use core::time::Duration;
use std::ops::{AddAssign, Mul, MulAssign, SubAssign};

use bevy::prelude::*;

#[derive(Debug, Copy)]
pub struct Velocity(Vec3);

impl Velocity {
    pub fn zero() -> Self {
        Self(Vec3::zero())
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
    pub fn from(vec: Vec3) -> Self {
        Self(vec)
    }
}

impl PartialEq for Velocity {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Clone for Velocity {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

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

impl Mul<Velocity> for Quat {
    type Output = Velocity;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Velocity::from(self * rhs.0)
    }
}
