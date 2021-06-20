use super::ticker_input::Component as TickerInput;
use crate::services::rpb::Service as RbpService;
use crate::services::yahoo::TickerInfo;
use anyhow::{anyhow, Result};
use core::GetWeightsQuery;
use once_cell::sync::Lazy;
use serde_json::json;
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
  TickerAdded(TickerInfo),
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
    };
    if !tickers.is_empty() {
      instance.get_weigths();
    }
    log::debug!("Weigths calculator component created");

    instance
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::WeightsResultsLoaded(weights) => {
        self.get_weights_task = None;
        match weights {
          Ok(weights) => {
            self.fetching_error = None;
            self.fetched_tickers = self.picked_tickers.clone();
            self.fetched_weights = weights;
            portfolio_dao::save(&self.picked_tickers);
          }
          Err(_) => {
            self.fetching_error = Some(());
          }
        }
      }
      Msg::TickerAdded(ticker_info) => {
        if !self.picked_tickers.contains(&ticker_info) {
          self.picked_tickers.push(ticker_info.clone());
          self.get_weigths();
        }
      }
      Msg::ClearPortfolio => {
        self.picked_tickers.clear();
        portfolio_dao::save(&self.picked_tickers);
      }
    }
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    log::debug!("Weigths calculator properties changed");
    false
  }

  fn view(&self) -> Html {
    let input_container_classes = || {
      let default_classes = vec!["relative h-10 input-component mt-5"];
      let result = vec![
        default_classes,
        if self.picked_tickers.len() > 0 {
          vec![]
        } else {
          vec!["empty"]
        },
      ]
      .concat();
      log::info!("Classes {:?}", result);
      result
    };
    html! {
      <>
      <TickerInput on_ticker_added={self.link.callback(Msg::TickerAdded)}/>
      <div class={input_container_classes()}>
        <input
          disabled=true
          type="text"
          name="tickers"
          class="h-full w-full border-gray-300 px-2 transition-all border-blue rounded-sm border"
          value={self.picked_tickers.iter().map(|ticker_info| ticker_info.symbol.to_string()).collect::<Vec<String>>().join(" ")}
          autocomplete="off" autocorrect="off" autocapitalize="off"
        />
        <label for="email" class="absolute left-0 -top-2 transition-all px-2 transform -translate-y-2/4
          text-xs text-blue-500">
          {"Your portfolio"}
        </label>
      </div>
      <div class="py-2">
        <button class="rounded-lg bg-yellow-300 filter drop-shadow-md p-2 active:bg-yellow-600 focus:outline-none"
          onclick=self.link.callback(|_| Msg::ClearPortfolio)>
          <img class="w-6 h-6 pointer-events-none" src="assets/clean.svg"/>
        </button>
      </div>
      <div class="py-2">
        {self.build_weights_results()}
      </div>
      </>
    }
  }
}

impl Component {
  // TODO can this be generated with a macro?
  fn get_weigths(&mut self) {
    let query = GetWeightsQuery {
      tickers: self
        .picked_tickers
        .iter()
        .map(|ticker_info| ticker_info.symbol.to_string())
        .collect(),
    };
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
          <span>{ticker_info.symbol.as_str()}</span>
          <span>{" = "}</span>
          <span>{format!("{:.2}%", 100f64 * weight)}</span>
        </div>
      }
    };

    if self.picked_tickers.is_empty() {
      html! {
        <div class="text-gray-500">{"Add some tickers above"}</div>
      }
    } else if self.get_weights_task.is_some() {
      html! {
        <div class="text-gray-500">{"Calculating weights..."}</div>
      }
    } else if self.fetching_error.is_some() {
      html! {
        <div class="text-red-500">{"Sorry, failed to caculated weights"}</div>
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
}
