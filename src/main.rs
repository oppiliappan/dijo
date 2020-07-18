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

use clap::{App as ClapApp, Arg};
use cursive::termion;
use cursive::views::NamedView;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIGURATION: AppConfig = load_configuration_file();
}

fn main() {
    let matches = ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .takes_value(true)
                .value_name("CMD")
                .help("run a dijo command"),
        )
        .get_matches();
    if let Some(c) = matches.value_of("command") {
        let command = Command::from_string(c);
        match command {
            Ok(Command::TrackUp(_)) | Ok(Command::TrackDown(_)) => {
                let mut app = App::load_state();
                app.parse_command(command);
                app.save_state();
            }
            Err(e) => {
                eprintln!("{}", e);
            }
            _ => eprintln!(
                "Commands other than `track-up` and `track-down` are currently not supported!"
            ),
        }
    } else {
        let mut s = termion().unwrap();
        let app = App::load_state();
        s.add_layer(NamedView::new("Main", app));
        s.add_global_callback(':', |s| open_command_window(s));

        s.set_theme(theme::theme_gen());
        s.run();
    }
}
