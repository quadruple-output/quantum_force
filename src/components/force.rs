use crate::components::{Acceleration, Mass};
use bevy::prelude::*;
use std::ops::{AddAssign, Div};

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
