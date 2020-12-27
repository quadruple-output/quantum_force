use num_traits::NumAssign;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct PercentPerSecond<N: NumAssign>(pub N);

impl<N: NumAssign> Mul<N> for PercentPerSecond<N> {
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<N: NumAssign> MulAssign<N> for PercentPerSecond<N> {
    fn mul_assign(&mut self, rhs: N) {
        self.0 *= rhs;
    }
}

impl<N: NumAssign> AddAssign for PercentPerSecond<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<N: NumAssign> Add for PercentPerSecond<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
