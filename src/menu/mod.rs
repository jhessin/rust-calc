use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone)]
pub enum Operations {
  CalculatePaycheck,
  ReportWages,
}

impl Display for Operations {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Operations::CalculatePaycheck => write!(f, "Calculate Paycheck"),
      Operations::ReportWages => write!(f, "Report Wages"),
    }
  }
}

pub fn show() -> Operations {
  let options = [Operations::ReportWages, Operations::CalculatePaycheck];
  let mut menu = youchoose::Menu::new(options.iter()).preview(preview);

  let choices = menu.show();
  options[*choices.first().expect("Invalid choice given")]
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
