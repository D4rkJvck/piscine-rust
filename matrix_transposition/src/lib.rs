#[derive(Debug, PartialEq, Eq)]
pub struct Matrix((i32, i32), (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    Matrix((m.0 .0, m.1 .0), (m.0 .1, m.1 .1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_zero() {
        let m = Matrix((0, 0), (0, 0));
        let m = transpose(m);
        assert_eq!(m, Matrix((0, 0), (0, 0)));
    }

    #[test]
    fn transpose_identity() {
        let m = Matrix((1, 0), (0, 1));
        let m = transpose(m);
        assert_eq!(m, Matrix((1, 0), (0, 1)));
    }

    #[test]
    fn transpose_matrix() {
        let m = Matrix((1, 3), (4, 5));
        let m = transpose(m);
        assert_eq!(m, Matrix((1, 4), (3, 5)));

        let m = Matrix((6, 8), (1, 3));
        let m = transpose(m);
        assert_eq!(m, Matrix((6, 1), (8, 3)));
    }
}
