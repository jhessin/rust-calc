use chrono::{DateTime, Local};

#[derive(Copy, Clone)]
pub struct Shift {
  clock_in: DateTime<Local>,
  clock_out: DateTime<Local>,
}

#[derive(Copy, Clone)]
pub struct PaycheckCalculator {
  /// The Base rate that a person is paid.
  base_rate: f32,
  /// The weekend_diff that a person is paid.
  weekend_diff: f32,
  /// Any incentive that is given
  incentive_diff: f32,
  /// Any amount given for working short
  short_diff: f32,
  shifts: (DateTime<Local>, DateTime<Local>),
}
