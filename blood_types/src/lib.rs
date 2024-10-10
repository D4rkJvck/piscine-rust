pub mod blood;

use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    str::FromStr,
};

pub use blood::*;

#[derive(Debug)]
pub enum BloodTypeError {
    Invalid,
    // TODO: Rest of potential `error` types
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
    type Err = BloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen = Antigen::O;
        let rh_factor = RhFactor::Negative;
        Ok(Self { antigen, rh_factor })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &BloodType) -> bool {
        self.recipients().contains(&other)
    }

    pub fn donors(&self) -> Vec<Self> {
        vec![]
    }

    pub fn recipients(&self) -> Vec<Self> {
        vec![]
    }
}
