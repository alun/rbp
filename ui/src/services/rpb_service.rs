use super::Service;
use anyhow::Result;
use core::GetWeightsQuery;
use yew::{services::fetch::FetchTask, Callback};

#[derive(PartialEq)]
pub struct RbpService {
  pub base: String,
}

impl Service for RbpService {}

impl RbpService {
  pub fn get_weigths(
    &self,
    query: GetWeightsQuery,
    callback: Callback<Result<Vec<f64>>>,
  ) -> FetchTask {
    Service::get(self, &self.prepend_base("weights"), Some(&query), callback)
  }

  // TODO this can be taken away with a new PathBuilder abstraction
  pub fn prepend_base(&self, sub_path: &str) -> String {
    format!("{}/{}", self.base, sub_path)
  }
}
