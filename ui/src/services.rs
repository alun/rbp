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

pub mod rpb;

trait Service {
  fn get<'a, Out, QueryParams>(
    &self,
    path: &str,
    query_params: Option<&QueryParams>,
    callback: Callback<Result<Out>>,
  ) -> FetchTask
  where
    Out: DeserializeOwned + 'static,
    QueryParams: Serialize,
  {
    let uri =
      query_params.map(|params| format!("{}?{}", path, serde_qs::to_string(params).unwrap()));
    self.send(
      Method::GET,
      Nothing,
      uri.as_ref().map(String::as_str).unwrap_or(path),
      callback,
    )
  }

  fn post<'a, In, Out>(
    &self,
    path: &str,
    payload: &In,
    callback: Callback<Result<Out>>,
  ) -> FetchTask
  where
    In: Serialize,
    Out: DeserializeOwned + 'static,
  {
    self.send(Method::POST, Json(payload), path, callback)
  }

  fn delete<'a, In, Out>(
    &self,
    path: &str,
    payload: &In,
    callback: Callback<Result<Out>>,
  ) -> FetchTask
  where
    In: Serialize,
    Out: DeserializeOwned + 'static,
  {
    self.send(Method::DELETE, Json(payload), path, callback)
  }

  fn send<'a, In, Out>(
    &self,
    method: Method,
    body: In,
    uri: &str,
    callback: Callback<Result<Out>>,
  ) -> FetchTask
  where
    In: Into<Text>,
    Out: DeserializeOwned + 'static,
  {
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
      .uri(uri)
      .header("content-type", "application/json;charset=UTF-8")
      .body(body)
      .unwrap();
    FetchService::fetch(request, handler.into()).unwrap()
  }
}
