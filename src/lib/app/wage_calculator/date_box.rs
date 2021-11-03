use crate::prelude::*;

pub fn get_date(siv: &mut Cursive) {
  siv.add_layer(CalendarView::<_, EnglishLocale>::new(Utc::today()).on_submit(
    |s, d| {
      s.data().date_box = Some(d.naive_utc());
      s.pop_layer();
    },
  ));
}
