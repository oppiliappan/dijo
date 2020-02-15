#![allow(unused_must_use)]

use chrono::NaiveDate;

use cursive::views::{Dialog, EditView, LinearLayout, ListView, SelectView};
use cursive::Cursive;

mod habit;
use crate::habit::{Habit, HabitTrait, HabitType};

mod views;
use crate::views::habitview::HabitView;

mod theme;

struct App {
    habits: Vec<Habit>,
    status: String,
}

fn main() {
    let mut s = Cursive::default();

    let mut gymming = Habit::new("gym", HabitType::Count(5));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 11), HabitType::Count(7));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 12), HabitType::Count(8));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 13), HabitType::Count(9));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 14), HabitType::Count(10));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 15), HabitType::Count(11));

    let mut reading = Habit::new("read", HabitType::Count(5));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 11), HabitType::Bit(true));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 12), HabitType::Bit(false));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 13), HabitType::Bit(true));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 14), HabitType::Bit(false));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 15), HabitType::Bit(true));

    let gym_title = gymming.get_name();
    let gym_view = HabitView::new(gymming);

    let read_title = reading.get_name();
    let read_view = HabitView::new(reading);

    s.add_global_callback('q', |a| a.quit());
    s.add_layer(
        LinearLayout::horizontal()
            .child(Dialog::around(gym_view).title(gym_title))
            .child(Dialog::around(read_view).title(read_title)),
    );

    s.set_theme(theme::theme_gen());
    s.run();
}
