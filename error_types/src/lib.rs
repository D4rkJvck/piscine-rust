use chrono::{Local, NaiveDate};

#[derive(Debug, PartialEq, Eq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Self {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name == "" {
            Err(FormError::new(
                "first_name".to_string(),
                self.first_name.to_string(),
                "No user name".to_string(),
            ))
        } else if self.password.len() < 8 {
            Err(FormError::new(
                "password".to_string(),
                self.password.to_string(),
                "At least 8 characters".to_string(),
            ))
        } else if !has_ascii_combination(&self.password) {
            Err(FormError::new(
                "password".to_string(), 
                self.password.to_string(), 
                "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()
            ))
        } else {
            Ok(vec!["Valid first name", "Valid password"])
        }
    }
}

//___________________________________________________________________
//

#[derive(Debug, PartialEq, Eq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> Self {
        let form_values = (field_name, field_value);
        let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        FormError {
            form_values,
            date,
            err,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

fn has_ascii_combination(s: &str) -> bool {
    let mut has_digit = false;
    let mut has_alpha = false;
    let mut has_signs = false;

    for c in s.chars() {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c.is_ascii_alphabetic() {
            has_alpha = true;
        } else if !c.is_ascii_alphanumeric() {
            has_signs = true;
        }

        if has_digit && has_alpha && has_signs {
            return true;
        }
    }

    false
}
