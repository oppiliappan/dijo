use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum HabitType {
    Bit(bool),
    Count(u32),
}

impl HabitType {
    fn inner_bit(&self) -> bool {
        if let HabitType::Bit(v) = self {
            *v
        } else {
            panic!("why");
        }
    }
    fn inner_count(&self) -> u32 {
        if let HabitType::Count(v) = self {
            *v
        } else {
            panic!("why");
        }
    }
}

pub trait HabitTrait {
    fn set_name(&mut self, name: impl AsRef<str>);
    fn set_goal(&mut self, goal: HabitType);
    fn get_name(&self) -> String;
    fn get_by_date(&self, date: NaiveDate) -> Option<&HabitType>;
    fn insert_entry(&mut self, date: NaiveDate, val: HabitType);
    fn reached_goal(&self, date: NaiveDate) -> bool;
    fn remaining(&self, date: NaiveDate) -> u32;
}

#[derive(Serialize, Debug)]
pub struct Habit {
    name: String,
    stats: HashMap<NaiveDate, HabitType>,
    goal: HabitType,
}

pub enum TrackEvent {
    Increment,
    Decrement,
}

impl Habit {
    pub fn new(name: impl AsRef<str>, goal: HabitType) -> Self {
        return Habit {
            name: name.as_ref().to_owned(),
            stats: HashMap::new(),
            goal,
        };
    }

    pub fn modify(&mut self, date: NaiveDate, event: TrackEvent) {
        if let Some(val) = self.stats.get_mut(&date) {
            match val {
                HabitType::Bit(b) => *b ^= true,
                HabitType::Count(c) => match event {
                    TrackEvent::Increment => *c += 1,
                    TrackEvent::Decrement => {
                        if *c > 0 {
                            *c -= 1;
                        } else {
                            *c = 0;
                        }
                    }
                },
            }
        } else {
            match self.goal {
                HabitType::Bit(_) => self.insert_entry(date, HabitType::Bit(true)),
                HabitType::Count(_) => self.insert_entry(date, HabitType::Count(0)),
            }
        }
    }
}

impl HabitTrait for Habit {
    fn get_name(&self) -> String {
        return self.name.to_owned();
    }
    fn set_name(&mut self, n: impl AsRef<str>) {
        self.name = n.as_ref().to_owned();
    }
    fn set_goal(&mut self, g: HabitType) {
        self.goal = g;
    }
    fn get_by_date(&self, date: NaiveDate) -> Option<&HabitType> {
        self.stats.get(&date)
    }
    fn insert_entry(&mut self, date: NaiveDate, val: HabitType) {
        *self.stats.entry(date).or_insert(val) = val;
    }
    fn reached_goal(&self, date: NaiveDate) -> bool {
        if let Some(val) = self.stats.get(&date) {
            match val {
                HabitType::Bit(b) => return *b,
                HabitType::Count(c) => {
                    if *c >= self.goal.inner_count() {
                        return true;
                    }
                }
            };
        }
        return false;
    }
    fn remaining(&self, date: NaiveDate) -> u32 {
        if self.reached_goal(date) {
            return 0;
        } else if let Some(val) = self.stats.get(&date) {
            match val {
                HabitType::Bit(_) => return 1,
                HabitType::Count(c) => return self.goal.inner_count() - *c,
            }
        } else {
            return 0;
        }
    }
}
