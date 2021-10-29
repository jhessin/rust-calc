use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct MenuResult<T: Clone> {
  pub data: T,
  pub display: String,
  pub preview: String,
}

impl<T: Clone> MenuResult<T> {
  pub fn new(display: &str, preview: &str, data: &T) -> Self {
    let (display, preview, data) =
      (String::from(display), String::from(preview), data.clone());
    Self { data, display, preview }
  }
}

impl<T: Clone> fmt::Display for MenuResult<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.display)
  }
}
