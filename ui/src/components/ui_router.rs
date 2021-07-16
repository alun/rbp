use crate::services::rpb::Service as RbpService;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;
use yewtil::ptr::Mrc;

#[derive(Clone, Debug, Switch)]
pub enum UiRoute {
  #[to = "/about"]
  About,
  #[to = "/404"]
  NotFound,
  #[to = "/!"]
  Main,
}
pub type UiRouter = Router<UiRoute>;
pub type UiRouteAnchor = RouterAnchor<UiRoute>;

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub rbp_service: Mrc<RbpService>,
}

pub struct Component {
  props: Props,
}

impl yew::Component for Component {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self { props }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let props = self.props.clone();
    let classes = classes!(
      "p-2",
      "rounded-lg",
      "hover:bg-white",
      "focus:outline-none",
      "focus:ring-2",
      "focus:ring-offset-2"
    );
    html! {
      <>
      <nav class="w-full text-center md:w-2/5 mx-auto pt-3">
        <ul class=classes!("active", "flex", "space-x-4", "items-center", "justify-center")>
          <UiRouteAnchor classes=classes.to_string() route=UiRoute::Main>
            { "Calculator" }
          </UiRouteAnchor>
          <UiRouteAnchor classes=classes.to_string() route=UiRoute::About>
            { "About" }
          </UiRouteAnchor>
        </ul>
      </nav>
      <div class="flex-grow w-full md:w-2/5 mx-auto p-3">
        <UiRouter
          render=UiRouter::render(move |ui_route| Self::switch(ui_route, &props))
          redirect=UiRouter::redirect(|_| {
            UiRoute::NotFound
          })
        />
      </div>
      </>
    }
  }
}

impl Component {
  fn switch(route: UiRoute, Props { ref rbp_service }: &Props) -> Html {
    let rbp_service = Mrc::clone(rbp_service);

    match route {
      UiRoute::Main => html! { <super::weights_calculator::Component rbp_service=rbp_service/> },
      // TODO make full page generated from markdown
      UiRoute::About => html! { <div>{"Based on quotes from Yahoo! Finance calculates \
        risk balanced portfolio weights using risk parity approach. "}<a target="blank" href="https://bit.ly/3yu2J9s"
        class=classes!("text-purple-400", "underline", "hover:text-purple-500")>{"Click for more info"}</a></div> 
      },
      UiRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
  }
}
