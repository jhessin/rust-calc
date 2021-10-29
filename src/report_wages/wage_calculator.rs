use std::collections::HashMap;

pub struct WageCalculator {
  pub names: Vec<String>,
  pub month: String,
  pub wages: HashMap<String, HashMap<i32, f64>>,
}

impl IntoIterator for WageCalculator {
  type Item = String;
  type IntoIter = WageCalculatorIterator;

  fn into_iter(self) -> Self::IntoIter {
    WageCalculatorIterator { wages: self, index: 0 }
  }
}

pub struct WageCalculatorIterator {
  wages: WageCalculator,
  index: usize,
}

impl Iterator for WageCalculatorIterator {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    let result = match self.index {
      0 => Some(String::from("Who's wages are you adding?")),
      i => self.wages.names.get(i).cloned(),
    };
    self.index += 1;
    result
  }
}

impl WageCalculator {
  pub fn new(names: Vec<String>, month: String) -> Self {
    WageCalculator { names, month, wages: HashMap::new() }
  }

  pub fn add_wage(&mut self, name: &str, day: i32, value: f64) {
    *self
      .wages
      .entry(String::from(name))
      .or_default()
      .entry(day)
      .or_insert(0.0) += value;
  }

  pub fn get_total_for(&self, name: &str, day: Option<i32>) -> f64 {
    if let Some(day) = day {
      self.wages[name][&day]
    } else {
      let mut total = 0.0;
      for (_, value) in self.wages.get(name).unwrap().iter() {
        total += value;
      }
      total
    }
  }

  pub fn get_totals(&self) -> HashMap<String, f64> {
    let mut result = HashMap::new();
    for (name, wages) in self.wages.iter() {
      for (day, wage) in wages.iter() {
        let key = format!("{}-{}", name, day);
        let entry = result.entry(key).or_insert(0.0);
        *entry += wage;
      }
    }
    result
  }
}
