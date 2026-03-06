use std::fmt;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Scalar {
    pub re: i32,
    pub im: i32,
}

impl Scalar {
    pub const fn new(re: i32, im: i32) -> Self {
        Self { re, im }
    }

    pub const fn from_int(n: i32) -> Self {
        Self { re: n, im: 0 }
    }

    pub const fn i() -> Self {
        Self { re: 0, im: 1 }
    }

    pub const fn is_zero(self) -> bool {
        self.re == 0 && self.im == 0
    }
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Sub for Scalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.re, self.im) {
            (0, 0) => write!(f, "0"),
            (re, 0) => write!(f, "{re}"),
            (0, 1) => write!(f, "i"),
            (0, -1) => write!(f, "-i"),
            (0, im) => write!(f, "{im}i"),
            (re, im) if im > 0 => write!(f, "{re}+{im}i"),
            (re, im) => write!(f, "{re}{im}i"),
        }
    }
}
