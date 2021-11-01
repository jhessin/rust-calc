use chrono::{DateTime, Local, TimeZone};

#[derive(Copy, Clone, Debug)]
pub struct Shift {
  /// The time clocked in.
  pub clock_in: DateTime<Local>,
  /// The time clocked out.
  pub clock_out: DateTime<Local>,
  /// The amount of incentive time if any.
  pub incentive: Option<f64>,
  /// The amount of time working short if any.
  pub short: Option<f64>,
}

impl Shift {
  /// This will prompt the user for new shift data.
  pub fn from_input(
    static_year: Option<i32>,
    static_month: Option<u32>,
  ) -> Self {
    let year = if let Some(year) = static_year {
      year
    } else {
      prompt!("What year did you clock in?")
    };
    let month = if let Some(month) = static_month {
      month
    } else {
      prompt!("What month did you clock in?")
    };
    let day = prompt!("Day?");
    let hour: f64 = prompt!("What hour did you clock in (with decimal)");
    let (hour, min, sec) =
      (hour.floor() as u32, (hour.fract() * 60.0) as u32, 0);
    let clock_in = Local.ymd(year, month, day).and_hms(hour, min, sec);
    let year = if let Some(year) = static_year {
      year
    } else {
      prompt!("What year did you clock out?")
    };
    let month = if let Some(month) = static_month {
      month
    } else {
      prompt!("What month did you clock out?")
    };
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
