use cursive::default;

#[macro_use]
mod lib;

// Shortcut to import everything in the app.
use crate::all::*;
pub mod all {
  pub use crate::lib::app::data::*;
  pub use crate::lib::app::menu::*;
}

fn main() {
  app();
}

fn app() {
  // instantiate the main cursive runnable
  let mut siv = default();

  // add the basic data layer.
  siv.add_layer(AppData::new());

  // TODO: add other layers here.

  siv.add_fullscreen_layer(menu());

  // finally, run the app.
  siv.run();
}
