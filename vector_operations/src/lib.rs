use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T> Add for ThreeDVector<T>
where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let i = self.i + rhs.i;
        let j = self.j + rhs.j;
        let k = self.k + rhs.k;

        Self { i, j, k }
    }
}

impl<T> Sub for ThreeDVector<T>
where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let i = self.i - rhs.i;
        let j = self.j - rhs.j;
        let k = self.k - rhs.k;

        Self { i, j, k }
    }
}