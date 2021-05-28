#![recursion_limit = "512"]

mod app;
mod components;
mod services;

use wasm_bindgen::prelude::*;
use web_sys::Window;

fn window() -> Window {
  web_sys::window().expect("Window must be present!")
}

fn origin() -> String {
  window()
    .location()
    .origin()
    .expect("Origin must be present!")
}

static LOG_LEVEL_KEY: &str = "log_level";

fn set_item(key: &str, value: &str) {
  if let Some(storage) = window().local_storage().expect("Local storage is missing!") {
    storage.set_item(key, value).expect("Can't set value");
  }
}

fn get_item(key: &str) -> Option<String> {
  if let Some(storage) = window().local_storage().expect("Local storage is missing!") {
    storage.get_item(key).expect("Can't get value")
  } else {
    None
  }
}

fn remove_item(key: &str) {
  if let Some(storage) = window().local_storage().expect("Local storage is missing!") {
    storage.remove_item(key).expect("Can't delete value");
  }
}

#[wasm_bindgen]
pub async fn run_app() -> Result<(), JsValue> {
  let log_level = match get_item(LOG_LEVEL_KEY).as_ref().map(String::as_str) {
    Some("debug") => log::Level::Debug,
    _ => log::Level::Info,
  };
  wasm_logger::init(wasm_logger::Config::new(log_level).module_prefix("rbp"));
  log::debug!("Debug is on");
  log::info!("Started app");
  yew::start_app_with_props::<app::App>(app::Props {});
  Ok(())
}
