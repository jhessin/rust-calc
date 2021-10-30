use std::fmt::Write;
use std::iter::Map;
use std::ops::{Deref, DerefMut};
use std::slice::Iter;

use chrono::{DateTime, Datelike, Local};

use crate::menu::menu_result::MenuResult;

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
    // TODO:
    Shift {
      clock_in: Local::now(),
      clock_out: Local::now(),
      incentive: None,
      short: None,
    }
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

impl Deref for PaycheckCalculator {
  type Target = Vec<Shift>;

  fn deref(&self) -> &Self::Target {
    &self.shifts
  }
}

impl DerefMut for PaycheckCalculator {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.shifts
  }
}

type ShiftIter<'a> = Map<Iter<'a, Shift>, fn(&Shift) -> MenuResult<Shift>>;

impl PaycheckCalculator {
  /// This will prompt the user for new data to build a Calculator.
  pub fn from_input() -> Self {
    // TODO:
    PaycheckCalculator {
      base_rate: 0.0,
      weekend_diff: 0.0,
      incentive_diff: 0.0,
      short_diff: 0.0,
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

  /// Generate an iterator
  pub fn iter(&self) -> ShiftIter {
    self.shifts.iter().map(|shift| {
      MenuResult::new(
        &shift.clock_in.date().format("MM/DD").to_string(),
        &format!("{:?} - {:?}", shift.clock_in, shift.clock_out),
        shift,
      )
    })
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
