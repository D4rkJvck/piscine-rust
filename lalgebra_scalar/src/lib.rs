pub mod scalar;

pub use scalar::*;

impl Scalar for u32 {
    type Num = u32;

    fn zero() -> Self::Num {
        0
    }

    fn one() -> Self::Num {
        1
    }
}

impl Scalar for u64 {
    type Num = u64;

    fn zero() -> Self::Num {
        0
    }

    fn one() -> Self::Num {
        1
    }
}

impl Scalar for i32 {
    type Num = i32;

    fn zero() -> Self::Num {
        0
    }

    fn one() -> Self::Num {
        1
    }
}

impl Scalar for i64 {
    type Num = i64;

    fn zero() -> Self::Num {
        0
    }

    fn one() -> Self::Num {
        1
    }
}

impl Scalar for f32 {
    type Num = f32;

    fn zero() -> Self::Num {
        0.0
    }

    fn one() -> Self::Num {
        1.0
    }
}

impl Scalar for f64 {
    type Num = f64;

    fn zero() -> Self::Num {
        0.0
    }

    fn one() -> Self::Num {
        1.0
    }
}
