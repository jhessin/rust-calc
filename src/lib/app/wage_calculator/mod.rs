pub mod date_box;
pub mod shift;

use crate::prelude::*;
pub use date_box::*;
pub use shift::*;

// This creates the ui and adds it to the screen.
pub fn run(siv: &mut Cursive) {
  // This will get a date if we need one and store it in .
  get_date(siv);
}
