use chrono::prelude::*;

fn main() {
    let now = Local::now();
    println!("Hello, world!");
}

fn to_julian_date<Tz: TimeZone>(datetime: DateTime<Tz>) -> f64 {
    let (y, m) = if datetime.month() <= 2 {
        (datetime.year() as f64 - 1.0, datetime.month() as f64 + 12.0)
    } else {
        (datetime.year() as f64, datetime.month() as f64)
    };

    let mjd = (y * 365.25).floor() + (y / 400.0).floor() - (y / 100.0).floor()
        + ((m - 2.0) * 30.59).floor()
        + datetime.day() as f64
        - 678912.0;
}
