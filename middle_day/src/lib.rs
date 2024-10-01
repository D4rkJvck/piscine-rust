pub use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    if is_leap_year(year) {
        None
    } else {
        let date = NaiveDate::from_ymd_opt(year, 7, 2).unwrap();
        let day = date.weekday();
        Some(day)
    }
}

////////////////////////////////////////////////////////////

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
