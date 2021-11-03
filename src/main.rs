use cursive::default;

#[macro_use]
mod lib;

// Shortcut to import everything in the app.
use crate::prelude::*;
pub mod prelude {
  pub use crate::{
    lib::app::data::*, lib::app::menu::*, lib::app::wage_calculator::*,
  };
  pub use anyhow::*;
  pub use chrono::{
    Date, DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc,
  };
  pub use cursive::{menu::*, traits::*, views::*, Cursive};
  pub use cursive_calendar_view::*;
  pub use serde::{Deserialize, Serialize};
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
