use std::cmp::{Eq, Ord, PartialEq};
use std::collections::HashMap;
use std::default::Default;
use std::fmt;
use std::ops::{Add, Sub};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::command::GoalKind;
use crate::habit::prelude::default_auto;
use crate::habit::traits::Habit;
use crate::habit::{InnerData, TrackEvent};

#[derive(Copy, Clone, Debug, Ord, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FloatData {
    value: u32,
    precision: u8,
}

impl FloatData {
    pub fn add(self, v: u32) -> Self {
        let f = FloatData {
            value: v,
            precision: self.precision,
        };
        self + f
    }
    pub fn sub(self, v: u32) -> Self {
        let f = FloatData {
            value: v,
            precision: self.precision,
        };
        self - f
    }
    pub fn zero() -> Self {
        FloatData {
            value: 0,
            precision: 0,
        }
    }
}

impl fmt::Display for FloatData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let characteristic = self.value / (10 * self.precision as u32);
        let mantissa = self.value % (10 * self.precision as u32);
        let s = if characteristic == 0 {
            format!(".{}", mantissa)
        } else if mantissa == 0 {
            format!("{}", characteristic)
        } else {
            format!("{}.{}", characteristic, mantissa)
        };
        write!(f, "{:^3}", s)
    }
}

impl Add for FloatData {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            precision: self.precision,
        }
    }
}

impl Sub for FloatData {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value.saturating_sub(other.value),
            precision: self.precision,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Float {
    name: String,
    stats: HashMap<NaiveDate, FloatData>,
    goal: FloatData,
    precision: u8,
    #[serde(default = "default_auto")]
    auto: bool,

    #[serde(skip)]
    inner_data: InnerData,
}

impl Float {
    pub fn new(name: impl AsRef<str>, goal: u32, precision: u8, auto: bool) -> Self {
        return Float {
            name: name.as_ref().to_owned(),
            stats: HashMap::new(),
            goal: FloatData {
                value: goal,
                precision,
            },
            precision,
            auto,
            inner_data: Default::default(),
        };
    }
}

impl Habit for Float {
    type HabitType = FloatData;

    fn name(&self) -> String {
        return self.name.clone();
    }
    fn set_name(&mut self, n: impl AsRef<str>) {
        self.name = n.as_ref().to_owned();
    }
    fn kind(&self) -> GoalKind {
        GoalKind::Float(self.goal.value, self.goal.precision)
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
            if let Some(&val) = self.stats.get(&date) {
                return (self.goal - val).value;
            } else {
                return self.goal.value;
            }
        }
    }
    fn goal(&self) -> u32 {
        return self.goal.value;
    }
    fn modify(&mut self, date: NaiveDate, event: TrackEvent) {
        if let Some(val) = self.stats.get_mut(&date) {
            match event {
                TrackEvent::Increment => *val = val.add(1),
                TrackEvent::Decrement => {
                    if *val > FloatData::zero() {
                        *val = val.sub(1);
                    } else {
                        self.stats.remove(&date);
                    };
                }
            }
        } else {
            match event {
                TrackEvent::Increment => self.insert_entry(
                    date,
                    FloatData {
                        value: 1,
                        precision: self.precision,
                    },
                ),
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
