use core::GetWeightsQuery;

use anyhow::{anyhow, Context, Result};
use http::{request::Builder, Method};
use serde::{de::DeserializeOwned, Serialize};
use yew::{
  format::{Json, Nothing, Text},
  services::{
    fetch::{FetchTask, Response},
    FetchService,
  },
  Callback,
};

#[derive(PartialEq)]
pub struct RbpService {
  pub base: String,
}

impl RbpService {
  pub fn get_weigths(
    &self,
    query: GetWeightsQuery,
    callback: Callback<Result<Vec<f64>>>,
  ) -> FetchTask {
    self.get("weights", Some(&query), callback)
  }

  pub fn get<'a, OUT, QueryParams>(
    &self,
    path: &str,
    query_params: Option<&QueryParams>,
    callback: Callback<Result<OUT>>,
  ) -> FetchTask
  where
    OUT: DeserializeOwned,
    OUT: 'static,
    QueryParams: Serialize,
  {
    let full_path =
      query_params.map(|params| format!("{}?{}", path, serde_qs::to_string(params).unwrap()));
    self.send(
      Method::GET,
      Nothing,
      full_path.as_ref().map(String::as_str).unwrap_or(path),
      callback,
    )
  }

  pub fn post<'a, IN, OUT>(
    &self,
    path: &str,
    payload: &IN,
    callback: Callback<Result<OUT>>,
  ) -> FetchTask
  where
    IN: Serialize,
    OUT: DeserializeOwned,
    OUT: 'static,
  {
    self.send(Method::POST, Json(payload), path, callback)
  }

  pub fn delete<'a, IN, OUT>(
    &self,
    path: &str,
    payload: &IN,
    callback: Callback<Result<OUT>>,
  ) -> FetchTask
  where
    IN: Serialize,
    OUT: DeserializeOwned,
    OUT: 'static,
  {
    self.send(Method::DELETE, Json(payload), path, callback)
  }

  fn send<'a, IN, OUT>(
    &self,
    method: Method,
    body: IN,
    path: &str,
    callback: Callback<Result<OUT>>,
  ) -> FetchTask
  where
    IN: Into<Text>,
    OUT: DeserializeOwned,
    OUT: 'static,
  {
    let url = format!("{}/{}", self.base, path);
    let handler = move |response: Response<Text>| {
      let (meta, text) = response.into_parts();
      if meta.status.is_success() {
        callback.emit(text.and_then(|inner_text| {
          serde_json::from_str(&inner_text)
            .with_context(|| format!("Failed to parse json from {}", &inner_text))
        }));
      } else {
        callback.emit(Err(anyhow!("{}: error sending request", meta.status)))
      }
    };

    let request = Builder::new()
      .method(method)
      .uri(url.as_str())
      .header("content-type", "application/json;charset=UTF-8")
      .body(body)
      .unwrap();
    FetchService::fetch(request, handler.into()).unwrap()
  }
}
