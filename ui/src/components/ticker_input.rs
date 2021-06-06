use crate::services::yahoo::{self, TickerInfo};
use anyhow::Result;
use web_sys::{FocusEvent, KeyboardEvent};
use yew::{
  html, services::fetch::FetchTask, Callback, ComponentLink, Html, InputData, Properties,
  ShouldRender,
};
pub enum Msg {
  AutoCompleteResutlsLoaded(Result<Vec<TickerInfo>>),
  InputChaging(InputData),
  KeyDown(KeyboardEvent),
  FocusOut(FocusEvent),
  SelectOption(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub on_ticker_added: Callback<TickerInfo>,
}

pub struct Component {
  fetched_tickers: Vec<TickerInfo>,
  link: ComponentLink<Self>,
  props: Props,
  query_auto_complete_task: Option<FetchTask>,
  value: String,
  yahoo_service: yahoo::Service,
  selected_option: i32,
}

impl yew::Component for Component {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let mut _self = Self {
      fetched_tickers: vec![],
      link,
      props,
      query_auto_complete_task: None,
      value: "".to_string(),
      yahoo_service: yahoo::Service {},
      selected_option: -1,
    };

    _self.value = "FB".to_string();
    _self.query_auto_complete_task = Some(_self.yahoo_service.search(
      _self.value.as_str().into(),
      _self.link.callback(Msg::AutoCompleteResutlsLoaded),
    ));
    _self
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AutoCompleteResutlsLoaded(Ok(new_fetched_tickers)) => {
        self.query_auto_complete_task = None;
        self.selected_option = -1;
        log::debug!("Auto complete options {:?}", &new_fetched_tickers);
        self.fetched_tickers = new_fetched_tickers;
      }
      Msg::AutoCompleteResutlsLoaded(_) => {}
      Msg::InputChaging(InputData { value }) => {
        log::debug!("Input changing {:?}", &value);
        self.value = value;
        self.query_auto_complete_task = Some(self.yahoo_service.search(
          self.value.as_str().into(),
          self.link.callback(Msg::AutoCompleteResutlsLoaded),
        ));
      }
      Msg::KeyDown(keyboard_event) => {
        let mut prevent_default = true;
        log::debug!("Key pressed {}", keyboard_event.key_code());
        match keyboard_event.key_code() {
          // TODO could be done with macro?
          x if x == super::Key::DOWN as u32 => self.move_selected_option(1),
          x if x == super::Key::UP as u32 => self.move_selected_option(-1),
          x if x == super::Key::ENTER as u32 => {
            self.use_selected_option();
          }
          _ => prevent_default = false,
        }
        if prevent_default {
          keyboard_event.prevent_default();
        }
      }
      Msg::FocusOut(_) => {
        self.fetched_tickers = vec![];
        self.selected_option = -1;
      }
      Msg::SelectOption(index) => {
        self.selected_option = index as i32;
        self.use_selected_option();
      }
    }
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let input_container_classes = || {
      let mut result = vec!["relative h-10 input-component mt-5"];
      if self.value.is_empty() {
        result.push("empty")
      }
      result
    };
    html! {
      <div class="relative">
        <div class={input_container_classes()}>
          <input type="text" name="tickers"
            class="h-full w-full border-gray-300 px-2 transition-all border-blue rounded-sm border"
            oninput=self.link.callback(Msg::InputChaging) onkeydown=self.link.callback(Msg::KeyDown)
            onblur=self.link.callback(Msg::FocusOut) autocomplete="off" autocorrect="off" autocapitalize="off"
            value=self.value />
          <label for="email" class="absolute left-0 -top-2 transition-all px-2 transform -translate-y-2/4
            text-xs text-blue-500 pointer-events-none">
            {"Add ticker"}
          </label>
        </div>
        { self.auto_complete_options() }
      </div>
    }
  }
}

impl Component {
  fn auto_complete_options(&self) -> Html {
    let render_ticker_option = |(index, ticker_info): (usize, &TickerInfo)| {
      let mut classes = vec!["px-2", "py-1", "mx-1", "flex", "flex-row", "cursor-pointer"];
      if index as i32 == self.selected_option {
        classes.push("bg-blue-200");
      } else {
        classes.push("bg-blue-50");
      }
      html! {
        <li class=classes onclick=self.link.callback(move |_| Msg::SelectOption(index))><span
            class="flex-grow">{&ticker_info.symbol}</span><span
            class="text-gray-500 text-xs pt-1">{&ticker_info.name}</span></li>
      }
    };
    if !self.fetched_tickers.is_empty() {
      html! {
        <ul class="absolute z-10 w-full">
          { for self.fetched_tickers.iter().enumerate().map(render_ticker_option) }
        </ul>
      }
    } else {
      super::empty()
    }
  }

  fn move_selected_option(&mut self, delta: i32) {
    self.selected_option += delta;
    if self.fetched_tickers.len() == 0 {
      self.selected_option = -1;
    } else {
      self.selected_option %= self.fetched_tickers.len() as i32;
    }
    self.value = self.fetched_tickers[self.selected_option as usize]
      .symbol
      .clone();
  }

  fn use_selected_option(&mut self) {
    if self.selected_option >= 0 {
      let ticker_info = self.fetched_tickers.remove(self.selected_option as usize);
      self.fetched_tickers = vec![];
      self.value = "".to_string();
      self.props.on_ticker_added.emit(ticker_info);
    }
  }
}
