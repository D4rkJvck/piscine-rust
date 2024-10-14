use std::iter::Sum;

pub use lalgebra_scalar::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T> Add for Vector<T>
where
    T: Scalar + Sum + Clone,
{
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        };

        let tab = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect::<Vec<T>>();

        Some(Vector(tab))
    }
}

impl<T> Vector<T>
where
    T: Scalar + Sum + Clone,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        };

        Some(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a.clone() * b.clone())
                .sum::<T>(),
        )
    }
}
