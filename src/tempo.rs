use std::fmt::{Display, Formatter, Result as FmtResult};

use anyhow::{bail, Result};
use chrono::prelude::*;

use crate::astro::{
    julian::{from_julian_date, to_julian_date},
    longitude::jcg78::{moon_longitude, sun_longitude},
};

/// Represents rokuyo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rokuyo {
    Taian,
    Shakku,
    Sensho,
    Tomobiki,
    Sempu,
    Butsumetsu,
}

#[allow(dead_code)]
impl Rokuyo {
    /// Gets Japanese string.
    pub fn to_japanese(self) -> &'static str {
        match self {
            Rokuyo::Taian => "大安",
            Rokuyo::Shakku => "赤口",
            Rokuyo::Sensho => "先勝",
            Rokuyo::Tomobiki => "友引",
            Rokuyo::Sempu => "先負",
            Rokuyo::Butsumetsu => "仏滅",
        }
    }

    /// Converts into numeral index.
    pub fn to_number(self) -> usize {
        match self {
            Rokuyo::Sensho => 0,
            Rokuyo::Tomobiki => 1,
            Rokuyo::Sempu => 2,
            Rokuyo::Butsumetsu => 3,
            Rokuyo::Taian => 4,
            Rokuyo::Shakku => 5,
        }
    }

    /// Converts from numeral index.
    pub fn from_number(index: usize) -> Result<Rokuyo> {
        match index {
            0 => Ok(Rokuyo::Sensho),
            1 => Ok(Rokuyo::Tomobiki),
            2 => Ok(Rokuyo::Sempu),
            3 => Ok(Rokuyo::Butsumetsu),
            4 => Ok(Rokuyo::Taian),
            5 => Ok(Rokuyo::Shakku),
            _ => bail!("Out of rokuyo index"),
        }
    }
}

/// Represents a tempo calendar date.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TempoDate {
    pub year: usize,
    pub leap_month: bool,
    pub month: usize,
    pub day: usize,
    pub jd: f64,
}

impl Default for TempoDate {
    fn default() -> Self {
        TempoDate {
            year: 1,
            leap_month: false,
            month: 1,
            day: 1,
            jd: 0.0,
        }
    }
}

impl Display for TempoDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:04}/", self.year)?;
        if self.leap_month {
            write!(f, "L")?;
        }
        write!(f, "{:02}/{:02}", self.month, self.day)?;

        Ok(())
    }
}

impl TempoDate {
    /// Converts into tempo calendar date.
    pub fn from_gregory_date<Tz: TimeZone>(jst_date: Date<Tz>) -> Result<TempoDate> {
        let jd = to_julian_date(&jst_date.and_hms(0, 0, 0));
        let jd_date = to_julian_date(&from_julian_date(jd + 0.375).date().and_hms(0, 0, 0));

        // 1. Calculate 24-sekkis -------------------------------------------------

        // 1-a. Start from current date
        let mut sekkis = vec![];
        let mut last_sekki = calculate_leading_24sekki(jd);
        sekkis.push(last_sekki);

        // 1-b. Calculate 24-sekkis back to last toji
        while last_sekki.1 as usize / 15 != 18 {
            // Why 13.0? It could be 1.0.
            let prev_sekki = calculate_leading_24sekki(last_sekki.0 - 13.0);
            sekkis.insert(0, prev_sekki);
            last_sekki = prev_sekki;
        }

        // 1-c. Calculate 24-sekkis forward to next usui
        last_sekki = *sekkis.last().expect("Should be have at 1 element");
        while last_sekki.1 as usize / 15 != 22 {
            // Why 18.0?
            let next_sekki = calculate_leading_24sekki(last_sekki.0 + 18.0);
            sekkis.push(next_sekki);
            last_sekki = next_sekki;
        }

        // 2. Calculate sakus -----------------------------------------------------

        // 2-a. Start from current date
        let mut sakus = vec![];
        let mut last_saku = calculate_leading_saku(jd)?;
        sakus.push(last_saku);

        // 2-b. Calculate sakus back to last toji
        let jd_toji = sekkis.first().expect("Should have 24 elements").0;
        while last_saku > jd_toji {
            let prev_saku = calculate_leading_saku(last_saku - 27.0)?;
            sakus.insert(0, prev_saku);
            last_saku = prev_saku;
        }

        // 2-c. Calculate sakus forward to next usui
        last_saku = *sakus.last().expect("Should be have at 1 element");
        let jd_usui = sekkis.last().expect("Should have 24 elements").0;
        while last_saku < jd_usui {
            let mut next_saku = calculate_leading_saku(last_saku + 30.0)?;
            if (next_saku - last_saku).abs() < 26.0 {
                next_saku = calculate_leading_saku(last_saku + 35.0)?;
            }
            sakus.push(next_saku);
            last_saku = next_saku;
        }

        // 3. Correspond chuki and sakus ------------------------------------------
        let chukis: Vec<_> = sekkis
            .iter()
            .filter(|x| x.1 as usize % 30 == 0)
            .copied()
            .collect();
        let mut tempo_months = vec![TempoDate::default(); sakus.len() - 1];
        for (saku, tempo) in sakus.windows(2).zip(&mut tempo_months) {
            let (saku_start, saku_end) = (
                from_julian_date(saku[0] + 0.375).date(),
                from_julian_date(saku[1] + 0.375).date(),
            );

            let corresponding_chuki = chukis.iter().find(|chuki| {
                let chuki_date = from_julian_date(chuki.0 + 0.375).date();
                (saku_start..saku_end).contains(&chuki_date)
            });
            match corresponding_chuki {
                Some((_, l)) => {
                    tempo.month = match *l as usize / 30 {
                        0 => 2,
                        3 => 5,
                        6 => 8,
                        9 => 11,
                        otherwise => (otherwise + 1) % 12 + 1,
                    };
                    tempo.leap_month = false;
                    tempo.jd = to_julian_date(&saku_start.and_hms(0, 0, 0));
                }
                None => {
                    tempo.month = 0;
                    tempo.leap_month = true;
                    tempo.jd = to_julian_date(&saku_start.and_hms(0, 0, 0));
                }
            }
        }

        for i in 1..(tempo_months.len()) {
            if tempo_months[i].leap_month {
                tempo_months[i].month = tempo_months[i - 1].month;
            }
        }

        let target_month = tempo_months
            .iter()
            .filter(|m| jd_date >= m.jd)
            .last()
            .expect("Should be found");
        let mut tempo_date = *target_month;
        tempo_date.day = (jd_date - tempo_date.jd) as usize + 1;
        tempo_date.year = match jst_date.year() {
            y if tempo_date.month >= 10 && tempo_date.month > jst_date.month() as usize => {
                y as usize - 1
            }
            otherwise => otherwise as usize,
        };
        Ok(tempo_date)
    }

    /// Gets rokuyo.
    pub fn rokuyo(&self) -> Rokuyo {
        Rokuyo::from_number((self.month + self.day - 2) % 6).expect("Should be rounded by 6")
    }
}

/// Calculates leading 24-sekki with Julian Date.
pub fn calculate_leading_24sekki(jd_now: f64) -> (f64, f64) {
    let l_sun_now = sun_longitude(jd_now);
    let l_sun0 = 15.0 * (l_sun_now / 15.0).floor();

    let mut delta_t = 1.0f64;
    let mut jd = jd_now;
    while delta_t.abs() > (1.0 / 86400.0) {
        let l_sun = sun_longitude(jd);
        let delta_l = match l_sun - l_sun0 {
            x if x > 180.0 => x - 360.0,
            x if x < -180.0 => x + 360.0,
            otherwise => otherwise,
        };

        delta_t = delta_l * 365.2 / 360.0;
        jd -= delta_t;
    }

    (jd, l_sun0)
}

/// Calculates saku chuki with Julian Date.
pub fn calculate_leading_saku(jd_now: f64) -> Result<f64> {
    let mut delta_t = 1.0f64;
    let mut jd = jd_now;
    let mut iter_count = 0;
    while delta_t.abs() > (1.0 / 86400.0) {
        let l_sun = sun_longitude(jd);
        let l_moon = moon_longitude(jd);
        let mut delta_l = l_moon - l_sun;

        if iter_count == 0 && delta_l < 0.0 {
            delta_l = delta_l.rem_euclid(360.0);
        }

        if (0.0..20.0).contains(&l_sun) && l_moon >= 300.0 {
            delta_l = 360.0 - delta_l.rem_euclid(360.0);
        }

        if delta_l.abs() > 40.0 {
            delta_l = delta_l.rem_euclid(360.0);
        }

        delta_t = delta_l * 29.530589 / 360.0;
        jd -= delta_t;

        if iter_count >= 30 {
            bail!("Saku calculation cannot be finished");
        } else if iter_count == 15 {
            jd = jd_now - 26.0;
        }
        iter_count += 1;
    }

    Ok(jd)
}
