#![allow(unused_must_use)]

use std::default::Default;

mod app;
mod command;
mod habit;
mod theme;
mod utils;
mod views;

use crate::app::App;
use crate::command::{open_command_window, Command};
use crate::habit::{Bit, Count, Habit};
use crate::utils::{load_configuration_file, AppConfig};

use chrono::NaiveDate;
use cursive::theme::{BaseColor, Color};
use cursive::views::NamedView;
use cursive::Cursive;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIGURATION: AppConfig = load_configuration_file();
}

fn main() {
    let mut s = Cursive::crossterm().unwrap();

    // let mut gymming = Count::new("gym", 5);
    // gymming.insert_entry(NaiveDate::from_ymd(2020, 5, 11), 7);
    // gymming.insert_entry(NaiveDate::from_ymd(2020, 5, 12), 8);
    // gymming.insert_entry(NaiveDate::from_ymd(2020, 5, 13), 9);
    // gymming.insert_entry(NaiveDate::from_ymd(2020, 5, 14), 10);
    // gymming.insert_entry(NaiveDate::from_ymd(2020, 5, 15), 11);

    // let mut reading = Bit::new("read");
    // reading.insert_entry(NaiveDate::from_ymd(2020, 5, 11), true.into());
    // reading.insert_entry(NaiveDate::from_ymd(2020, 5, 12), false.into());
    // reading.insert_entry(NaiveDate::from_ymd(2020, 5, 13), true.into());
    // reading.insert_entry(NaiveDate::from_ymd(2020, 5, 14), false.into());
    // reading.insert_entry(NaiveDate::from_ymd(2020, 5, 15), true.into());

    // let mut walking = Bit::new("walk");
    // walking.insert_entry(NaiveDate::from_ymd(2020, 5, 11), true.into());
    // walking.insert_entry(NaiveDate::from_ymd(2020, 5, 12), false.into());
    // walking.insert_entry(NaiveDate::from_ymd(2020, 5, 13), true.into());
    // walking.insert_entry(NaiveDate::from_ymd(2020, 5, 14), false.into());
    // walking.insert_entry(NaiveDate::from_ymd(2020, 5, 15), true.into());

    // app.add_habit(Box::new(gymming));
    // app.add_habit(Box::new(reading));
    // app.add_habit(Box::new(walking));

    let app = App::load_state();
    s.add_layer(NamedView::new("Main", app));
    s.add_global_callback(':', |s| open_command_window(s));

    s.set_theme(theme::theme_gen());
    s.run();
}
