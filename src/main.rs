#![allow(unused_must_use)]

use chrono::NaiveDate;

use cursive::views::{Dialog, LinearLayout};
use cursive::Cursive;

mod habit;
use crate::habit::Habit;

mod views;
use crate::views::bitview::BitView;
use crate::views::countview::CountView;

mod theme;

enum ViewMode {
    Daily,
    Monthly,
}

fn main() {
    let mut work_out: Habit<bool> = Habit::new("gymming", true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 4), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 2), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 3), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 1), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 5), false);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 8), false);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 11), false);

    let mut again: Habit<u32> = Habit::new("reading", 5);
    again.insert_entry(NaiveDate::from_ymd(2020, 2, 4), 4);
    again.insert_entry(NaiveDate::from_ymd(2020, 2, 2), 2);
    again.insert_entry(NaiveDate::from_ymd(2020, 2, 7), 5);

    let mut s = Cursive::default();

    let gym_view = BitView::new(work_out);
    let gym_title = gym_view.get_title();

    let reading_view = CountView::new(again);
    let reading_title = reading_view.get_title();

    s.add_global_callback('q', |a| a.quit());
    s.add_layer(
        LinearLayout::horizontal()
            .child(Dialog::around(gym_view).title(gym_title))
            .child(Dialog::around(reading_view).title(reading_title)),
    );

    s.set_theme(theme::theme_gen());
    s.run();
}
