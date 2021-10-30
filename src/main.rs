#![allow(dead_code)]
use std::io;

use tui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  widgets::{Block, Borders},
  Terminal,
};

use crate::lib::{
  calculate_paycheck, menu::operations::Operations, report_wages,
};

#[macro_use]
mod lib;

fn main() -> anyhow::Result<()> {
  run_program()
}

fn tui_test() -> anyhow::Result<()> {
  let stdout = io::stdout();
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.draw(|f| {
    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .margin(1)
      .constraints(
        [
          Constraint::Percentage(10),
          Constraint::Percentage(80),
          Constraint::Percentage(10),
        ]
        .as_ref(),
      )
      .split(f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
  })?;
  Ok(())
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
