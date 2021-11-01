use std::fmt::Write;

use chrono::Datelike;

use crate::lib::calculate_paycheck::paycheck_calculator::shift::Shift;

use super::super::menu::menu_result::{MenuData, MenuResult};

pub mod shift;

#[derive(Clone, Debug)]
pub struct PaycheckCalculator {
  /// The Base rate that a person is paid.
  base_rate: f64,
  /// The weekend_diff that a person is paid.
  weekend_diff: f64,
  /// Any incentive that is given
  incentive_diff: f64,
  /// Any amount given for working short
  short_diff: f64,
  /// A list of all the shifts that were worked this pay period.
  shifts: Vec<Shift>,
  /// If there is a static year for this pay period (which is common) save it here.
  static_year: Option<i32>,
  /// If there is a static month for this pay period save it here.
  static_month: Option<u32>,
}

impl PaycheckCalculator {
  /// Get menu data for calculator
  pub fn menu(&mut self) -> Vec<MenuResult<Shift>> {
    let mut data: Vec<MenuResult<Shift>> = self
      .shifts
      .iter()
      .map(|shift| {
        MenuResult::new(
          &shift.clock_in.date().format("MM/DD").to_string(),
          &format!("{:?} - {:?}", shift.clock_in, shift.clock_out),
          MenuData::Data(*shift),
        )
      })
      .collect();
    data.push(MenuResult::new(
      "New shift",
      "Add a new shift",
      MenuData::NewItem,
    ));
    data.push(MenuResult::new(
      "Total",
      &self.calculate().unwrap_or_else(|_| {
        String::from(
          "Error \
		  calculating a total",
        )
      }),
      MenuData::Quit,
    ));
    data
  }

  pub fn update_shift(&mut self, index: usize, shift: Shift) -> bool {
    if let Some(entry) = self.shifts.get_mut(index) {
      *entry = shift.update(self.static_year, self.static_month);
      true
    } else {
      false
    }
  }
}

impl PaycheckCalculator {
  /// This will prompt the user for new data to build a Calculator.
  pub fn from_input() -> Self {
    let static_year: bool =
      prompt!("(true/false) This pay period is within a single year.");
    let static_year: Option<i32> =
      if static_year { Some(prompt!("What year?")) } else { None };
    let static_month: bool =
      prompt!("(true/false) This pay period is within a single month.");
    let static_month: Option<u32> =
      if static_month { Some(prompt!("What number month?")) } else { None };
    PaycheckCalculator {
      base_rate: prompt!("What is the base rate?"),
      weekend_diff: prompt!("How much more is paid on weekends?"),
      incentive_diff: prompt!("How much incentive is offered?"),
      short_diff: prompt!("How much is given when the shift is short?"),
      shifts: vec![],
      static_month,
      static_year,
    }
  }

  /// add a shift to the paycheck
  pub fn new_shift(&mut self) {
    self.shifts.push(Shift::from_input(self.static_year, self.static_month));
  }

  pub fn rm_shift(&mut self, shift: usize) -> Shift {
    self.shifts.remove(shift)
  }

  /// Generate a report for how much the paycheck should be.
  pub fn calculate(&self) -> anyhow::Result<String> {
    let mut result = String::new();
    let mut total = 0.0;
    for shift in &self.shifts {
      // get the shift len and multiply by wage
      let base = shift.len() * self.base_rate;

      result += &format!(
        "Total for {} the {} is ",
        shift.clock_in.weekday(),
        shift.clock_in.day()
      );
      let calculated = match (shift.incentive, shift.short) {
        (None, None) => {
          total += base;
          format!("{}\n", base)
        }
        (None, Some(short)) => {
          let short = short * self.short_diff;
          total += base + short;
          format!("{} base and {} short pay \n", base, short)
        }
        (Some(incentive), None) => {
          let incentive = incentive * self.incentive_diff;
          total += base + incentive;
          format!("{} base and {} incentive pay\n", base, incentive)
        }
        (Some(incentive), Some(short)) => {
          let incentive = incentive * self.incentive_diff;
          let short = short * self.short_diff;
          total += base + incentive + short;
          format!(
            "{} base, {} incentive, and {} short pay\n",
            base, incentive, short
          )
        }
      };
      result += &calculated;
    }
    result += "============================================\n";
    writeln!(result, "Total Earned this paycheck is {}", total)?;
    Ok(result)
  }
}

impl std::ops::AddAssign<Shift> for PaycheckCalculator {
  fn add_assign(&mut self, rhs: Shift) {
    self.shifts.push(rhs);
  }
}
