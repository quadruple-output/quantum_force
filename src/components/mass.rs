use std::ops::{Add, Mul};

#[derive(Copy, Clone, PartialEq)]
pub struct Mass(pub f32);

impl Add for Mass {
    type Output = f32;

    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Mul for Mass {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0
    }
}
