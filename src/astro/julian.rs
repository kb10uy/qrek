//! Contains Julian day/century manipulations.

use chrono::prelude::*;

/// Converts Gregory datetime into julian date (JD).
pub fn to_julian_date<Tz: TimeZone>(datetime: &DateTime<Tz>) -> f64 {
    let datetime = datetime.naive_utc();

    let (y, m) = if datetime.month() <= 2 {
        (datetime.year() as f64 - 1.0, datetime.month() as f64 + 12.0)
    } else {
        (datetime.year() as f64, datetime.month() as f64)
    };

    let mjd = (y * 365.25).floor() + (y / 400.0).floor() - (y / 100.0).floor()
        + ((m - 2.0) * 30.59).floor()
        + datetime.day() as f64
        - 678912.0;

    let time = (datetime.hour() as f64 / 24.0)
        + (datetime.minute() as f64 / 1440.0)
        + (datetime.second() as f64 / 86400.0);
    let jd = mjd + 2400000.5 + time;

    jd
}

/// Converts Julian date (JD) into Gregory datetime.
pub fn from_julian_date(jd: f64) -> DateTime<Utc> {
    let mjd = jd - 2400000.5;
    let n = (mjd + 678881.0) as i32;
    let a = n * 4 + 3 + ((((n + 1) * 4 / 146097) + 1) * 3 / 4) * 4;
    let b = (a.rem_euclid(1461) / 4) * 5 + 2;

    let mut year = a / 1461;
    let mut month = b / 153 + 3;
    let day = b.rem_euclid(153) / 5 + 1;

    if month > 12 {
        year += 1;
        month -= 12;
    }

    let time = mjd.fract();
    let hour = (time * 24.0) as u32;
    let minute = (time * 1440.0) as u32 % 60;
    let second = (time * 86400.0) as u32 % 60;
    Utc.ymd(year, month as u32, day as u32)
        .and_hms(hour, minute, second)
}

/// Calculates Julian century from J2000.0.
pub fn julian_century(jd: f64) -> f64 {
    // JD2451545 is 2000/01/01 12:00:00
    (jd - 2451545.0) / 36525.0
}
