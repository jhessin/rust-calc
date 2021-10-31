use std::fmt;

use super::disp::DispFunc;

pub struct Item<'a> {
  pub icon: &'a str,
  pub chosen_icon: &'a str,
  pub chosen: bool,
  pub repr: String,
  pub preview: Option<String>,
}

impl<'a> Item<'a> {
  pub fn new(
    thing: &impl fmt::Display,
    icon: &'a str,
    chosen_icon: &'a str,
  ) -> Item<'a> {
    Item {
      icon,
      chosen_icon,
      chosen: false,
      repr: thing.to_string(),
      preview: None,
    }
  }

  pub fn select(&mut self) {
    self.chosen = !self.chosen;
  }

  pub fn chosen(&self) -> bool {
    self.chosen
  }

  pub fn icon(&self) -> &str {
    if self.chosen {
      self.chosen_icon
    } else {
      self.icon
    }
  }

  pub fn string(&self) -> &String {
    &self.repr
  }

  pub fn preview<D: fmt::Display>(&mut self, thing: D, func: &DispFunc<D>) {
    self.preview = Some(func.eval(thing));
  }
}

impl<'a> fmt::Display for Item<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format!("{} {}", self.icon(), self.repr))
  }
}
