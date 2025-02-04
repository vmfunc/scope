#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub struct Snowflake {
  pub content: u64,
}

impl ToString for Snowflake {
  fn to_string(&self) -> String {
    self.content.to_string()
  }
}
