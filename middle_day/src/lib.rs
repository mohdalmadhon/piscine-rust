use chrono::{Datelike, NaiveDate, Weekday};

fn isLeapYear(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if isLeapYear(year) {
        return None;
    }

    let date = NaiveDate::from_ymd_opt(year as i32, 7, 2).unwrap();
    
    let w: Weekday = date.weekday();
    Some(w)
}
