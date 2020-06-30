#![allow(unused_must_use)]

use chrono::NaiveDate;

use lazy_static::lazy_static;

//use cursive::views::{Dialog, EditView, LinearLayout, ListView, SelectView};
use cursive::theme::{BaseColor, Color};
use cursive::views::NamedView;
use cursive::Cursive;

mod habit;
use crate::habit::{Bit, Count, Habit};

mod app;
mod command;
mod theme;
use crate::app::{App, ViewMode};
use crate::command::{open_command_window, Command};

mod views;

pub struct AppConfig {
    pub true_chr: char,
    pub false_chr: char,
    pub future_chr: char,

    // view dimensions
    pub view_width: usize,
    pub view_height: usize,

    // app dimensions
    pub grid_width: usize,

    // color config
    pub reached_color: Color,
    pub todo_color: Color,
    pub future_color: Color,
}

lazy_static! {
    pub static ref CONFIGURATION: AppConfig = load_configuration_file();
}

fn load_configuration_file() -> AppConfig {
    return AppConfig {
        true_chr: '·',
        false_chr: '·',
        future_chr: '·',
        view_width: 25,
        view_height: 8,
        grid_width: 3,
        reached_color: Color::Dark(BaseColor::Cyan),
        todo_color: Color::Dark(BaseColor::Magenta),
        future_color: Color::Light(BaseColor::Black),
    };
}

fn main() {
    let mut s = Cursive::default();

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
