use std::fmt::Write;

use chrono::{DateTime, Datelike, Local, TimeZone};

use crate::lib::menu::menu_result::{MenuData, MenuResult};

#[derive(Copy, Clone, Debug)]
pub struct Shift {
  /// The time clocked in.
  clock_in: DateTime<Local>,
  /// The time clocked out.
  clock_out: DateTime<Local>,
  /// The amount of incentive time if any.
  incentive: Option<f64>,
  /// The amount of time working short if any.
  short: Option<f64>,
}

impl Shift {
  /// This will prompt the user for new shift data.
  pub fn from_input() -> Self {
    let year = prompt!("What year did you clock in?");
    let month = prompt!("Month?");
    let day = prompt!("Day?");
    let hour: f64 = prompt!("What hour did you clock in (with decimal)");
    let (hour, min, sec) =
      (hour.floor() as u32, (hour.fract() * 60.0) as u32, 0);
    let clock_in = Local.ymd(year, month, day).and_hms(hour, min, sec);
    let year = prompt!("What year did you clock out?");
    let month = prompt!("Month?");
    let day = prompt!("Day?");
    let hour: f64 = prompt!("What hour did you clock out (with decimal)");
    let (hour, min, sec) =
      (hour.floor() as u32, (hour.fract() * 60.0) as u32, 0);
    let clock_out = Local.ymd(year, month, day).and_hms(hour, min, sec);
    let incentive: bool = prompt!("Was incentive offered?");
    let incentive =
      if incentive { Some(prompt!("How many hours?")) } else { None };
    let short: bool = prompt!("Were you short during the shift?");
    let short = if short { Some(prompt!("How many hours?")) } else { None };
    Shift { clock_in, clock_out, incentive, short }
  }

  /// returns the duration of the shift in hours
  pub fn len(&self) -> f64 {
    let dur = self.clock_out - self.clock_in;
    (dur.num_hours() as f64) + (dur.num_minutes() as f64 / 60.0)
  }

  /// returns the amount of incentive earned
  pub fn incentive_len(&self) -> Option<f64> {
    self.incentive
  }

  /// returns the amount of short time earned
  pub fn short_len(&self) -> Option<f64> {
    self.short
  }

  /// Update shift data with user input
  pub fn update(&self) -> Self {
    todo!("Prompt user for updated info here")
  }
}

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
}

impl PaycheckCalculator {
  /// Get menu data for calculator
  pub fn menu(&self) -> Vec<MenuResult<Shift>> {
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
      MenuData::Action(Shift::from_input),
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

  pub fn update_shift(&mut self, index: usize, data: Shift) -> bool {
    if let Some(entry) = self.shifts.get_mut(index) {
      *entry = data;
      true
    } else {
      false
    }
  }
}

impl PaycheckCalculator {
  /// This will prompt the user for new data to build a Calculator.
  pub fn from_input() -> Self {
    PaycheckCalculator {
      base_rate: prompt!("What is the base rate?"),
      weekend_diff: prompt!("How much more is paid on weekends?"),
      incentive_diff: prompt!("How much incentive is offered?"),
      short_diff: prompt!("How much is given when the shift is short?"),
      shifts: vec![],
    }
  }

  /// add a shift to the paycheck
  pub fn new_shift(&mut self) {
    self.shifts.push(Shift::from_input());
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
