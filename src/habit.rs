use std::collections::HashMap;

use chrono::NaiveDate;

#[derive(Debug)]
pub struct Habit<T> {
    name: String,
    stats: HashMap<NaiveDate, T>,
}

impl<T> Habit<T>
where
    T: Copy,
{
    pub fn new(name: &str) -> Habit<T> {
        return Habit {
            name: name.to_owned(),
            stats: HashMap::new(),
        };
    }

    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }

    pub fn get_by_date(&self, date: NaiveDate) -> Option<&T> {
        self.stats.get(&date)
    }

    pub fn insert_entry(&mut self, date: NaiveDate, val: T) {
        *self.stats.entry(date).or_insert(val) = val;
    }
}

impl Habit<bool> {
    pub fn toggle(&mut self, date: NaiveDate) {
        if let Some(v) = self.stats.get_mut(&date) {
            *v ^= true
        } else {
            self.insert_entry(date, true);
        }
    }
}

impl Habit<u32> {
    pub fn increment(&mut self, date: NaiveDate) {
        if let Some(v) = self.stats.get_mut(&date) {
            *v += 1
        }
    }
    pub fn decrement(&mut self, date: NaiveDate) {
        if let Some(v) = self.stats.get_mut(&date) {
            if *v >= 1 {
                *v -= 1;
            } else {
                *v = 0;
            };
        }
    }
    pub fn set(&mut self, date: NaiveDate, val: u32) {
        *self.stats.entry(date).or_insert(val) = val;
    }
}
