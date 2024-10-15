use lalgebra_vector::*;
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
    T: Mul<Output = T> + Copy + Scalar<Num = T> + Sum,
{
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if !self.number_of_cols() == rhs.number_of_rows() {
            return None;
        }

        let mut matrix: Vec<Vec<T>> =
            vec![vec![T::zero(); self.number_of_rows()]; rhs.number_of_cols()];

        for i in 0..self.number_of_rows() {
            for j in 0..rhs.number_of_cols() {
                let v1 = Vector(self.row(i));
                let v2 = Vector(rhs.col(j));

                matrix[i][j] = v1.dot(&v2).unwrap();
            }
        }

        Some(Self(matrix))
    }
}
