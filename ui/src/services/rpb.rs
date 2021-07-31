use anyhow::Result;
use core::{GetWeightsQuery, SearchQuery};
use serde::{Deserialize, Serialize};
use yew::{services::fetch::FetchTask, Callback};

#[derive(PartialEq)]
pub struct Service {
  pub base: String,
}

impl super::Service for Service {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TickerInfo {
  pub symbol: String,
  pub name: String,
  pub exch_disp: String,
  pub type_disp: String,
}

impl Service {
  pub fn get_weigths(
    &self,
    query: GetWeightsQuery,
    callback: Callback<Result<Vec<f64>>>,
  ) -> FetchTask {
    super::Service::get(self, &self.prepend_base("weights"), Some(&query), callback)
  }

  pub fn get_search(
    &self,
    query: SearchQuery,
    callback: Callback<Result<Vec<TickerInfo>>>,
  ) -> FetchTask {
    super::Service::get(self, &self.prepend_base("search"), Some(&query), callback)
  }

  // TODO this can be taken away with a new PathBuilder abstraction
  pub fn prepend_base(&self, sub_path: &str) -> String {
    format!("{}/{}", self.base, sub_path)
  }
}
