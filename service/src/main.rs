use anyhow::Result;
use pyo3::prelude::*;

fn main() -> Result<(), ()> {
  let tickers = vec![
    "MAGN.ME", "FIVE.ME", "MTSS.ME", "MRKP.ME", "OGKB.ME", "SNGSP.ME",
  ];
  let weights = Python::with_gil(|py| {
    calc_weights(py, tickers).map_err(|e| {
      // We can't display Python exceptions via std::fmt::Display,
      // so print the error here manually.
      e.print_and_set_sys_last_vars(py);
    })
  });
  println!("Weights {:?}", weights?);
  Ok(())
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
