use anyhow::Context;

use paycheck_calculator::shift::Shift;

use super::menu::menu_result::{MenuData, MenuResult};

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
      MenuData::NewItem => {
        calculator.new_shift();
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
