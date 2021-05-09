mod actix_anyhow;

use actix_anyhow::ApiResult;
use actix_web::{get, web::Json, App, HttpRequest, HttpServer, Responder};
use listenfd::ListenFd;
use pyo3::prelude::*;

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
    server.bind(("0.0.0.0", 8080))?
  };
  server.run().await
}

#[get("/service/v1/weights")]
async fn get_weights(req: HttpRequest) -> ApiResult<impl Responder> {
  // TODO call python
  let res: Vec<i32> = vec![];
  Ok(Json(res))
}

// fn main() -> Result<(), ()> {
// let tickers = vec![
//   "MAGN.ME", "FIVE.ME", "MTSS.ME", "MRKP.ME", "OGKB.ME", "SNGSP.ME",
// ];
// let weights = Python::with_gil(|py| {
//   calc_weights(py, tickers).map_err(|e| {
//     // We can't display Python exceptions via std::fmt::Display,
//     // so print the error here manually.
//     e.print_and_set_sys_last_vars(py);
//   })
// });
// println!("Weights {:?}", weights?);
//   Ok(())
// }

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
