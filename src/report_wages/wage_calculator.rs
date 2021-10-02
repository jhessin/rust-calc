use std::collections::HashMap;

struct WageCalculator {
  names: Vec<String>,
  month: String,
  wages: HashMap<String, HashMap<i32, f64>>,
}

impl WageCalculator {
  pub fn new(names: Vec<String>, month: String) -> Self {
    WageCalculator { names, month, wages: HashMap::new() }
  }

  pub fn add_wage(&mut self, name: &str, day: i32, value: f64) {
    self.wages[name][&day] += value;
  }

  pub fn get_total_for(&self, name: &str, day: Option<i32>) -> f64 {
    if let Some(day) = day {
      self.wages[name][&day]
    } else {
      let mut total = 0.0;
      for (_, value) in self.wages[name].into_iter() {
        total += value;
      }
      total
    }
  }

  pub fn get_totals(&self) -> HashMap<&String, f64> {
    let mut result = HashMap::new();
    for (name, wages) in self.wages.into_iter() {
      for (day, wage) in wages.into_iter() {
        let key = &format!("{}-{}", name, day);
        let mut entry = result.entry(key).or_insert(0.0);
        *entry += wage;
      }
    }
    result
  }
}
