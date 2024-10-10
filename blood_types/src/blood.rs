use std::str::FromStr;

#[derive(Debug)]
pub enum AntigenError {
    Invalid,
    // TODO: Rest of potential `error` types
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Antigen {
    A,
    B,
    AB,
    O,
}

impl FromStr for Antigen {
    type Err = AntigenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::O)
    }
}

//________________________________________________________________
//

#[derive(Debug)]
pub enum RhFactorError {
    Invalid,
    // TODO: Rest of potential `error` types
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl FromStr for RhFactor {
    type Err = RhFactorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Negative)
    }
}
