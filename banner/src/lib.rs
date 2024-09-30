use std::{collections::HashMap, num::ParseFloatError};

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Self {
        let short_hand = format!("-{}", l_h.chars().next().unwrap());
        let long_hand = format!("--{l_h}");
        let desc = d.to_string();

        Flag {
            short_hand,
            long_hand,
            desc,
        }
    }
}

//_______________________________________________________________________________
//

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&self, flag: (String, String), argv: &[&str]) -> String {
        let callback = match self.flags.get(&flag) {
            None => return "Callback not found".to_string(),
            Some(callback) => callback,
        };

        match callback(argv[0], argv[1]) {
            Ok(res) => res,
            Err(err) => format!("Error parsing float: {}", err),
        }
    }
}

//__________________________________________________________________________________
//

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let ops = match parse_float(a, b) {
        Err(err) => return Err(err),
        Ok(val) => val
    };

    Ok((ops.0 / ops.1).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let ops = match parse_float(a, b) {
        Err(err) => return Err(err),
        Ok(val) => val
    };

    Ok((ops.0 % ops.1).to_string())
}

//_______________________________________________________________________________________
//

fn parse_float(a: &str, b: &str) -> Result<(f32, f32), ParseFloatError> {
    let num_a = match a.parse::<f32>() {
        Ok(num) => num,
        Err(err) => return Err(err),
    };

    let num_b = match b.parse::<f32>() {
        Ok(num) => num,
        Err(err) => return Err(err),
    };

    Ok((num_a, num_b))
}