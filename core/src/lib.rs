#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
pub struct GetWeightsQuery {
  pub tickers: Vec<String>,
}

#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
pub struct SearchQuery {
  pub term: String,
}
