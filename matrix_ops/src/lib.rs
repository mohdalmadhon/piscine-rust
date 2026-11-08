use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + Sized
{
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

macro_rules! impl_scalar {
    ($t:ty, $zero:expr, $one:expr) => {
        impl Scalar for $t {
            type Item = $t;

            fn zero() -> Self::Item {
                $zero
            }

            fn one() -> Self::Item {
                $one
            }
        }
    };
}

impl_scalar!(u32, 0, 1);
impl_scalar!(u64, 0, 1);
impl_scalar!(i32, 0, 1);
impl_scalar!(i64, 0, 1);
impl_scalar!(f32, 0.0, 1.0);
impl_scalar!(f64, 0.0, 1.0);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar<Item = T>> Matrix<W, H, T> {
    pub fn zero() -> Self {
        Matrix([[T::zero(); W]; H])
    }
}

impl<const S: usize, T: Scalar<Item = T>> Matrix<S, S, T> {
    pub fn identity() -> Self {
        let mut m = Self::zero();
        for i in 0..S {
            m.0[i][i] = T::one();
        }
        m
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
    fn from(arr: [[T; W]; H]) -> Self {
        Wrapper(Matrix(arr))
    }
}

impl<const W: usize, const H: usize, T: Scalar<Item = T>> Add for Wrapper<W, H, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut res = self;
        for r in 0..H {
            for c in 0..W {
                res.0.0[r][c] = self.0.0[r][c] + rhs.0.0[r][c];
            }
        }
        res
    }
}

impl<const W: usize, const H: usize, T: Scalar<Item = T>> Sub for Wrapper<W, H, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut res = self;
        for r in 0..H {
            for c in 0..W {
                res.0.0[r][c] = self.0.0[r][c] - rhs.0.0[r][c];
            }
        }
        res
    }
}

impl<const S: usize, T: Scalar<Item = T>> Mul for Wrapper<S, S, T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut res = Matrix::<S, S, T>::zero();
        for i in 0..S {
            for j in 0..S {
                for k in 0..S {
                    res.0[i][j] = res.0[i][j] + self.0.0[i][k] * rhs.0.0[k][j];
                }
            }
        }
        Wrapper(res)
    }
}