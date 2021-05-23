mod services;

use anyhow::Result;
use services::RbpService;
use yew::virtual_dom::VNode;
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::{prelude::*, Switch};
use yewtil::ptr::Mrc;

use core::GetWeightsQuery;

#[derive(Switch, Clone, PartialEq)]
enum AppRoute {
  #[to = "/"]
  Main,
}

fn api_origin() -> String {
  let window_origin = super::origin();
  if window_origin.find("localhost").is_some() {
    // default config for dev environment
    "http://localhost:9090".to_string()
  } else {
    // using ingress via the same origin for qa/prod envs
    window_origin
  }
}

pub enum Msg {
  ChangeRoute(AppRoute),
  UpdateWeightsResults(Result<Vec<f64>>),
  RouteChanged(Route<()>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct App {
  rbp_service: Mrc<RbpService>,
  get_weights_task: Option<FetchTask>,
  link: ComponentLink<Self>,

  route: Route<()>,
  route_service: RouteService<()>,

  fetched_weights: Vec<f64>,
}

impl Component for App {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let mut route_service: RouteService<()> = RouteService::new();
    let route = route_service.get_route();
    route_service.register_callback(link.callback(Msg::RouteChanged));

    let mut app = App {
      rbp_service: Mrc::new(RbpService {
        base: format!("{}/service/v1", api_origin()).to_owned(),
      }),
      get_weights_task: None,
      link,
      route,
      route_service,
      fetched_weights: vec![],
    };
    log::info!("App created");
    app.get_weights(GetWeightsQuery {
      tickers: vec!["FB", "AAPL", "GOOGL"]
        .iter()
        .map(ToString::to_string)
        .collect(),
    });
    app
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
      Msg::UpdateWeightsResults(weights) => match weights {
        Ok(weights) => self.fetched_weights = weights,
        Err(_) => {} // TODO show error
      },
    }
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <>
      <div>
        { format!("Welcome to Risk Balanced Portfolio!") }
      </div>
      <nav>
        <div class="nav-wrapper">
          <ul class="left">
            // TODO use function component to render this
            <li class=self.active_if_route(AppRoute::Main)><a href="#" onclick=self.change_route(AppRoute::Main)>{ "Main" }</a></li>
          </ul>
        </div>
      </nav>
      <div class="container">
        { self.render_route() }
      </div>
      </>
    }
  }
}

impl App {
  fn get_weights(&mut self, request: GetWeightsQuery) {
    log::info!("Getting weights");
    self.get_weights_task = Some(self.rbp_service.get::<Vec<f64>, _>(
      "weights", // TODO move under service API
      Some(&request),
      self.link.callback(Msg::UpdateWeightsResults),
    ));
  }

  fn change_route(&self, app_route: AppRoute) -> Callback<MouseEvent> {
    self
      .link
      .callback(move |_| Msg::ChangeRoute(app_route.clone()))
  }

  fn render_route(&self) -> Html {
    match AppRoute::switch(self.route.clone()) {
      Some(AppRoute::Main) => {
        html! { format!("{:?}", self.fetched_weights) }
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
