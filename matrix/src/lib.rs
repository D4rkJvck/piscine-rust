pub use lalgebra_scalar::Scalar;

#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Num = T>> Matrix<T> {
    pub fn new() -> Self {
        Self(vec![vec![]])
    }

    /// Returns the matrix identity of a
    /// given number: it fills the diagonals
    /// to 1 while the rest will be filled with 0.
    pub fn identity(n: usize) -> Self {
        let mut matrix = vec![vec![T::zero(); n]; n];

        for i in 0..n {
            matrix[i][i] = T::one()
        }

        Self(matrix)
    }

    /// Does the same operation as Identity but
    /// without identity: it fills a matrix sized
    /// with the given numbers with 0.
    pub fn zero(row: usize, col: usize) -> Self {
        Self(vec![vec![T::zero(); col]; row])
    }
}
