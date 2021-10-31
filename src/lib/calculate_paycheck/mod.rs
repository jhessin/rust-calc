use anyhow::Context;

use crate::lib::{
  calculate_paycheck::paycheck_calculator::Shift,
  menu::menu_result::{MenuData, MenuResult},
};

mod paycheck_calculator;

pub fn run() -> anyhow::Result<()> {
  println!("Running Calculate Paycheck module");
  let mut calculator = paycheck_calculator::PaycheckCalculator::from_input();

  loop {
    let calc_menu = calculator.menu();
    let mut menu = youchoose::Menu::new(calc_menu.iter()).preview(preview);
    let choice = menu.show();
    let choice_index = choice.first().context("Invalid selection")?;
    let choice = calc_menu.get(*choice_index).context("Invalid choice")?;
    match choice.data {
      MenuData::Action(f) => {
        let shift = f();
        calculator += shift;
      }
      MenuData::Data(d) => {
        calculator.update_shift(*choice_index, d.update());
      }
      MenuData::Quit => break,
    }
  }
  Ok(())
}

fn preview(choice: &MenuResult<Shift>) -> String {
  choice.preview.clone()
}
