use serde::{Deserialize, Serialize};

pub enum TrackEvent {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ViewMode {
    Day,
    Week,
    Month,
    Year,
}

impl std::default::Default for ViewMode {
    fn default() -> Self {
        ViewMode::Day
    }
}

pub fn default_auto() -> bool {
    false
}
