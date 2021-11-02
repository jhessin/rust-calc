use std::ops::{Deref, DerefMut};

use cursive::CursiveRunnable;

pub struct App {
  siv: CursiveRunnable,
}

impl Deref for App {
  type Target = CursiveRunnable;

  fn deref(&self) -> &Self::Target {
    &self.siv
  }
}

impl DerefMut for App {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.siv
  }
}

impl App {
  pub fn new() -> Self {
    Self { siv: cursive::default() }
  }
}
