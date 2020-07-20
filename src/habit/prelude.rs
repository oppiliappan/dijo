use serde::{Deserialize, Serialize};
use std::default;
use std::fmt;

#[derive(Debug, PartialEq)]
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

impl default::Default for ViewMode {
    fn default() -> Self {
        ViewMode::Day
    }
}

impl fmt::Display for ViewMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewMode::Day => write!(f, "DAY"),
            ViewMode::Week => write!(f, "WEEK"),
            ViewMode::Month => write!(f, "MONTH"),
            ViewMode::Year => write!(f, "YEAR"),
        }
    }
}

pub fn default_auto() -> bool {
    false
}
