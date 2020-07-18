use std::default::Default;
use std::sync::mpsc::Receiver;

use notify::{DebouncedEvent, INotifyWatcher};

use crate::habit::HabitWrapper;

mod impl_self;
mod impl_view;

pub struct StatusLine(String, String);

pub struct App {
    // holds app data
    habits: Vec<Box<dyn HabitWrapper>>,

    _file_watcher: INotifyWatcher,
    file_event_recv: Receiver<DebouncedEvent>,
    focus: usize,
    view_month_offset: u32,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
