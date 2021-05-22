#[derive(Debug)]
pub struct AnyhowErrorWrapper {
  err: anyhow::Error,
}

impl std::fmt::Display for AnyhowErrorWrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!("{:?}", self.err))
  }
}
impl From<anyhow::Error> for AnyhowErrorWrapper {
  fn from(err: anyhow::Error) -> AnyhowErrorWrapper {
    AnyhowErrorWrapper { err }
  }
}
impl actix_web::ResponseError for AnyhowErrorWrapper {}
