use super::components;
use super::services;
use services::RbpService;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::{prelude::*, Switch};
use yewtil::ptr::Mrc;

#[derive(Switch, Clone, PartialEq)]
pub enum AppRoute {
  #[to = "/"]
  Main,
}

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

pub enum Msg {
  ChangeRoute(AppRoute),
  RouteChanged(Route<()>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

// TODO move to components::ui_router::Component
pub struct App {
  rbp_service: Mrc<RbpService>,
  link: ComponentLink<Self>,

  route: Route<()>,
  route_service: RouteService<()>,
}

impl Component for App {
  type Message = Msg;
  type Properties = Props;

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let mut route_service: RouteService<()> = RouteService::new();
    let route = route_service.get_route();
    route_service.register_callback(link.callback(Msg::RouteChanged));

    let instance = Self {
      rbp_service: Mrc::new(RbpService {
        base: format!("{}/service/v1", api_origin()).to_owned(),
      }),
      link,
      route,
      route_service,
    };
    log::info!("App created");
    instance
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::RouteChanged(route) => self.route = route, // route changed from browser (externally)
      Msg::ChangeRoute(app_route) => {
        // route changed from app (internally)
        // this might be derived in the future
        self.route = app_route.into();
        self.route_service.set_route(&self.route.route, ());
      }
    }
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <>
      <div class="w-full border-b border-b border-blue-200 text-center p-3">
        <span class="text-xl">
          { format!("Welcome to Risk Balanced Portfolio!") }
        </span>

        // <nav>
        //   <div>
        //     <ul>
        //       // TODO use function component to render this
        //       <li class=self.active_if_route(AppRoute::Main)><a href="#" onclick=self.change_route(AppRoute::Main)>{ "Main" }</a></li>
        //     </ul>
        //   </div>
        // </nav>
      </div>
      <div class="w-full md:w-2/5 mx-auto p-3">
        { self.render_route() }
      </div>
      </>
    }
  }
}

impl App {
  fn change_route(&self, app_route: AppRoute) -> Callback<MouseEvent> {
    self
      .link
      .callback(move |_| Msg::ChangeRoute(app_route.clone()))
  }

  fn render_route(&self) -> Html {
    match AppRoute::switch(self.route.clone()) {
      Some(AppRoute::Main) => {
        html! { <components::weights_calculator::Component rbp_service={self.rbp_service.clone()} /> }
      }
      _ => VNode::from("404"),
    }
  }

  fn active_if_route(&self, app_route: AppRoute) -> Vec<&'static str> {
    AppRoute::switch(self.route.clone())
      .and_then(|route| {
        if route == app_route {
          Some(vec!["active"])
        } else {
          None
        }
      })
      .unwrap_or(vec![])
  }
}
