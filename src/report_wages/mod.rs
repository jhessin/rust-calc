use std::io::{stdin, stdout, Write};

mod wage_calculator;

pub fn run() {
  println!("Running The Report Wages Module...");

  let month = prompt("What month are you reporting?");

  let mut names = Vec::new();
  loop {
    let name = prompt("Who's wages are you adding(.done when done)");
    if name.trim() == ".done" {
      break;
    }
    names.push(name.trim().to_string());
  }
  // lock down the names
  let names = names;

  println!("Your are reporting for {:?} in the month of {}", names, month);
}

fn prompt(prompt: &str) -> String {
  let mut response = String::new();

  print!("{}? ->", prompt);
  stdout().flush();
  stdin().read_line(&mut response).expect("Failed to read line");
  response
}
