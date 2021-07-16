use super::services::rpb::Service as RbpService;
use yew::{macros::html, prelude::*};
use yewtil::ptr::Mrc;

const VERSION: &'static str = std::include_str!("../version.txt");

fn api_origin() -> String {
  let window_origin = super::origin();
  if window_origin.find("localhost").is_some() || window_origin.find(".local.").is_some() {
    // default config for dev environment
    window_origin.replace("8080", "9090")
  } else {
    // using ingress via the same origin for qa/prod envs
    window_origin
  }
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct App {
  rbp_service: Mrc<RbpService>,
}

impl Component for App {
  type Message = Msg;
  type Properties = Props;

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      rbp_service: Mrc::new(RbpService {
        base: format!("{}/service/v1", api_origin()).to_owned(),
      }),
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <>
      <div class="w-full border-b border-blue-200 text-center p-3">
        <span class="text-xl">
          { format!("Welcome to Risk Balanced Portfolio!") }
        </span>
      </div>
      <super::components::ui_router::Component rbp_service=Mrc::clone(&self.rbp_service)/>
      <div class="w-full md:w-2/5 mx-auto px-3 text-center">
        <span class="text-xs text-gray-400">
          { "Made with ❤ in Dublin, Ireland 🍀" }
        </span>
        <span class="text-xs text-gray-300">
          { format!(" Version: {}", VERSION) }
        </span>
      </div>
      </>
    }
  }
}
