use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rokuyo {
    Taian,
    Shakku,
    Sensho,
    Tomobiki,
    Sempu,
    Butsumetsu,
}

impl Rokuyo {
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

    pub fn to_number(self) -> usize {
        match self {
            Rokuyo::Taian => 0,
            Rokuyo::Shakku => 1,
            Rokuyo::Sensho => 2,
            Rokuyo::Tomobiki => 3,
            Rokuyo::Sempu => 4,
            Rokuyo::Butsumetsu => 5,
        }
    }

    pub fn from_number(index: usize) -> Result<Rokuyo> {
        match index {
            0 => Ok(Rokuyo::Taian),
            1 => Ok(Rokuyo::Shakku),
            2 => Ok(Rokuyo::Sensho),
            3 => Ok(Rokuyo::Tomobiki),
            4 => Ok(Rokuyo::Sempu),
            5 => Ok(Rokuyo::Butsumetsu),
            _ => bail!("Out of rokuyo index"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Tempo {
    year: usize,
    leap: bool,
    month: usize,
    day: usize,
}

fn calculate_leading_nishinibun(jd: f64) {
    let jd_dec = jd.floor();
    let jd_fract = jd.fract() - 9.0 / 24.0;

    let t = (jd_dec - 2451545.0) / 36525.0 + (jd_fract + 0.5) / 36525.0;
    let l_sun0 = sun_longitude()
}
