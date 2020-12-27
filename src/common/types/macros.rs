macro_rules! newtype {
    {$pub:vis $typename:ident<N>;} => {
				#[derive(Copy, Clone, Default, PartialEq)]
        $pub struct $typename<N>($pub N) where N: num_traits::NumAssign;

				impl<N: num_traits::NumAssign> core::ops::Mul<N> for $typename<N> {
						type Output = Self;

						fn mul(self, rhs: N) -> Self::Output {
								Self(self.0 * rhs)
						}
				}

				impl<N: num_traits::NumAssign> core::ops::MulAssign<N> for $typename<N> {
						fn mul_assign(&mut self, rhs: N) {
								self.0 *= rhs;
						}
				}

				impl<N: num_traits::NumAssign> core::ops::AddAssign for $typename<N> {
						fn add_assign(&mut self, rhs: Self) {
								self.0 += rhs.0;
						}
				}

				impl<N: num_traits::NumAssign> core::ops::Add for $typename<N> {
						type Output = Self;

						fn add(self, rhs: Self) -> Self::Output {
								Self(self.0 + rhs.0)
						}
				}
    };
}
