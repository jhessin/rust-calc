use wage_calculator::WageCalculator;

mod wage_calculator;

pub fn run() -> anyhow::Result<()> {
  println!("Running The Report Wages Module...");

  let month = prompt!("What month are you reporting?", String);

  let mut names = Vec::new();
  loop {
    let name = prompt!("Who's wages are you adding(.done when done)", String);
    if name.trim() == ".done" {
      break;
    }
    names.push(name.trim().to_string());
  }
  // lock down the names
  let names = names;

  println!("Your are reporting for {:?} in the month of {}", names, month);

  let _wages = WageCalculator::new(names, month);

  todo!("Complete this thing!")
}
