use std::collections::HashMap;
use std::default::Default;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::command::GoalKind;
use crate::habit::prelude::default_auto;
use crate::habit::traits::Habit;
use crate::habit::{InnerData, TrackEvent};

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    name: String,
    stats: HashMap<NaiveDate, u32>,
    goal: u32,

    #[serde(default = "default_auto")]
    auto: bool,

    #[serde(skip)]
    inner_data: InnerData,
}

impl Count {
    pub fn new(name: impl AsRef<str>, goal: u32, auto: bool) -> Self {
        return Count {
            name: name.as_ref().to_owned(),
            stats: HashMap::new(),
            goal,
            auto,
            inner_data: Default::default(),
        };
    }
}

impl Habit for Count {
    type HabitType = u32;

    fn name(&self) -> String {
        return self.name.clone();
    }
    fn set_name(&mut self, n: impl AsRef<str>) {
        self.name = n.as_ref().to_owned();
    }
    fn kind(&self) -> GoalKind {
        GoalKind::Count(self.goal)
    }
    fn set_goal(&mut self, g: Self::HabitType) {
        self.goal = g;
    }
    fn get_by_date(&self, date: NaiveDate) -> Option<&Self::HabitType> {
        self.stats.get(&date)
    }
    fn insert_entry(&mut self, date: NaiveDate, val: Self::HabitType) {
        *self.stats.entry(date).or_insert(val) = val;
    }
    fn reached_goal(&self, date: NaiveDate) -> bool {
        if let Some(val) = self.stats.get(&date) {
            if val >= &self.goal {
                return true;
            }
        }
        return false;
    }
    fn remaining(&self, date: NaiveDate) -> u32 {
        if self.reached_goal(date) {
            return 0;
        } else {
            if let Some(val) = self.stats.get(&date) {
                return self.goal - val;
            } else {
                return self.goal;
            }
        }
    }
    fn goal(&self) -> u32 {
        return self.goal;
    }
    fn modify(&mut self, date: NaiveDate, event: TrackEvent) {
        if let Some(val) = self.stats.get_mut(&date) {
            match event {
                TrackEvent::Increment => *val += 1,
                TrackEvent::Decrement => {
                    if *val > 0 {
                        *val -= 1
                    } else {
                        self.stats.remove(&date);
                    };
                }
            }
        } else {
            match event {
                TrackEvent::Increment => self.insert_entry(date, 1),
                _ => {}
            };
        }
    }
    fn inner_data_ref(&self) -> &InnerData {
        &self.inner_data
    }
    fn inner_data_mut_ref(&mut self) -> &mut InnerData {
        &mut self.inner_data
    }
    fn is_auto(&self) -> bool {
        self.auto
    }
}
