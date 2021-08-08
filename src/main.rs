mod astro;
mod tempo;

use anyhow::Result;
use async_std::prelude::*;
use chrono::prelude::*;
use log::error;
use serde::Deserialize;
use serde_json::json;
use tide::{Request, Response, Result as TideResult, StatusCode};

use tempo::TempoDate;

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let ctrlc = async {
        async_ctrlc::CtrlC::new()
            .expect("Handler creation failed")
            .await;
        Ok(())
    };

    let app = async {
        let mut app = tide::new();
        app.at("/tempo_date").get(get_tempo_date);
        app.listen("0.0.0.0:8000").await
    };
    app.race(ctrlc).await?;
    Ok(())
}

/// GET `/tempo_date`
async fn get_tempo_date(request: Request<()>) -> TideResult {
    #[derive(Debug, Clone, Deserialize)]
    struct QueryParameters {
        date: String,
    }

    let query: QueryParameters = request.query()?;
    let src_str = format!("{}T00:00:00+09:00", query.date);
    let datetime = match DateTime::parse_from_str(&src_str, "%+") {
        Ok(dt) => dt,
        Err(e) => {
            error!("DateTime parse error: {}", e);
            return Err(e.into());
        }
    };
    let date = datetime.date();
    let tempo_date = TempoDate::from_gregory_date(date)?;

    let body = json!({
        "date_str": datetime,
        "tempo_date_str": tempo_date.to_string(),
        "tempo_date": {
            "year": tempo_date.year,
            "month": tempo_date.month,
            "day": tempo_date.day,
            "leap_month": tempo_date.leap_month,
            "rokuyo_index": tempo_date.rokuyo().to_number(),
            "rokuyo_str": tempo_date.rokuyo().to_japanese(),
        }
    });
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}
