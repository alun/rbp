use yew::{
  virtual_dom::{VList, VNode},
  Html,
};

pub mod ticker_input;
pub mod weights_calculator;

fn empty() -> Html {
  VNode::from(VList::new())
}

enum Key {
  UP = 38,
  DOWN = 40,
  ENTER = 13,
}
