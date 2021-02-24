use std::default::Default;

mod traits;
pub use traits::{Habit, HabitWrapper};

mod count;
pub use count::Count;

mod bit;
pub use bit::Bit;

mod float;
pub use float::Float;

mod prelude;
pub use prelude::{TrackEvent, ViewMode};

use crate::app::Cursor;

use cursive::direction::Absolute;

#[derive(Debug, Default)]
pub struct InnerData {
    pub cursor: Cursor,
    pub view_mode: ViewMode,
}

impl InnerData {
    pub fn move_cursor(&mut self, d: Absolute) {
        self.cursor.small_seek(d);
    }
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }
    pub fn view_mode(&self) -> ViewMode {
        self.view_mode
    }
}
