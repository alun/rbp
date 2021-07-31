mod actix_anyhow;

use crate::actix_anyhow::AnyhowErrorWrapper;
use actix_cors::Cors;
use actix_web::error;
use actix_web::{get, http, web::Json, App, HttpServer, Responder};
use anyhow::{anyhow, Context};
use listenfd::ListenFd;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::PyList;
use serde_qs::actix::QsQuery;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();
  log::info!("Starting server");

  let mut listenfd = ListenFd::from_env();

  let server = HttpServer::new(|| {
    App::new()
      .wrap(
        Cors::default()
          .allowed_origin(
            std::env::var("ALLOWED_ORIGIN")
              .as_ref()
              .map(String::as_str)
              .unwrap_or("http://localhost:8080"),
          )
          .allowed_methods(vec![
            http::Method::GET,
            http::Method::POST,
            http::Method::DELETE,
          ])
          .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
          ])
          .max_age(3600),
      )
      .service(get_weights)
      .service(get_search)
  });

  // if we are given a tcp listener on listen fd 0, we use that one
  let server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
    server.listen(listener)?
  // otherwise fall back to local listening
  } else {
    server.bind(("0.0.0.0", 9090))?
  };
  server.run().await
}

#[get("/service/v1/weights")]
async fn get_weights(query: QsQuery<core::GetWeightsQuery>) -> actix_web::Result<impl Responder> {
  let weights_result = Python::with_gil(|py| {
    calc_weights(py, query.tickers.iter().map(String::as_str).collect())
      .map_err(|e| AnyhowErrorWrapper::from(anyhow!("error calculating weigths: {}", e)))
  })?;

  match weights_result {
    Ok(weights) => Ok(Json(weights)),
    Err(anyhow_error) => Err(error::ErrorBadRequest(anyhow_error)),
  }
}

#[get("/service/v1/search")]
async fn get_search(query: QsQuery<core::SearchQuery>) -> actix_web::Result<impl Responder> {
  match find_tickers(&query).await {
    Ok(result) => Ok(Json(result)),
    Err(anyhow_error) => Err(error::ErrorBadRequest(anyhow_error)),
  }
}

async fn find_tickers(query: &core::SearchQuery) -> anyhow::Result<serde_json::Value> {
  let url = format!("https://finance.yahoo.com/_finance_doubledown/api/resource/searchassist;searchTerm={}?device=console&returnMeta=true", &query.term);
  let mut json: serde_json::Value = reqwest::get(&url)
    .await
    .with_context(|| format!("Can't fetch URL {}", &url))?
    .json()
    .await
    .with_context(|| "Can't parse json")?;
  // will return Value::Null null if not found in json
  let items = json["data"]["items"].take();
  Ok(items)
}

// TODO move to py_bridge
fn calc_weights(py: Python, tickers: Vec<&str>) -> PyResult<anyhow::Result<Vec<f64>>> {
  let sys = py.import("sys")?;
  sys.get("path")?.call_method(
    "extend",
    (vec![
      // TODO use env var for this
      "",
      "/Users/alunacharskii/.local/lib/python3.7/site-packages",
      "/opt/anaconda3/lib/python3.7/site-packages",
    ],),
    None,
  )?;

  // Default calculating of YTD date interval
  let datetime = py.import("datetime")?;
  let today = datetime.getattr("date")?.call_method("today", (), None)?;
  let current_year: u32 = today.getattr("year")?.extract()?;
  let year_ago = today.call_method(
    "replace",
    (),
    Some([("year", current_year - 1)].into_py_dict(py)),
  )?;

  let rpar = py.import("rpar")?;
  let prices = rpar.call_method("get_prices", (tickers, year_ago, today), None)?;

  let missing_data = rpar
    .call_method("find_tickers_with_missing_data", (prices,), None)?
    .downcast::<PyList>()?;

  Ok(if missing_data.is_empty() {
    let weights: Vec<f64> = rpar
      .call_method("get_weights", (prices,), None)?
      .extract()?;

    Ok(weights)
  } else {
    let tickers: Vec<&str> = missing_data.extract()?;
    Err(anyhow!("missing data for tickers {}", tickers.join(", ")))
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;

  #[actix_rt::test]
  async fn it_can_find_tickers() -> Result<()> {
    let tickers = find_tickers(&core::SearchQuery {
      term: "spy".to_string(),
    })
    .await?;
    println!("Response: {}", serde_json::to_string(&tickers)?);

    Ok(())
  }
}
