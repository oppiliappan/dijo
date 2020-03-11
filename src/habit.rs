use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Serialize;

use cursive::direction::Direction;
use cursive::event::{Event, EventResult};
use cursive::{Printer, Vec2};

use crate::views::ShadowView;
use crate::CONFIGURATION;

pub enum TrackEvent {
    Increment,
    Decrement,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct CustomBool(bool);

use std::fmt;
impl fmt::Display for CustomBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
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

pub trait Habit {
    type HabitType;

    fn set_name(&mut self, name: impl AsRef<str>);
    fn set_goal(&mut self, goal: Self::HabitType);
    fn name(&self) -> String;
    fn get_by_date(&self, date: NaiveDate) -> Option<&Self::HabitType>;
    fn insert_entry(&mut self, date: NaiveDate, val: Self::HabitType);
    fn reached_goal(&self, date: NaiveDate) -> bool;
    fn remaining(&self, date: NaiveDate) -> u32;
    fn total(&self) -> u32;
    fn modify(&mut self, date: NaiveDate, event: TrackEvent);
}

pub trait HabitWrapper {
    fn remaining(&self, date: NaiveDate) -> u32;
    fn total(&self) -> u32;
    fn modify(&mut self, date: NaiveDate, event: TrackEvent);
    fn draw(&self, printer: &Printer);
    fn on_event(&mut self, event: Event) -> EventResult;
    fn required_size(&mut self, _: Vec2) -> Vec2;
    fn take_focus(&mut self, _: Direction) -> bool;
}

impl<T> HabitWrapper for T
where
    T: Habit + ShadowView,
    T::HabitType: std::fmt::Display,
{
    fn remaining(&self, date: NaiveDate) -> u32 {
        Habit::remaining(self, date)
    }
    fn total(&self) -> u32 {
        Habit::total(self)
    }
    fn modify(&mut self, date: NaiveDate, event: TrackEvent) {
        Habit::modify(self, date, event);
    }
    fn draw(&self, printer: &Printer) {
        ShadowView::draw(self, printer)
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        ShadowView::on_event(self, event)
    }
    fn required_size(&mut self, x: Vec2) -> Vec2 {
        ShadowView::required_size(self, x)
    }
    fn take_focus(&mut self, d: Direction) -> bool {
        ShadowView::take_focus(self, d)
    }
}

#[derive(Debug, Serialize)]
pub struct Count {
    name: String,
    stats: HashMap<NaiveDate, u32>,
    goal: u32,
}

impl Count {
    pub fn new(name: impl AsRef<str>, goal: u32) -> Self {
        return Count {
            name: name.as_ref().to_owned(),
            stats: HashMap::new(),
            goal,
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
    fn total(&self) -> u32 {
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
                        *val = 0
                    };
                }
            }
        } else {
            self.insert_entry(date, 1);
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Bit {
    name: String,
    stats: HashMap<NaiveDate, CustomBool>,
    goal: CustomBool,
}

impl Bit {
    pub fn new(name: impl AsRef<str>) -> Self {
        return Bit {
            name: name.as_ref().to_owned(),
            stats: HashMap::new(),
            goal: CustomBool(true),
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
    fn total(&self) -> u32 {
        return 1;
    }
    fn modify(&mut self, date: NaiveDate, _: TrackEvent) {
        if let Some(val) = self.stats.get_mut(&date) {
            *val = (val.0 ^ true).into();
        } else {
            self.insert_entry(date, CustomBool(true));
        }
    }
}
