use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
