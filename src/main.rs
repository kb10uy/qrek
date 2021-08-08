mod astro;
mod tempo;

use std::env::args;

use anyhow::Result;
use chrono::prelude::*;

use tempo::TempoDate;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args: Vec<_> = args().collect();
    let date = if let Some(s) = args.get(1) {
        s.parse()?
    } else {
        Local::now()
    }
    .date();

    let tempo_date = TempoDate::from_gregory_date(date)?;

    println!("西暦: {}", date.naive_local());
    println!("旧暦: {} {}", tempo_date, tempo_date.rokuyo().to_japanese());

    Ok(())
}
