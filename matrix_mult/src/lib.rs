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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.first().map_or(0, |row| row.len())
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

impl<T: Scalar<Item = T>> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let row = self.row(i);
            let mut new_row = Vec::new();
            for j in 0..rhs.number_of_cols() {
                let col = rhs.col(j);
                let sum = row
                    .iter()
                    .zip(col.iter())
                    .fold(T::zero(), |acc, (&a, &b)| acc + a * b);
                new_row.push(sum);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}