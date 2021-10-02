use crate::menu::Operations;

mod calculate_paycheck;
mod menu;
mod report_wages;

fn main() {
  // Print a menu to the user asking which operation will be done.
  let operation = menu::show();

  // Operations:
  match operation {
    // - Report Wages
    Operations::ReportWages => report_wages::run(),
    // - Calculate Paycheck
    Operations::CalculatePaycheck => calculate_paycheck::run(),
  }
}
