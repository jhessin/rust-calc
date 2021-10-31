use super::item::Item;

pub struct MenuState<'a> {
  pub hover: usize,
  pub start: usize,
  pub items: Vec<Item<'a>>,
}

pub struct MenuConfig {
  pub multiselect: bool,
}
