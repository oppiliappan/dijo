use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::habit::prelude::default_auto;
use crate::habit::traits::Habit;
use crate::habit::{TrackEvent, ViewMode};
use crate::CONFIGURATION;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CustomBool(bool);

use std::fmt;
impl fmt::Display for CustomBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:^3}",
            if self.0 {
                CONFIGURATION.true_chr
            } else {
                CONFIGURATION.false_chr
            }
        )
    }
}

impl From<bool> for CustomBool {
    fn from(b: bool) -> Self {
        CustomBool(b)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bit {
    name: String,
    stats: HashMap<NaiveDate, CustomBool>,
    goal: CustomBool,

    #[serde(default = "default_auto")]
    auto: bool,

    #[serde(skip)]
    view_month_offset: u32,

    #[serde(skip)]
    view_mode: ViewMode,
}

impl Bit {
    pub fn new(name: impl AsRef<str>, auto: bool) -> Self {
        return Bit {
            name: name.as_ref().to_owned(),
            stats: HashMap::new(),
            goal: CustomBool(true),
            auto,
            view_month_offset: 0,
            view_mode: ViewMode::Day,
        };
    }
}

impl Habit for Bit {
    type HabitType = CustomBool;
    fn name(&self) -> String {
        return self.name.clone();
    }
    fn set_name(&mut self, n: impl AsRef<str>) {
        self.name = n.as_ref().to_owned();
    }
    fn set_goal(&mut self, _: Self::HabitType) {
        self.goal = true.into();
    }
    fn get_by_date(&self, date: NaiveDate) -> Option<&Self::HabitType> {
        self.stats.get(&date)
    }
    fn insert_entry(&mut self, date: NaiveDate, val: Self::HabitType) {
        *self.stats.entry(date).or_insert(val) = val;
    }
    fn reached_goal(&self, date: NaiveDate) -> bool {
        if let Some(val) = self.stats.get(&date) {
            if val.0 >= self.goal.0 {
                return true;
            }
        }
        return false;
    }
    fn remaining(&self, date: NaiveDate) -> u32 {
        if let Some(val) = self.stats.get(&date) {
            if val.0 {
                return 0;
            } else {
                return 1;
            }
        } else {
            return 1;
        }
    }
    fn goal(&self) -> u32 {
        return 1;
    }
    fn modify(&mut self, date: NaiveDate, _: TrackEvent) {
        if let Some(val) = self.stats.get_mut(&date) {
            *val = (val.0 ^ true).into();
        } else {
            self.insert_entry(date, CustomBool(true));
        }
    }
    fn set_view_month_offset(&mut self, offset: u32) {
        self.view_month_offset = offset;
    }
    fn view_month_offset(&self) -> u32 {
        self.view_month_offset
    }
    fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }
    fn view_mode(&self) -> ViewMode {
        self.view_mode
    }
    fn is_auto(&self) -> bool {
        self.auto
    }
}
