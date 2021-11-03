use crate::prelude::*;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PaycheckCalculator {
  /// The Base rate that a person is paid.
  default_base_rate: f64,
  /// The weekend_diff that a person is paid.
  default_weekend_diff: f64,
  /// Any amount given for working short
  default_short_diff: f64,
  /// Any amount given for night work
  default_night_diff: f64,
  /// Any other incentive that is given
  default_incentive_diff: f64,
  /// A list of all the shifts that were worked this pay period.
  shifts: Vec<Shift>,
}

impl PaycheckCalculator {
  /// -------------------------------------------------------------------------
  ///     Mutators
  /// These methods mutate the calculator for piecewise construction.
  /// -------------------------------------------------------------------------
  ///
  /// Add the default base pay rate.
  pub fn with_base(mut self, rate: f64) -> Self {
    self.default_base_rate = rate;
    self
  }

  /// Add default weekend incentive rate.
  pub fn with_weekend(mut self, rate: f64) -> Self {
    self.default_weekend_diff = rate;
    self
  }

  /// Add default incentive for picked up shifts.
  pub fn with_incentive(mut self, rate: f64) -> Self {
    self.default_incentive_diff = rate;
    self
  }

  /// Add default compensation for working short-staffed
  pub fn with_short(mut self, rate: f64) -> Self {
    self.default_short_diff = rate;
    self
  }

  /// Add default compensation for working at night.
  pub fn with_night(mut self, rate: f64) -> Self {
    self.default_night_diff = rate;
    self
  }

  /// Just for completion you can build the shifts yourself.
  pub fn with_shifts(mut self, shifts: Vec<Shift>) -> Self {
    self.shifts = shifts;
    self
  }
}

/// This returns the view for creating a calculator.
pub fn new_calculator_view(siv: &mut Cursive) -> Dialog {
  let calc: PaycheckCalculator = if let Some(calc) = siv.take_user_data() {
    calc
  } else {
    siv.data().calculator.clone()
  };
  siv.set_user_data(calc.clone());

  let mut root = LinearLayout::horizontal();

  let mut fields = LinearLayout::vertical();

  fields.add_child(TextView::new("Base pay rate:"));
  let mut edit_view = EditView::new().on_edit(|siv, text, _cursor| {
    siv.with_user_data(|data: &mut PaycheckCalculator| {
      if let Ok(value) = text.parse() {
        data.default_base_rate = value;
      }
    });
  });
  edit_view.set_content(calc.default_base_rate.to_string());
  fields.add_child(edit_view);

  fields.add_child(TextView::new("Weekend incentive:"));
  edit_view = EditView::new().on_edit(|siv, text, _cursor| {
    siv.with_user_data(|data: &mut PaycheckCalculator| {
      if let Ok(value) = text.parse() {
        data.default_weekend_diff = value;
      }
    });
  });
  edit_view.set_content(calc.default_weekend_diff.to_string());
  fields.add_child(edit_view);

  fields.add_child(TextView::new("Incentive for working short:"));
  edit_view = EditView::new().on_edit(|siv, text, _cursor| {
    siv.with_user_data(|data: &mut PaycheckCalculator| {
      if let Ok(value) = text.parse() {
        data.default_short_diff = value;
      }
    });
  });
  edit_view.set_content(calc.default_short_diff.to_string());
  fields.add_child(edit_view);

  fields.add_child(TextView::new("Incentive for working nights:"));
  edit_view = EditView::new().on_edit(|siv, text, _cursor| {
    siv.with_user_data(|data: &mut PaycheckCalculator| {
      if let Ok(value) = text.parse() {
        data.default_night_diff = value;
      }
    });
  });
  edit_view.set_content(calc.default_night_diff.to_string());
  fields.add_child(edit_view);

  fields.add_child(TextView::new("Other incentive offered:"));
  edit_view = EditView::new().on_edit(|siv, text, _cursor| {
    siv.with_user_data(|data: &mut PaycheckCalculator| {
      if let Ok(value) = text.parse() {
        data.default_incentive_diff = value;
      }
    });
  });
  edit_view.set_content(calc.default_incentive_diff.to_string());
  fields.add_child(edit_view);

  let mut buttons = LinearLayout::vertical();
  buttons.add_child(Button::new("Create", |siv| {
    if let Some(data) = siv.take_user_data() {
      siv.data().calculator = data;
      siv.pop_layer();
      // TODO: load the view where shifts can be added here.
    }
  }));

  root.add_child(fields);
  root.add_child(buttons);

  Dialog::new().content(root)
}
