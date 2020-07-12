#![allow(unused_must_use)]

mod app;
mod command;
mod habit;
mod theme;
mod utils;
mod views;

use crate::app::App;
use crate::command::{open_command_window, Command};
use crate::utils::{load_configuration_file, AppConfig};

use cursive::crossterm;
use cursive::views::NamedView;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIGURATION: AppConfig = load_configuration_file();
}

fn main() {
    let mut s = crossterm().unwrap();
    let app = App::load_state();
    s.add_layer(NamedView::new("Main", app));
    s.add_global_callback(':', |s| open_command_window(s));

    s.set_theme(theme::theme_gen());
    s.run();
}
