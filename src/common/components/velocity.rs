use core::time::Duration;
use std::{
    marker::PhantomData,
    ops::{AddAssign, Mul, MulAssign, SubAssign},
};

use bevy::prelude::*;

#[derive(Debug, Copy)]
pub struct Velocity<P: Plugin>(
    Vec3,
    PhantomData<P>, // We need two independent kinds of Velocity (Camera & Physics) which do not interact with each other
);

impl<P: Plugin> Velocity<P> {
    pub fn zero() -> Self {
        Self(Vec3::zero(), PhantomData)
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z), PhantomData)
    }
    pub fn from(vec: Vec3) -> Self {
        Self(vec, PhantomData)
    }
}

impl<P: Plugin> PartialEq for Velocity<P> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<P: Plugin> Clone for Velocity<P> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<P: Plugin> AddAssign for Velocity<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<P: Plugin> MulAssign<f32> for Velocity<P> {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

impl<P: Plugin> Mul<Duration> for Velocity<P> {
    type Output = Vec3;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.0 * rhs.as_secs_f32()
    }
}

impl<P: Plugin> SubAssign for Velocity<P> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<P: Plugin> PartialEq<f32> for Velocity<P> {
    fn eq(&self, other: &f32) -> bool {
        self.0.length() == *other
    }
}

impl<P: Plugin> PartialOrd<f32> for Velocity<P> {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.0.length().partial_cmp(other)
    }
}

impl<P: Plugin> Mul<Velocity<P>> for Quat {
    type Output = Velocity<P>;

    fn mul(self, rhs: Velocity<P>) -> Self::Output {
        Velocity::from(self * rhs.0)
    }
}
