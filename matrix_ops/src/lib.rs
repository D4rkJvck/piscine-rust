use std::ops::{Add, Sub};

pub use matrix::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T>
where
    T: Scalar<Num = T> + Add<Output = T> + Copy,
{
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix: Vec<Vec<T>> = Vec::new();

        for (vec1, vec2) in self.0.iter().zip(rhs.0.iter()) {
            if vec1.len()!= vec2.len() {
                return None;
            }

            let mut submatrix: Vec<T> = Vec::new();

            for (&a, &b) in vec1.iter().zip(vec2.iter()) {
                submatrix.push(a + b)
            }

            matrix.push(submatrix);
        }

        Some(Self(matrix))
    }
}

//----------------------------------------------------------------

impl<T> Sub for Matrix<T>
where 
    T: Scalar<Num = T> + Sub<Output = T> + Copy,
{
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut matrix: Vec<Vec<T>> = Vec::new();

        for (vec1, vec2) in self.0.iter().zip(rhs.0.iter()) {
            if vec1.len()!= vec2.len() {
                return None;
            }

            let mut submatrix: Vec<T> = Vec::new();

            for (&a, &b) in vec1.iter().zip(vec2.iter()) {
                submatrix.push(a - b)
            }

            matrix.push(submatrix);
        }

        Some(Self(matrix))
    }
}
