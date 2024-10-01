use error_types::*;

fn create_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("Invalid date format")
}


fn main() {
    let mut form_output = Form::new(
        String::from("Lee"),
        String::from("Silva"),
        create_date("2015-09-05"),
        String::from("Africa"),
        String::from("qwqwsa1dty_"),
    );

    println!("{:?}", form_output);
    println!("{:?}", form_output.validate().unwrap());

    form_output.first_name = String::from("");
    println!("{:?}", form_output.validate().unwrap_err());

    form_output.first_name = String::from("as");
    form_output.password = String::from("dty_1");
    println!("{:?}", form_output.validate().unwrap_err());

    form_output.password = String::from("asdasASd(_");
    println!("{:?}", form_output.validate().unwrap_err());

    form_output.password = String::from("asdasASd123SA");
    println!("{:?}", form_output.validate().unwrap_err());
}


// $ cargo run
// Form { first_name: "Lee", last_name: "Silva", birth: 2015-09-05, birth_location: "Africa", password: "qwqwsa1dty_" }
// ["Valid first name", "Valid password"]
// FormError { form_values: ("first_name", ""), date: "2022-10-17 12:09:25", err: "No user name" }
// FormError { form_values: ("password", "dty_1"), date: "2022-10-17 12:09:25", err: "At least 8 characters" }
// FormError { form_values: ("password", "asdasASd(_"), date: "2022-10-17 12:09:25", err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)" }
// FormError { form_values: ("password", "asdasASd123SA"), date: "2022-10-17 12:09:25", err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)" }
// $
