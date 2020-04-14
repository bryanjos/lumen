use chrono::prelude::*;
use chrono::{NaiveDate, TimeZone};

pub fn get_local_now() -> [usize; 6] {
    datetime_to_array(Local::now())
}

pub fn get_utc_now() -> [usize; 6] {
    datetime_to_array(Utc::now())
}

pub fn convert_local_datetime_to_utc_datetime<Tz: TimeZone>(datetime: [usize; 6]) -> DateTime<Utc> {
    let d = TimeZone::ymd(now[0], now[1], now[2]).and_hms(now[3], now[4], now[5]).unwrap();

    DateTime<Utc>::from(d)
}

pub fn convert_utc_datetime_to_local_datetime<Tz: TimeZone>(datetime: [usize; 6]) -> DateTime<Local> {
    let d = Utc.ymd(now[0], now[1], now[2]).and_hms(now[3], now[4], now[5]).unwrap();

    DateTime::Local::from(d)
}

fn datetime_to_array<Tz: TimeZone>(datetime: DateTime<Tz>) -> [usize; 6] {
    [
        datetime.year() as usize,
        datetime.month() as usize,
        datetime.day() as usize,
        datetime.hour() as usize,
        datetime.minute() as usize,
        datetime.second() as usize,
    ]
}
