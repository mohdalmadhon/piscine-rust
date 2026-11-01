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

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar<Item = T>> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        Some(Vector(
            self.0.iter().zip(rhs.0.iter()).map(|(&a, &b)| a + b).collect(),
        ))
    }
}

impl<T: Scalar<Item = T>> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        Some(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .fold(T::zero(), |acc, (&a, &b)| acc + a * b),
        )
    }
}