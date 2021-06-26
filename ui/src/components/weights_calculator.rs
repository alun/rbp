use super::ticker_input::Component as TickerInput;
use crate::services::rpb::Service as RbpService;
use crate::services::yahoo::TickerInfo;
use anyhow::Result;
use core::GetWeightsQuery;
use once_cell::sync::Lazy;
use serde_json::json;
use yew::services::Task;
use yew::virtual_dom::{VList, VNode};
use yew::{html, services::fetch::FetchTask, ComponentLink, Html, Properties, ShouldRender};
use yewtil::ptr::Mrc;

// TODO should this be constructed direcly in Rust?
static DEFAULT_TICKERS: Lazy<Vec<TickerInfo>> = Lazy::new(|| {
  serde_json::from_value(json!([
    {
      "symbol": "FB",
      "name": "Facebook, Inc.",
      "exch": "NGM",
      "exchDisp": "NASDAQ",
      "typeDisp": "Equity"
    },
    {
      "symbol": "AAPL",
      "name": "Apple Inc.",
      "exch": "NAS",
      "exchDisp": "NASDAQ",
      "typeDisp": "Equity"
    },
    {
      "symbol": "AMZN",
      "name": "Amazon.com, Inc.",
      "exch": "NMS",
      "exchDisp": "NASDAQ",
      "typeDisp": "Equity"
    },
    {
      "symbol": "NFLX",
      "name": "Netflix, Inc.",
      "exch": "NMS",
      "exchDisp": "NASDAQ",
      "typeDisp": "Equity"
    },
    {
      "symbol": "GOOG",
      "name": "Alphabet Inc.",
      "exch": "NGM",
      "exchDisp": "NASDAQ",
      "typeDisp": "Equity"
    }
  ]))
  .unwrap()
});

pub enum Msg {
  WeightsResultsLoaded(Result<Vec<f64>>),
  AddTicker(TickerInfo),
  SelectTicker(Option<TickerInfo>),
  DeleteSelectedTicker,
  ClearPortfolio,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub rbp_service: Mrc<RbpService>,
}

pub struct Component {
  get_weights_task: Option<FetchTask>,
  link: ComponentLink<Self>,
  fetched_tickers: Vec<TickerInfo>,
  picked_tickers: Vec<TickerInfo>,
  fetched_weights: Vec<f64>,
  fetching_error: Option<()>,
  selected_ticker: Option<TickerInfo>,
  props: Props,
}

mod portfolio_dao {
  use crate::services::yahoo::TickerInfo;
  use anyhow::{anyhow, Result};

  const PORTFOLIO_STORAGE_KEY: &str = "rbp.katlex.com.portfolio";

  fn parse_stored_portfolio(json: String) -> Result<Vec<TickerInfo>> {
    serde_json::from_str(&json).map_err(|_| anyhow!("Can't parse porfolio"))
  }

  pub fn load() -> Result<Vec<TickerInfo>> {
    crate::get_item(PORTFOLIO_STORAGE_KEY)
      .ok_or_else(|| anyhow!("No stored portfolio"))
      .and_then(parse_stored_portfolio)
  }

  pub fn save(portfolio: &Vec<TickerInfo>) {
    crate::set_item(
      PORTFOLIO_STORAGE_KEY,
      &serde_json::to_string(portfolio).unwrap(),
    );
  }
}

impl yew::Component for Component {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let tickers = portfolio_dao::load().unwrap_or_else(|_| DEFAULT_TICKERS.clone());

    let mut instance = Self {
      get_weights_task: None,
      link,
      props,
      picked_tickers: tickers.clone(),
      fetched_tickers: vec![],
      fetched_weights: vec![],
      fetching_error: None,
      selected_ticker: None,
    };
    if !tickers.is_empty() {
      instance.get_weigths();
    }
    log::debug!("Weigths calculator component created");

    instance
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::WeightsResultsLoaded(weights) => match weights {
        Ok(weights) => {
          self.fetching_error = None;
          self.fetched_weights = weights;
          portfolio_dao::save(&self.fetched_tickers);
        }
        Err(_) => {
          self.fetching_error = Some(());
        }
      },
      Msg::AddTicker(ticker_info) => {
        if !self.picked_tickers.contains(&ticker_info) {
          self.picked_tickers.push(ticker_info.clone());
          self.get_weigths();
        }
      }
      Msg::ClearPortfolio => {
        self.picked_tickers.clear();
        self.fetched_tickers.clear();
        portfolio_dao::save(&self.picked_tickers);
      }
      Msg::SelectTicker(maybe_ticker) => {
        self.selected_ticker = maybe_ticker;
      }
      Msg::DeleteSelectedTicker => {
        if let Some(selected) = self.selected_ticker.take() {
          self.picked_tickers.retain(|ticker| ticker != &selected)
        }
        self.get_weigths();
      }
    }
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    log::debug!("Weigths calculator properties changed");
    false
  }

  fn view(&self) -> Html {
    html! {
      <>
      <TickerInput on_ticker_added={self.link.callback(Msg::AddTicker)}/>
      {self.render_portfolio()}
      <div class="py-2">
        {self.render_selected_ticker_info()}
      </div>
      <div class="py-2">
        {self.build_weights_results()}
      </div>
      </>
    }
  }
}

impl Component {
  fn get_weigths(&mut self) {
    let query = GetWeightsQuery {
      tickers: self
        .picked_tickers
        .iter()
        .map(|ticker_info| ticker_info.symbol.to_string())
        .collect(),
    };
    self.fetched_tickers = self.picked_tickers.clone();
    self.get_weights_task = Some(
      self
        .props
        .rbp_service
        .as_ref()
        .get_weigths(query, self.link.callback(Msg::WeightsResultsLoaded)),
    );
  }

  fn build_weights_results(&self) -> Html {
    let render_ticker_weight = |(ticker_info, weight): (&TickerInfo, &f64)| {
      html! {
        <div>
          <span>{&ticker_info.symbol}</span>
          <span>{" = "}</span>
          <span>{format!("{:.2}%", 100f64 * weight)}</span>
        </div>
      }
    };

    if self.fetched_tickers.is_empty() {
      super::empty()
    } else if self
      .get_weights_task
      .as_ref()
      .map(Task::is_active)
      .unwrap_or(false)
    {
      html! {
        <div class="text-gray-500">{"Calculating weights..."}</div>
      }
    } else if self.fetching_error.is_some() {
      html! {
        <div class="text-red-500">{"Sorry, failed to caculate weights"}</div>
      }
    } else {
      html! {
      <>
        <div class="text-gray-500">{"Calculated porfolio weights"}</div>
        { for self.fetched_tickers.iter().zip(self.fetched_weights.iter()).map(render_ticker_weight) }
      </>
      }
    }
  }

  fn render_picked_tickers(&self) -> Html {
    let mut children: Vec<VNode> = vec![];

    for ticker_info in self.picked_tickers.iter() {
      let ticker_info_clone = ticker_info.clone();
      let onclick = self
        .link
        .callback(move |_| Msg::SelectTicker(Some(ticker_info_clone.clone())));
      children.push(
        html! {
          <button type="button"
            class="drop-shadow-md mr-2 mb-2 rounded-md border border-transparent shadow-sm px-4 py-2 bg-green-400 hover:bg-green-500 focus:ring-green-300 text-base font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2"
            onclick=onclick>
            { &ticker_info.symbol }
          </button>
        }
      );
    }
    VNode::VList(VList {
      children,
      key: None,
    })
  }

  fn render_portfolio(&self) -> Html {
    if self.picked_tickers.is_empty() {
      html! {
        <div class="text-gray-500">{"Add some tickers above"}</div>
      }
    } else {
      html! {
        <div class="mt-5">
          <div class="text-gray-500">{"Your portfolio"}</div>
          { self.render_picked_tickers() }
          <button class="drop-shadow-md mr-2 rounded-md border border-transparent shadow-sm px-4 py-2 bg-yellow-400 hover:bg-yellow-500 focus:ring-yellow-300 text-base font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 "
            onclick=self.link.callback(|_| Msg::ClearPortfolio)>
            <i class="fa fa-eraser" aria-hidden="true"></i>
          </button>
        </div>
      }
    }
  }

  fn render_selected_ticker_info(&self) -> Html {
    if self.selected_ticker.is_some() {
      let ticker = self.selected_ticker.as_ref().unwrap();

      // TODO extract modal frame / split inner component
      html! {
      <div class="fixed z-10 inset-0 overflow-y-auto">
        <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
          <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity p-2 sm:p-0">
            <div
              class="inline-block align-bottom sm:align-middle bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:max-w-lg w-full">
              <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                <div class="text-lg text-center">
                  {&ticker.symbol}
                </div>
                <div class="text-sm text-center">
                  {&ticker.name}
                </div>
                <div class="text-xs text-center text-gray-400">
                  {format!("{}/{}", &ticker.exch_disp, &ticker.type_disp)}
                </div>
              </div>
              <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
                <button type="button"
                  class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
                  onclick=self.link.callback(|_| Msg::DeleteSelectedTicker)>
                  { "Delete" }
                </button>
                <button type="button"
                  class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
                  onclick=self.link.callback(|_| Msg::SelectTicker(None))>
                  { "Ok" }
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      }
    } else {
      super::empty()
    }
  }
}
