use std::str::FromStr;

#[derive(Debug)]
pub enum BloodTypeError {
    Invalid(String),
    // TODO: Rest of potential `error` types
}


//________________________________________________________________
//

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Antigen {
    A,
    B,
    AB,
    O,
}

impl FromStr for Antigen {
    type Err = BloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "AB" => Ok(Self::AB),
            "O" => Ok(Self::O),
            _ => Err(BloodTypeError::Invalid(
                "Invalid blood antigen value".to_string(),
            )),
        }
    }
}

//________________________________________________________________
//

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl FromStr for RhFactor {
    type Err = BloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Positive),
            "-" => Ok(Self::Negative),
            _ => Err(BloodTypeError::Invalid(
                "Invalid Rhesus factor value".to_string()
            ))
        }
    }
}
