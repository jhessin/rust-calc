//! This is the Main Menu of the project. Here we can choose from different calculation apps.

// This can be used to import everything in the app.
use crate::all::*;
use cursive::{menu::MenuTree, views::Menubar};

pub fn menu() -> Menubar {
  // The main menu
  let mut menu_bar = Menubar::new();

  // The file menu
  let mut file_menu = MenuTree::new();
  file_menu.add_leaf("Save", |s| s.data().save().unwrap_or_default());
  file_menu.add_delimiter();
  file_menu.add_leaf("Quit", |s| s.quit());

  // Activity menu
  let mut activity_menu = MenuTree::new();
  activity_menu.add_leaf("Wage Calculator", |s| {
    s.data().app_type = Some(AppType::WageCalculator);
    s.data().app_type.unwrap().run();
  });

  // Add menus to menu bar
  menu_bar.add_subtree("File", file_menu);
  menu_bar.add_subtree("Activities", activity_menu);

  menu_bar
}
