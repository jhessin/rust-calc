use crate::prelude::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Shift {
  /// The time clocked in.
  pub clock_in: NaiveTime,
  /// The time clocked out.
  pub clock_out: NaiveTime,
  /// The amount of incentive time if any.
  pub incentive: Option<f64>,
  /// The amount of time working short if any.
  pub short: Option<f64>,
}

impl Shift {
  /// This will prompt the user for new shift data.
  pub fn from_input() -> Self {
    todo!()
  }

  /// returns the duration of the shift in hours
  pub fn len(&self) -> f64 {
    let dur = self.clock_out - self.clock_in;
    (dur.num_hours() as f64) + (dur.num_minutes() as f64 / 60.0)
  }

  /// Update shift data with user input
  pub fn update(&self) -> Self {
    todo!()
  }
}
