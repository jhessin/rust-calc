use std::fmt;

use super::disp::DispFunc;
use super::screen::{Screen, ScreenSide};

pub struct Preview<D>
where
  D: fmt::Display,
{
  pub func: DispFunc<D>,
  pub box_screen: Screen,
  pub screen: Screen,
  pub label: Option<String>,
}

impl<D> Preview<D>
where
  D: fmt::Display,
{
  pub fn new(func: DispFunc<D>, side: ScreenSide, width: f64) -> Preview<D> {
    let box_screen = Screen::new(side, width);
    let screen = Screen::new(side, width);

    Preview { func, box_screen, screen, label: None }
  }

  pub fn draw_box(&mut self) {
    self.box_screen.draw_box(ScreenSide::Full, 1.0, &self.label);
  }

  pub fn show(&mut self) {
    self.box_screen.show();
    self.screen.show();
    self.update_bounds();
  }

  pub fn update_bounds(&mut self) {
    self.screen.bounds.0.y += 1;
    self.screen.bounds.0.x += 1;
    self.screen.bounds.1.y -= 1;
    self.screen.bounds.1.x -= 1;
  }

  pub fn refresh(&mut self) {
    self.screen.refresh();
    self.box_screen.refresh();
    self.update_bounds();
  }

  pub fn set_pos(&mut self, side: ScreenSide, width: f64) {
    self.screen.set_pos(side, width);
    self.box_screen.set_pos(side, width);
    self.update_bounds();
  }

  pub fn set_label(&mut self, label: String) {
    self.label = Some(label);
  }
}
