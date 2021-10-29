#![allow(dead_code)]

use menu::operations::Operations;

mod calculate_paycheck;
mod menu;
mod report_wages;
mod util;
fn main() -> anyhow::Result<()> {
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
