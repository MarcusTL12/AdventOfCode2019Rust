use std::ops;

use num::{Zero, Num};

use modinverse::modinverse;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Zn<const N: u64> {
    i: u64,
}

impl<const N: u64> Zn<N> {
    pub fn new(i: u64) -> Zn<N> {
        Zn { i: i % N }
    }
    // pub fn n(&self) -> u64 {
    //     N
    // }
}

impl<const N: u64> std::fmt::Debug for Zn<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} mod {:?}", self.i, N)
    }
}

impl<const N: u64> ops::Add<Zn<N>> for Zn<N> {
    type Output = Zn<N>;
    fn add(self, rhs: Self) -> Self {
        Zn::new(self.i + rhs.i)
    }
}

impl<const N: u64> ops::AddAssign<Zn<N>> for Zn<N> {
    fn add_assign(&mut self, rhs: Zn<N>) {
        *self = *self + rhs;
    }
}

impl<const N: u64> ops::Neg for Zn<N> {
    type Output = Zn<N>;
    fn neg(self) -> Self {
        Zn::new(N - self.i)
    }
}

impl<const N: u64> ops::Sub<Zn<N>> for Zn<N> {
    type Output = Zn<N>;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl<const N: u64> ops::SubAssign<Zn<N>> for Zn<N> {
    fn sub_assign(&mut self, other: Zn<N>) {
        *self = *self - other;
    }
}

impl<const N: u64> Zero for Zn<N> {
    fn is_zero(&self) -> bool {
        self.i.is_zero()
    }
    fn zero() -> Zn<N> {
        Zn::new(0)
    }
}

impl<const N: u64> num::One for Zn<N> {
    fn is_one(&self) -> bool {
        self.i.is_one()
    }
    fn one() -> Zn<N> {
        Zn::new(1)
    }
}

impl<T: num::Integer, const N: u64> ops::Mul<T> for Zn<N> {
    type Output = Zn<N>;
    fn mul(self, mut rhs: T) -> Self {
        let mut acc = Self::zero();
        let mut cur = self;
        let neg = rhs < T::zero();
        if neg {
            rhs = T::zero() - rhs;
        }
        while rhs != T::zero() {
            let (q, r) = rhs.div_rem(&(T::one() + T::one()));
            rhs = q;
            if r != T::zero() {
                acc += cur;
            }
            cur += cur;
        }
        if neg {
            -acc
        } else {
            acc
        };
        
        self
    }
}

impl<const N: u64> ops::Mul<Zn<N>> for Zn<N> {
    type Output = Zn<N>;
    fn mul(self, rhs: Zn<N>) -> Self {
        self * rhs.i
    }
}

impl<const N: u64> ops::MulAssign<Zn<N>> for Zn<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const N: u64> ops::Div<Zn<N>> for Zn<N> {
    type Output = Zn<N>;
    fn div(self, rhs: Self) -> Self {
        let inv = match modinverse(rhs.i, N) {
            Some(x) => x,
            None => panic!("Division by noninvertible element"),
        };
        self * Self::new(inv)
    }
}

impl<const N: u64> ops::DivAssign<Zn<N>> for Zn<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const N: u64> ops::Rem<Zn<N>> for Zn<N> {
    type Output = Zn<N>;
    fn rem(self, _: Self) -> Self {
        Self::zero()
    }
}

impl<const N: u64> Num for Zn<N> {
    type FromStrRadixErr = num::traits::ParseFloatError;
    fn from_str_radix(
        s: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr> {
        match u64::from_str_radix(s, radix) {
            Ok(x) => Ok(Self::new(x)),
            Err(_) => panic!("NaNi?"),
        }
    }
}