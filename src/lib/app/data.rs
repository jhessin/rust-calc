//! This holds all the global data for the app.
//! Can be accessed with the s.data() trait that is added to the Cursive object.

use crate::prelude::*;
use std::fs::File;

use cursive::Printer;

const DATA: &str = "data";
const FILENAME: &str = "data.yml";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppData {
  pub calculator: PaycheckCalculator,
  pub shifts: Vec<Shift>,
  pub date_box: Option<NaiveDate>,
  pub time_box: Option<NaiveTime>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum AppType {
  WageCalculator,
}

impl Default for AppType {
  fn default() -> Self {
    AppType::WageCalculator
  }
}

impl AppType {
  pub fn run(&self, siv: &mut Cursive) {
    match self {
      AppType::WageCalculator => {
        super::wage_calculator::run_wage_calculator(siv)
      }
    }
  }
}

impl View for AppData {
  fn draw(&self, _printer: &Printer) {
    // Nothing to draw.
  }
}

impl AppData {
  pub fn new() -> HideableView<NamedView<AppData>> {
    let s: Self = if let Ok(file) = File::open(FILENAME) {
      serde_yaml::from_reader(file).unwrap_or_default()
    } else {
      Self::default()
    };
    HideableView::new(s.with_name(DATA)).hidden()
  }

  pub fn save(&self) -> Result<()> {
    let file = File::create(FILENAME)?;
    serde_yaml::to_writer(file, self)?;
    Ok(())
  }
}

pub trait Datable {
  fn data(&mut self) -> ViewRef<AppData>;
}

impl Datable for Cursive {
  fn data(&mut self) -> ViewRef<AppData> {
    self.find_name(DATA).context("There is no data layer!").unwrap()
  }
}
