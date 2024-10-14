use matrix_ops::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T>
where
    T: Clone + Copy,
{
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].to_vec()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

//----------------------------------------------------------------------------

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Vec::new();

        for (v1, v2) in self.0.iter().zip(rhs.0.iter()) {
            if v1.len() != v2.len() {
                return None;
            }

            let mut submatrix = Vec::new();

            for (&a, &b) in v1.iter().zip(v2.iter()) {
                submatrix.push(a * b);
            }

            matrix.push(submatrix)
        }

        Some(Self(matrix))
    }
}
