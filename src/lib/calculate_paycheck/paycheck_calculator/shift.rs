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
    let clock_in = get_clock_in(static_year, static_month);
    let clock_out = get_clock_out(static_year, static_month);
    let incentive = get_incentive();
    let short = get_short();
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
  pub fn update(
    &self,
    static_year: Option<i32>,
    static_month: Option<u32>,
  ) -> Self {
    let clock_in = if prompt!(format!(
      "(true/false)Clock in time {:?} has changed.",
      self.clock_in
    )) {
      get_clock_in(static_year, static_month)
    } else {
      self.clock_in
    };
    let clock_out = if prompt!(format!(
      "(true/false)Clock out time {:?} has changed.",
      self.clock_out
    )) {
      get_clock_out(static_year, static_month)
    } else {
      self.clock_out
    };
    let incentive = if self.incentive.is_some()
      && prompt!(format!(
        "(true/false)Incentive time: \
	  {} hr has changed.",
        self.incentive.unwrap()
      )) {
      get_incentive()
    } else {
      self.incentive
    };
    let short = if self.short.is_some()
      && prompt!(format!(
        "(true/false)Short time: {} hr(s) has \
	  changed.",
        self.short.unwrap()
      )) {
      get_short()
    } else {
      self.short
    };
    Self { clock_in, clock_out, incentive, short }
  }
}

fn get_clock_in(
  static_year: Option<i32>,
  static_month: Option<u32>,
) -> DateTime<Local> {
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
  let day = prompt!("What day did you clock in?");
  let hour: f64 = prompt!("What hour did you clock in (with decimal)");
  let (hour, min, sec) = (hour.floor() as u32, (hour.fract() * 60.0) as u32, 0);
  Local.ymd(year, month, day).and_hms(hour, min, sec)
}

fn get_clock_out(
  static_year: Option<i32>,
  static_month: Option<u32>,
) -> DateTime<Local> {
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
  let day = prompt!("What day did you clock out?");
  let hour: f64 = prompt!("What hour did you clock out (with decimal)");
  let (hour, min, sec) = (hour.floor() as u32, (hour.fract() * 60.0) as u32, 0);
  Local.ymd(year, month, day).and_hms(hour, min, sec)
}

fn get_incentive() -> Option<f64> {
  let incentive: bool = prompt!("Was incentive offered?");
  if incentive {
    Some(prompt!("How many hours?"))
  } else {
    None
  }
}

fn get_short() -> Option<f64> {
  let short: bool = prompt!("Were you short during the shift?");
  if short {
    Some(prompt!("How many hours?"))
  } else {
    None
  }
}
