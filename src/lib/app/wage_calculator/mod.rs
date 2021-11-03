pub mod date_box;
pub mod paycheck_calculator;
pub mod shift;

use crate::prelude::*;
pub use date_box::*;
pub use paycheck_calculator::*;
pub use shift::*;

// This creates the ui and adds it to the screen.
pub fn run(siv: &mut Cursive) {
  let calc_view = new_calculator_view(siv);
  siv.add_layer(calc_view);
  // This gets a date and stores it
  // get_date(siv);
}
