mod actix_anyhow;

use actix_web::{get, web::Json, App, HttpRequest, HttpServer, Responder};
use anyhow::{anyhow, Result};
use listenfd::ListenFd;
use pyo3::prelude::*;
use qstring::QString;

use crate::actix_anyhow::AnyhowErrorWrapper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let mut listenfd = ListenFd::from_env();

  let server = HttpServer::new(|| App::new().service(get_weights));

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
async fn get_weights(req: HttpRequest) -> actix_web::Result<impl Responder> {
  let query_string = QString::from(req.query_string());

  let tickers: Vec<String> = require_param(&query_string, "tickers", |value| {
    value.split(",").map(str::to_string).collect()
  })
  .map_err(AnyhowErrorWrapper::from)?;

  let weights = Python::with_gil(|py| {
    calc_weights(py, tickers.iter().map(String::as_str).collect()).map_err(|e| {
      // We can't display Python exceptions via std::fmt::Display,
      // so print the error here manually.
      println!("{:?}", "Error");
      AnyhowErrorWrapper::from(anyhow!("error calculating weigths: {}", e))
    })
  })?;

  Ok(Json(weights))
}

fn require_param<T, F>(query_string: &QString, param: &str, mapping: F) -> Result<T>
where
  F: Fn(&str) -> T,
{
  if let Some(param_value) = query_string.get(param) {
    Ok(mapping(param_value))
  } else {
    Err(anyhow!("param is required: {}", param))
  }
}

fn calc_weights(py: Python, tickers: Vec<&str>) -> PyResult<Vec<f64>> {
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

  let datetime = py.import("datetime")?;
  let start_date = datetime.call_method("datetime", (2018, 4, 17), None)?;
  let end_date = datetime.call_method("datetime", (2021, 4, 14), None)?;

  let rpar = py.import("rpar")?;
  let prices = rpar.call_method("get_prices", (tickers, start_date, end_date), None)?;
  let weights: Vec<f64> = rpar
    .call_method("get_weights", (prices,), None)?
    .extract()?;

  Ok(weights)
}
