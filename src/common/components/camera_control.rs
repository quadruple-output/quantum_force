use std::ops::{AddAssign, MulAssign};

use super::super::types::{PercentPerSecond, RadPerSecond};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct CameraControl {
    pub yaw: RadPerSecond<f32>,
    pub pitch: RadPerSecond<f32>,
    pub escape_velocity: PercentPerSecond<f32>,
}

impl CameraControl {
    pub fn is_initial(&self) -> bool {
        *self == Self::default()
    }

    fn pseudo_velocity(&self) -> f32 {
        self.yaw.0.abs() * self.yaw.0.abs()
            + self.pitch.0.abs() * self.pitch.0.abs()
            + self.escape_velocity.0.abs() * self.escape_velocity.0.abs()
    }
}

impl PartialEq<f32> for CameraControl {
    fn eq(&self, other: &f32) -> bool {
        self.pseudo_velocity() == *other
    }
}

impl PartialOrd<f32> for CameraControl {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.pseudo_velocity().partial_cmp(other)
    }
}

impl MulAssign<f32> for CameraControl {
    fn mul_assign(&mut self, rhs: f32) {
        self.yaw *= rhs;
        self.pitch *= rhs;
        self.escape_velocity *= rhs;
    }
}

impl AddAssign for CameraControl {
    fn add_assign(&mut self, rhs: Self) {
        self.yaw += rhs.yaw;
        self.pitch += rhs.pitch;
        self.escape_velocity += rhs.escape_velocity;
    }
}
