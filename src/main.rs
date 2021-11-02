#![allow(dead_code)]

use cursive::default;

#[macro_use]
mod lib;

fn main() {
  cursive_bare();
}

fn cursive_bare() {
  let mut siv = default();

  siv.run();
}
