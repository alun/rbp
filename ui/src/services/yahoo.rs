use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use yew::{services::fetch::FetchTask, Callback};

const AUTO_COMPLETE_URL: &str = "https://cors.katlex.com/http://d.yimg.com/aq/autoc";

#[derive(Serialize)]
pub struct SearchParams {
  query: String,
  lang: String,
}

impl From<&str> for SearchParams {
  fn from(query: &str) -> Self {
    SearchParams {
      query: query.to_string(),
      lang: "en-US".to_string(),
    }
  }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerInfo {
  pub symbol: String,
  pub name: String,
  pub exch: String,
  pub exch_disp: String,
  pub type_disp: String,
}

#[derive(PartialEq)]
pub struct Service {}

impl super::Service for Service {}

impl Service {
  pub fn search(
    &self,
    query: SearchParams,
    callback: Callback<Result<Vec<TickerInfo>>>,
  ) -> FetchTask {
    super::Service::get(
      self,
      AUTO_COMPLETE_URL,
      Some(&query),
      Callback::once(move |result: Result<serde_json::Value>| {
        callback.emit(transform_search_result(result))
      }),
    )
  }
}

fn transform_search_result(input: Result<serde_json::Value>) -> Result<Vec<TickerInfo>> {
  let value = input?;
  let value = value
    .get("ResultSet")
    .ok_or(anyhow!("Error getting result set: {:?}", &value))?;
  let value = value
    .get("Result")
    .ok_or(anyhow!("Error getting result: {:?}", &value))?;
  let tickers_info: Vec<TickerInfo> = serde_json::from_value(value.clone())
    .with_context(|| format!("Error parsing tickers info {:?}", value))?;
  Ok(tickers_info)
}
