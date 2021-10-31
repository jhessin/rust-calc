use std::fmt;
use std::iter::Peekable;

use config::*;
use disp::*;
use item::*;
use keys::*;
use preview::*;
use screen::*;
use MenuReturnCode::{Done, Pass};

use crate::lib::menu::you_choose::screen::Screen;

mod config;
mod disp;
mod item;
mod keys;
mod pair;
mod preview;
mod screen;

const ERROR_MARGIN: f64 = 0.0001;

/// A Menu that lazily displays an iterable and (optionally) its preview.
pub struct Menu<'a, I, D>
where
  D: fmt::Display,
  I: Iterator<Item = D>,
{
  iter: Peekable<I>,
  screen: Screen,
  preview: Option<Preview<D>>,
  item_icon: &'a str,
  chosen_item_icon: &'a str,
  selection: Vec<usize>,
  keys: Keys,

  state: MenuState<'a>,
  config: MenuConfig,
}

enum MenuReturnCode {
  Done,
  Pass,
}
type RetCode = MenuReturnCode;

impl<'a, I, D> Menu<'a, I, D>
where
  D: fmt::Display,
  I: Iterator<Item = D>,
{
  /// Create a new menu object. The iterable passed in must contain displayable
  /// items.
  pub fn new(iter: I) -> Menu<'a, I, D> {
    todo!()
  }

  /// Initialize and display the menu on the screen.
  pub fn show(&mut self) -> Vec<usize> {
    todo!()
  }

  fn finish(&self) -> Vec<usize> {
    self.selection.clone()
  }

  fn yield_item(&mut self, i: usize) -> Option<&Item> {
    while self.state.items.len() <= i {
      if let Some(item) = self.iter.next() {
        let mut new_item =
          Item::new(&item, self.item_icon, self.chosen_item_icon);
        if let Some(preview) = &self.preview {
          new_item.preview(item, &preview.func);
        }
        self.state.items.push(new_item);
      } else {
        return None;
      }
    }
    Some(&self.state.items[i])
  }

  fn refresh(&mut self) {
    // Maximum index that will fit on current screen state
    let end = self.state.start + self.screen.max_y();
    self.yield_item(end);

    self.screen.reset_pos();
    if let Some(prev) = &mut self.preview {
      prev.draw_box();
      prev.screen.reset_pos();
    }
    let mut i = self.state.start;
    let pos = self.state.hover + i;
    while let Some(item) = self.state.items.get(i) {
      if !self.screen.write_item(item, pos == i) {
        break;
      }
      if pos == i {
        if let Some(prev) = &mut self.preview {
          prev.screen.addstr(item.preview.as_ref().unwrap());
        }
      }

      i += 1;
    }

    self.screen.refresh();

    if let Some(prev) = &mut self.preview {
      prev.refresh();
    }
  }

  fn handle_key(&mut self, val: i32) -> RetCode {
    if self.keys.down.contains(&val) {
      self.move_selection(1)
    } else if self.keys.up.contains(&val) {
      self.move_selection(-1)
    } else if self.config.multiselect && self.keys.multiselect.contains(&val) {
      self.multiselect_item()
    } else if self.keys.select.contains(&val) {
      self.select_item()
    } else {
      Pass
    }
  }

  fn select_item(&mut self) -> RetCode {
    let curr_item_idx = self.state.start + self.state.hover;
    match self.selection.last() {
      Some(&num) if num == curr_item_idx => return Done,
      _ => (),
    }
    self.state.items[curr_item_idx].select();
    self.selection.push(curr_item_idx);
    Done
  }

  fn multiselect_item(&mut self) -> RetCode {
    let curr_item_idx = self.state.start + self.state.hover;
    let curr_item = &mut self.state.items[curr_item_idx];
    curr_item.select();

    let item_idx_pos =
      match self.selection.iter().position(|x| *x == curr_item_idx) {
        Some(idx) => idx as i32,
        None => -1,
      };

    if curr_item.chosen() && item_idx_pos == -1 {
      self.selection.push(curr_item_idx);
    } else if !curr_item.chosen() && item_idx_pos != -1 {
      self.selection.remove(item_idx_pos as usize);
    }

    Pass
  }

  fn scroll(&mut self, amount: i32) {
    self.state.start = ((self.state.start as i32) + amount) as usize;
    assert!(self.state.start < 1_000_000);
  }

  fn move_selection(&mut self, amount: i32) -> RetCode {
    let num_items = self.screen.items_on_screen as f64;
    let new_hover = ((self.state.hover as i32) + amount) as f64;

    if new_hover < 0.0 || (new_hover - num_items).abs() < ERROR_MARGIN {
      return Pass;
    }

    self.state.hover = new_hover as usize;

    if new_hover > num_items * 0.67
      && self.state.start + self.screen.items_on_screen < self.state.items.len()
    {
      self.scroll(1);
      self.state.hover -= 1;
    } else if new_hover < num_items * 0.33 && self.state.start > 0 && amount < 0
    {
      self.scroll(-1);
      self.state.hover += 1;
    }

    Pass
  }

  /// Add a preview pane that displays the result of applying the function
  /// passed in to each item in the iterable. The function must return a
  /// String.
  pub fn preview<F>(mut self, func: F) -> Menu<'a, I, D>
  where
    F: Fn(D) -> String + 'static,
  {
    let func = DispFunc::new(Box::new(func));
    self.screen.set_pos(ScreenSide::Left, 0.5);
    self.preview = Some(Preview::new(func, ScreenSide::Right, 0.5));
    self
  }

  /// Sets the position of the preview pane. The `side` parameter determines
  /// the side on which the pane sits. The `width` parameter is a float between
  /// `0.0` and `1.0`, inclusive. It determines the proportion of the screen that
  /// the preview pane should use.
  ///
  /// The menu's side is automatically switched to opposite the preview pane's side,
  /// and the menu's width is set to `1 - width`.
  pub fn preview_pos(mut self, side: ScreenSide, width: f64) -> Menu<'a, I, D> {
    self.screen.set_pos(!side, 1.0 - width);
    self
      .preview
      .as_mut()
      .expect("Must create preview before settting it's position")
      .set_pos(side, width);

    self
  }

  /// Sets the default icon of the menu. This is displayed before each entry.
  pub fn icon(mut self, icon: &'a str) -> Menu<'a, I, D> {
    self.item_icon = icon;
    self
  }

  /// Sets the icon displayed when an item is selected in multiselect mode.
  pub fn selected_icon(mut self, icon: &'a str) -> Menu<'a, I, D> {
    self.chosen_item_icon = icon;
    self
  }

  /// Sets the text displayed on top of the preview box. It is recommended to surround the label
  /// with spaces for aesthetic reasons. If it is not set, `" preview "` will be used.
  pub fn preview_label(mut self, label: String) -> Menu<'a, I, D> {
    self
      .preview
      .as_mut()
      .expect("Must create preview before settting it's position")
      .set_label(label);
    self
  }

  /// Adds a keybinding that triggers a multiselection. This inputs an `ncurses` keycode.
  /// All ascii keys can be set by passing the character as an `i32`. The keycodes for
  /// special keys can be found by importing `ncurses` and using the provided constants
  /// or by testing with the `getch` function. For example, running the following will display
  /// the keycodes on the screen.
  ///
  /// ```
  /// // Make sure to add ncurses as a dependency!
  /// use ncurses::*;
  ///     initscr();
  ///     loop {
  ///         let c: i32 = getch();
  ///         clear();
  ///         if c == 'q' as i32 {break}
  ///         addstr(&format!("Pressed key with keycode {}!", c.to_string()));
  ///     }
  ///     endwin();
  /// ```
  pub fn add_multiselect_key(mut self, key: i32) -> Menu<'a, I, D> {
    self.keys.multiselect.push(key);
    // self
    todo!("Use something other than ncurses here")
  }

  /// Adds a keybinding that triggers an up movement. See [`add_multiselect_key`](struct.Menu.html#method.add_multiselect_key) for more information.
  pub fn add_up_key(mut self, key: i32) -> Menu<'a, I, D> {
    self.keys.up.push(key);
    self
  }

  /// Adds a keybinding that triggers a down movement. See [`add_multiselect_key`](struct.Menu.html#method.add_multiselect_key) for more information.
  pub fn add_down_key(mut self, key: i32) -> Menu<'a, I, D> {
    self.keys.down.push(key);
    self
  }

  /// Adds a keybinding that triggers a selection. See [`add_multiselect_key`](struct.Menu.html#method.add_multiselect_key) for more information.
  pub fn add_select_key(mut self, key: i32) -> Menu<'a, I, D> {
    self.keys.select.push(key);
    self
  }

  /// Allow multiple items to be selected from the menu.
  pub fn multiselect(mut self) -> Menu<'a, I, D> {
    self.config.multiselect = true;
    self
  }
}
