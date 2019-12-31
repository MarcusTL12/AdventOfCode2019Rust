use std::ops;

use num::Integer;
use num::Num;

use std::fmt;

use modinverse::modinverse;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Zn<T: Integer + Copy> {
    i: T,
    n: T,
}

impl<T: Integer + Copy> Zn<T> {
    pub fn new(i: T, n: T) -> Zn<T> {
        Zn { i: i % n, n: n }
    }
    pub fn order(&self) -> T {
        self.n
    }
}

impl<T: Integer + Copy + fmt::Debug> fmt::Debug for Zn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} mod {:?}", self.i, self.n)
    }
}

impl<T: Integer + Copy> ops::Add<Zn<T>> for Zn<T> {
    type Output = Zn<T>;
    fn add(self, rhs: Self) -> Self {
        let n = self.n.max(rhs.n);
        Zn::new((self.i + rhs.i) % n, n)
    }
}

impl<T: Integer + Copy> ops::AddAssign<Zn<T>> for Zn<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Integer + Copy> ops::Neg for Zn<T> {
    type Output = Zn<T>;
    fn neg(self) -> Self {
        Self::new(self.n - self.i, self.n)
    }
}

impl<T: Integer + Copy> ops::Sub<Zn<T>> for Zn<T> {
    type Output = Zn<T>;
    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl<T: Integer + Copy> ops::SubAssign<Zn<T>> for Zn<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Integer + Copy, S: Integer> ops::Mul<S> for Zn<T> {
    type Output = Zn<T>;
    fn mul(self, mut rhs: S) -> Self {
        let mut acc = Self::new(T::zero(), self.n);
        let mut cur = self;
        let neg = rhs < S::zero();
        if neg {
            rhs = S::zero() - rhs;
        }
        while rhs != S::zero() {
            let (q, r) = rhs.div_rem(&(S::one() + S::one()));
            rhs = q;
            if r != S::zero() {
                acc += cur;
            }
            cur += cur;
        }
        if neg {
            -acc
        } else {
            acc
        }
    }
}

impl<T: Integer + Copy> ops::Mul<Zn<T>> for Zn<T> {
    type Output = Zn<T>;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.i, self.n.max(rhs.n)) * rhs.i
    }
}

impl<T: Integer + Copy> ops::MulAssign<Zn<T>> for Zn<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<T: Integer + Copy> ops::Div<Zn<T>> for Zn<T> {
    type Output = Zn<T>;
    fn div(self, rhs: Self) -> Self {
        let inv = match modinverse(rhs.i, rhs.n) {
            Some(x) => x,
            None => panic!("Division by noninvertible element"),
        };
        self * Self::new(inv, rhs.n)
    }
}

impl<T: Integer + Copy> ops::DivAssign<Zn<T>> for Zn<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Integer + Copy> num::Zero for Zn<T> {
    fn is_zero(&self) -> bool {
        self.i == T::zero()
    }
    fn zero() -> Zn<T> {
        Zn::new(T::zero(), T::one())
    }
}

impl<T: Integer + Copy> num::One for Zn<T> {
    fn is_one(&self) -> bool {
        self.i == T::one()
    }
    fn one() -> Zn<T> {
        Zn::new(T::one(), T::one())
    }
}

impl<T: Integer + Copy> ops::Rem<Zn<T>> for Zn<T> {
    type Output = Zn<T>;
    fn rem(self, _: Self) -> Self {
        Self::new(T::zero(), self.n)
    }
}

impl<T: Integer + Copy> Num for Zn<T> {
    type FromStrRadixErr = num::traits::ParseFloatError;
    fn from_str_radix(
        s: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr> {
        match T::from_str_radix(s, radix) {
            Ok(x) => Ok(Self::new(x, T::zero())),
            Err(_) => panic!("NaNi?"),
        }
    }
}
