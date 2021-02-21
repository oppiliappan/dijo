use std::default::Default;
use std::sync::mpsc::Receiver;

use notify::{DebouncedEvent, RecommendedWatcher};

use crate::habit::HabitWrapper;

mod cursor;
mod impl_self;
mod impl_view;
mod message;

pub struct StatusLine(String, String);
pub use cursor::Cursor;
pub use message::{Message, MessageKind};

pub struct App {
    // holds app data
    habits: Vec<Box<dyn HabitWrapper>>,

    _file_watcher: RecommendedWatcher,
    file_event_recv: Receiver<DebouncedEvent>,
    focus: usize,
    cursor: Cursor,
    message: Message,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
