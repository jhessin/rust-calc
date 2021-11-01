use std::fmt;

#[derive(Copy, Clone)]
pub enum Operations {
  ReportWages,
  CalculatePaycheck,
}

impl Operations {
  pub const EACH: [Operations; 2] =
    [Operations::ReportWages, Operations::CalculatePaycheck];

  pub fn show() -> Operations {
    // let mut menu =
    //   youchoose::Menu::new(Operations::EACH.iter()).preview(preview);
    //
    // let choices = menu.show();
    // Operations::EACH[*choices.first().expect("Invalid choice given")]
    todo!("Show each operation in a menu")
  }
}

impl fmt::Display for Operations {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Operations::CalculatePaycheck => write!(f, "Calculate Paycheck"),
      Operations::ReportWages => write!(f, "Report Wages"),
    }
  }
}

fn preview(op: &Operations) -> String {
  match op {
    Operations::ReportWages => "\
    Report Wages:
      Calculate how much each of you made last month.
    "
    .to_string(),
    Operations::CalculatePaycheck => "\
    Calculate Paycheck:
      Calculate how much you should have made on your last paycheck.
    "
    .to_string(),
  }
}
