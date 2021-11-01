#![allow(dead_code)]

use crate::lib::{
  calculate_paycheck, menu::operations::Operations, report_wages,
};
use cursive::views::Dialog;
use cursive::Cursive;

#[macro_use]
mod lib;

fn main() -> anyhow::Result<()> {
  cursive_test()
}

fn cursive_test() -> anyhow::Result<()> {
  let mut siv = Cursive::default();
  siv.add_global_callback('q', |s| s.quit());
  siv.add_layer(
    Dialog::text(
      "This is a survey!\n\
  Press <Next> when you're ready.",
    )
    .title("Important survey")
    .button("Next", |s| {
      s.pop_layer();
      s.add_layer(
        Dialog::text("Did you do the thing?")
          .title("Question 1")
          .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
          .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
          .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))),
      )
    }),
  );

  siv.run();
  Ok(())
}

fn show_answer(s: &mut Cursive, msg: &str) {
  s.pop_layer();
  s.add_layer(
    Dialog::text(msg).title("Results").button("Finish", |s| s.quit()),
  );
}

fn preview(op: &Operations) -> String {
  match op {
    Operations::ReportWages => "Report Wages".to_string(),
    Operations::CalculatePaycheck => "Calculate your paycheck".to_string(),
  }
}

fn run_program() -> anyhow::Result<()> {
  // Print a menu to the user asking which operation will be done.
  let operation = Operations::show();

  // Operations:
  match operation {
    // - Report Wages
    Operations::ReportWages => report_wages::run(),
    // - Calculate Paycheck
    Operations::CalculatePaycheck => calculate_paycheck::run(),
  }
}
