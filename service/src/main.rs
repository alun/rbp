mod actix_anyhow;

use actix_cors::Cors;
use actix_web::{get, http, web::Json, App, HttpServer, Responder};
use anyhow::anyhow;
use listenfd::ListenFd;
use pyo3::prelude::*;
use serde_qs::actix::QsQuery;

use crate::actix_anyhow::AnyhowErrorWrapper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

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
  let weights = Python::with_gil(|py| {
    calc_weights(py, query.tickers.iter().map(String::as_str).collect()).map_err(|e| {
      // We can't display Python exceptions via std::fmt::Display,
      // so print the error here manually.
      AnyhowErrorWrapper::from(anyhow!("error calculating weigths: {}", e))
    })
  })?;

  Ok(Json(weights))
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
