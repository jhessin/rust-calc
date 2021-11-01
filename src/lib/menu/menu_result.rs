use std::fmt;
use std::fmt::Formatter;

pub enum MenuData<T: Clone> {
  NewItem,
  Data(T),
  Quit,
}

pub struct MenuResult<T: Clone> {
  pub data: MenuData<T>,
  pub display: String,
  pub preview: String,
}

impl<T: Clone> MenuResult<T> {
  pub fn new(display: &str, preview: &str, data: MenuData<T>) -> Self {
    let (display, preview) = (String::from(display), String::from(preview));
    Self { data, display, preview }
  }
}

impl<T: Clone> fmt::Display for MenuResult<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.display)
  }
}
