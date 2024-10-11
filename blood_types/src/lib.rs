pub mod blood;

use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    str::FromStr, vec,
};

pub use blood::*;

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

//------------------------------------------------------------------------------------------------

impl FromStr for BloodType {
    type Err = BloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let last = s.len() - 1;

        if last < 1 || last > 2 {
            return Err(BloodTypeError::Invalid("Invalid given value".to_string()));
        };

        let antigen = Antigen::from_str(&s[..last])?;
        let rh_factor = RhFactor::from_str(&s[last..])?;

        Ok(Self { antigen, rh_factor })
    }
}

//------------------------------------------------------------------------------------------------

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        use Antigen::*;
        use RhFactor::*;

        match (&self.antigen, &other.antigen) {
            (A, B) | (A, AB) | (A, O) | (B, AB) | (B, O) | (AB, O) => Ordering::Greater,
            (O, AB) | (O, B) | (O, A) | (AB, B) | (AB, A) | (B, A) => Ordering::Less,
            (A, A) | (B, B) | (AB, AB) | (O, O) => match (&self.rh_factor, &other.rh_factor) {
                (Positive, Negative) => Ordering::Greater,
                (Negative, Positive) => Ordering::Less,
                _ => Ordering::Equal,
            },
        }
    }
}

//------------------------------------------------------------------------------------------------

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh_factor = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen, rh_factor)
    }
}

//------------------------------------------------------------------------------------------------

impl BloodType {
    pub fn can_receive_from(&self, other: &BloodType) -> bool {
        self.recipients().contains(&other)
    }

    pub fn donors(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;

        match (self.antigen.clone(), self.rh_factor.clone()) {
            (O, Negative) => vec![
                Self {antigen: O, rh_factor: Negative}
                ],
            (O, Positive) => vec![
                Self {antigen: O, rh_factor: Negative},
                Self {antigen: O, rh_factor: Positive}
            ],
            (A, Negative) => vec![
                Self {antigen: A, rh_factor: Negative},
                Self { antigen: O, rh_factor: Negative,},
            ],
            (A, Positive) => vec![
                Self {antigen: A, rh_factor: Negative},
                Self { antigen: A, rh_factor: Positive,},
                Self { antigen: O, rh_factor: Negative,},
                Self { antigen: O, rh_factor: Positive,},
            ],
            (B, Negative) => vec![
                Self {antigen: B, rh_factor: Negative},
                Self { antigen: O, rh_factor: Negative,},
            ],
            (B, Positive) => vec![
                Self {antigen: B, rh_factor: Negative},
                Self { antigen: B, rh_factor: Positive,},
                Self { antigen: O, rh_factor: Negative,},
                Self { antigen: O, rh_factor: Positive,}
            ],
            (AB, Negative) => vec![
                Self {antigen: O, rh_factor: Negative},
                Self {antigen: A, rh_factor: Negative,},
                Self {antigen: B, rh_factor: Negative},
                Self {antigen: AB, rh_factor: Negative,}
            ],
            (AB, Positive) => vec![
                Self {antigen: O, rh_factor: Negative},
                Self {antigen: O, rh_factor: Positive},
                Self {antigen: A, rh_factor: Positive},
                Self {antigen: A, rh_factor: Negative},
                Self {antigen: B, rh_factor: Positive},
                Self {antigen: B, rh_factor: Negative},
                Self {antigen: AB, rh_factor: Positive},
                Self {antigen: AB, rh_factor: Negative},
            ]
        }
    }

    pub fn recipients(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;

        match (self.antigen.clone(), self.rh_factor.clone()) {
            (AB, Positive) => vec![
                Self {antigen: A, rh_factor: Positive}
                ],
            (AB, Negative) => vec![
                Self {antigen: A, rh_factor: Negative},
                Self {antigen: AB, rh_factor: Positive}
            ],
            (A, Positive) => vec![
                Self {antigen: A, rh_factor: Positive},
                Self { antigen: AB, rh_factor: Positive,},
            ],
            (A, Negative) => vec![
                Self {antigen: A, rh_factor: Negative},
                Self { antigen: A, rh_factor: Positive,},
                Self { antigen: AB, rh_factor: Negative,},
                Self { antigen: AB, rh_factor: Positive,},
            ],
            (B, Positive) => vec![
                Self {antigen: B, rh_factor: Positive},
                Self { antigen: AB, rh_factor: Positive,},
            ],
            (B, Negative) => vec![
                Self {antigen: B, rh_factor: Negative},
                Self { antigen: B, rh_factor: Positive,},
                Self { antigen: AB, rh_factor: Negative,},
                Self { antigen: AB, rh_factor: Positive,}
            ],
            (O, Positive) => vec![
                Self {antigen: O, rh_factor: Positive},
                Self {antigen: A, rh_factor: Positive,},
                Self {antigen: B, rh_factor: Positive},
                Self {antigen: AB, rh_factor: Positive,}
            ],
            (O, Negative) => vec![
                Self {antigen: O, rh_factor: Negative},
                Self {antigen: O, rh_factor: Positive},
                Self {antigen: A, rh_factor: Positive},
                Self {antigen: A, rh_factor: Negative},
                Self {antigen: B, rh_factor: Positive},
                Self {antigen: B, rh_factor: Negative},
                Self {antigen: AB, rh_factor: Positive},
                Self {antigen: AB, rh_factor: Negative},
            ]
        }
    }
}
