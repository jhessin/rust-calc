use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Pair {
  pub y: i32,
  pub x: i32,
}

impl fmt::Display for Pair {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Pair({}, {})", self.y, self.x)
  }
}
