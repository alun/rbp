use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
pub struct GetWeightsQuery {
  pub tickers: Vec<String>,
}
