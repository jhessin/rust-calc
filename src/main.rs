#![allow(dead_code)]

use cursive::default;

#[macro_use]
mod lib;

fn main() {
  app();
}

fn cursive_bare() {
  let mut siv = default();

  siv.run();
}

fn app() {
  let mut siv = lib::app::App::new();

  siv.run();
}
