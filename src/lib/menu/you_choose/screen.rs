use std::ops;

use super::item::Item;
use super::pair::Pair;

pub struct Screen {
  pub bounds: (Pair, Pair),
  pub pos: Pair,
  pub items_on_screen: usize,
  pub side: ScreenSide,
  pub width: f64,
}

impl Screen {
  pub fn new(side: ScreenSide, width: f64) -> Screen {
    assert!(width > 0.0 && width <= 1.0);

    let bounds = (Pair { y: 0, x: 0 }, Pair { y: 0, x: 0 });
    let pos = Pair { y: 0, x: 0 };

    Screen { bounds, pos, items_on_screen: 0, side, width }
  }

  pub fn show(&mut self) {
    self.bounds =
      self.side.get_bounds((Pair { y: 0, x: 0 }, Self::get_size()), self.width);
  }

  pub fn write_item(&mut self, item: &Item, highlight: bool) -> bool {
    todo!()
  }

  pub fn draw_box(
    &mut self,
    side: ScreenSide,
    width: f64,
    label: &Option<String>,
  ) {
    todo!()
  }

  pub fn get_key(&self) -> i32 {
    todo!()
  }

  pub fn refresh(&mut self) {
    todo!()
  }

  pub fn erase(&mut self) {
    todo!()
  }

  pub fn max_y(&mut self) -> usize {
    self.bounds.1.y as usize
  }

  pub fn _max_x(&mut self) -> usize {
    self.bounds.1.x as usize
  }

  pub fn reset_pos(&mut self) {
    self.pos.y = self.bounds.0.y;
    self.pos.x = self.bounds.0.x;
    self.items_on_screen = 0;
  }

  pub fn get_size() -> Pair {
    todo!()
  }

  pub fn addstr(&mut self, s: &str) {
    let screen_width = self.bounds.1.x - self.bounds.0.x;
    let mut chars = s.chars();
    let mut char_counter = 0;
    let mut curr_string = String::new();

    loop {
      let next_char = chars.next();
      if let Some(c) = next_char {
        // TODO: shorten the code here
        let mut both = false;
        if char_counter >= screen_width {
          self.addstr_clean(&curr_string);
          curr_string.clear();
          self.pos.y += 1;
          self.pos.x = self.bounds.0.x;
          char_counter = 0;

          both = true;
        }
        if c == '\n' {
          self.addstr_clean(&curr_string);
          curr_string.clear();
          self.pos.y += 1;
          self.pos.x = self.bounds.0.x;
          char_counter = 0;
          both &= true;
          if both {
            self.pos.y -= 1;
          }

          continue;
        }
        if self.pos.y >= self.bounds.1.y {
          curr_string.clear();
          break;
        }
        assert!(c != '\n');
        curr_string.push(c);
        char_counter += 1;
      } else {
        break;
      }
    }
    assert!(!curr_string.contains('\n'));
    self.addstr_clean(&curr_string);
  }

  pub fn addstr_clean(&mut self, s: &str) {
    todo!()
  }

  pub fn addch(&mut self, c: char) {
    todo!()
  }

  pub fn skiplines(&mut self, n: i32) {
    self.pos.y += n;
    self.pos.x = self.bounds.0.x;
  }

  pub fn set_pos(&mut self, new_side: ScreenSide, width: f64) {
    assert!(width > 0.0 && width <= 1.0);

    self.side = new_side;
    self.width = width;
  }
}

/// Determines the side on which a pane should be located.
#[derive(Copy, Clone)]
pub enum ScreenSide {
  Left,
  Right,
  Top,
  Bottom,
  /// This option is not affected by width. It will always fill the screen.
  Full,
}

impl ScreenSide {
  pub fn get_bounds(
    &self,
    screen_bounds: (Pair, Pair),
    width: f64,
  ) -> (Pair, Pair) {
    assert!(width <= 1.0 && width > 0.0);
    match self {
      Self::Top => (
        screen_bounds.0,
        Pair {
          y: ((screen_bounds.1.y as f64) * width) as i32,
          x: screen_bounds.1.x,
        },
      ),
      Self::Bottom => (
        // TL: height * (1 - width) + 1
        // BR: BR
        Pair {
          y: (((screen_bounds.1.y - screen_bounds.0.y) as f64) * (1.0 - width))
            as i32
            + 1,
          x: screen_bounds.0.x,
        },
        screen_bounds.1,
      ),
      Self::Left => (
        // TL: TL
        // BR: screen_width * width
        screen_bounds.0,
        Pair {
          y: screen_bounds.1.y,
          x: screen_bounds.0.x
            + (((screen_bounds.1.x - screen_bounds.0.x) as f64) * width) as i32,
        },
      ),
      Self::Right => (
        // TL: screen_width * (1 - width) + 1
        // BR: BR
        Pair {
          y: screen_bounds.0.y,
          x: screen_bounds.0.x
            + (((screen_bounds.1.x - screen_bounds.0.x) as f64) * (1.0 - width))
              as i32
            + 1,
        },
        screen_bounds.1,
      ),
      Self::Full => screen_bounds,
    }
  }
}

impl ops::Not for ScreenSide {
  type Output = Self;
  fn not(self) -> Self {
    match self {
      Self::Top => Self::Bottom,
      Self::Bottom => Self::Top,
      Self::Left => Self::Right,
      Self::Right => Self::Left,
      Self::Full => Self::Full,
    }
  }
}
