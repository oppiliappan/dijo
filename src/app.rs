use std::default::Default;
use std::f64;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use cursive::direction::{Absolute, Direction};
use cursive::event::{Event, EventResult, Key};
use cursive::view::View;
use cursive::{Printer, Vec2};

use chrono::Local;

use crate::habit::{Bit, Count, HabitWrapper, ViewMode};
use crate::utils;
use crate::Command;
use crate::CONFIGURATION;

struct StatusLine(String, String);

pub struct App {
    // holds app data
    habits: Vec<Box<dyn HabitWrapper>>,

    focus: usize,
    view_month_offset: u32,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    pub fn new() -> Self {
        return App {
            habits: vec![],
            focus: 0,
            view_month_offset: 0,
        };
    }

    pub fn add_habit(&mut self, h: Box<dyn HabitWrapper>) {
        self.habits.push(h);
    }

    pub fn delete_by_name(&mut self, name: &str) {
        self.habits.retain(|h| h.get_name() != name);
    }

    pub fn set_mode(&mut self, mode: ViewMode) {
        if !self.habits.is_empty() {
            self.habits[self.focus].set_view_mode(mode);
        }
    }

    pub fn set_view_month_offset(&mut self, offset: u32) {
        self.view_month_offset = offset;
        for v in self.habits.iter_mut() {
            v.set_view_month_offset(offset);
        }
    }

    pub fn sift_backward(&mut self) {
        self.view_month_offset += 1;
        for v in self.habits.iter_mut() {
            v.set_view_month_offset(self.view_month_offset);
        }
    }

    pub fn sift_forward(&mut self) {
        if self.view_month_offset > 0 {
            self.view_month_offset -= 1;
            for v in self.habits.iter_mut() {
                v.set_view_month_offset(self.view_month_offset);
            }
        }
    }

    fn set_focus(&mut self, d: Absolute) {
        let grid_width = CONFIGURATION.grid_width;
        match d {
            Absolute::Right => {
                if self.focus != self.habits.len() - 1 {
                    self.focus += 1;
                }
            }
            Absolute::Left => {
                if self.focus != 0 {
                    self.focus -= 1;
                }
            }
            Absolute::Down => {
                if self.focus + grid_width < self.habits.len() - 1 {
                    self.focus += grid_width;
                } else {
                    self.focus = self.habits.len() - 1;
                }
            }
            Absolute::Up => {
                if self.focus as isize - grid_width as isize >= 0 {
                    self.focus -= grid_width;
                } else {
                    self.focus = 0;
                }
            }
            Absolute::None => {}
        }
    }

    fn status(&self) -> StatusLine {
        let today = chrono::Local::now().naive_utc().date();
        let remaining = self.habits.iter().map(|h| h.remaining(today)).sum::<u32>();
        let total = self.habits.iter().map(|h| h.goal()).sum::<u32>();
        let completed = total - remaining;

        let timestamp = if self.view_month_offset == 0 {
            format!(
                "{:>width$}",
                Local::now().date().format("%d/%b/%y"),
                width = CONFIGURATION.view_width * CONFIGURATION.grid_width
            )
        } else {
            let months = self.view_month_offset;
            format!(
                "{:>width$}",
                format!("{} months ago", months),
                width = CONFIGURATION.view_width * CONFIGURATION.grid_width
            )
        };

        StatusLine {
            0: format!("Today: {} completed, {} remaining", completed, remaining),
            1: timestamp,
        }
    }

    fn max_size(&self) -> Vec2 {
        let grid_width = CONFIGURATION.grid_width;
        let width = {
            if self.habits.len() > 0 {
                grid_width * CONFIGURATION.view_width
            } else {
                0
            }
        };
        let height = {
            if self.habits.len() > 0 {
                (CONFIGURATION.view_height as f64
                    * (self.habits.len() as f64 / grid_width as f64).ceil())
                    as usize
            } else {
                0
            }
        };
        Vec2::new(width, height + 2)
    }

    pub fn load_state() -> Self {
        let data_file = utils::data_file();
        if let Ok(ref mut file) = File::open(data_file) {
            let mut j = String::new();
            file.read_to_string(&mut j);
            return App {
                habits: serde_json::from_str(&j).unwrap(),
                ..Default::default()
            };
        } else {
            Self::new()
        }
    }

    // this function does IO
    // TODO: convert this into non-blocking async function
    fn save_state(&self) {
        let j = serde_json::to_string_pretty(&self.habits).unwrap();
        let data_file = utils::data_file();

        match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(data_file)
        {
            Ok(ref mut file) => file.write_all(j.as_bytes()).unwrap(),
            Err(_) => panic!("Unable to write!"),
        };
    }

    pub fn parse_command(&mut self, input: &str) {
        let c = Command::from_string(input);
        match c {
            Command::Add(name, kind, goal) => {
                if kind == "count" {
                    self.add_habit(Box::new(Count::new(name, goal.unwrap_or(0))));
                } else if kind == "bit" {
                    self.add_habit(Box::new(Bit::new(name)));
                }
            }
            Command::Delete(name) => {
                self.delete_by_name(&name);
                self.focus = 0;
            }
            Command::Quit => self.save_state(),
            Command::MonthNext => self.sift_forward(),
            Command::MonthPrev => self.sift_backward(),
            _ => {
                eprintln!("UNKNOWN COMMAND!");
            }
        }
    }
}

impl View for App {
    fn draw(&self, printer: &Printer) {
        let grid_width = CONFIGURATION.grid_width;
        let mut offset = Vec2::zero();
        for (idx, i) in self.habits.iter().enumerate() {
            if idx >= grid_width && idx % grid_width == 0 {
                offset = offset.map_y(|y| y + CONFIGURATION.view_height).map_x(|_| 0);
            }
            i.draw(&printer.offset(offset).focused(self.focus == idx));
            offset = offset.map_x(|x| x + CONFIGURATION.view_width + 2);
        }

        offset = offset.map_x(|_| 0).map_y(|_| self.max_size().y - 2);
        printer.print(offset, &self.status().1); // right status
        printer.print(offset, &self.status().0); // left status
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let grid_width = CONFIGURATION.grid_width;
        let view_width = CONFIGURATION.view_width;
        let view_height = CONFIGURATION.view_height;
        let width = {
            if self.habits.len() > 0 {
                grid_width * (view_width + 2)
            } else {
                0
            }
        };
        let height = {
            if self.habits.len() > 0 {
                (view_height as f64 * (self.habits.len() as f64 / grid_width as f64).ceil())
                    as usize
                    + 2 // to acoomodate statusline and commandline
            } else {
                0
            }
        };
        Vec2::new(width, height)
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        false
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        match e {
            Event::Key(Key::Right) | Event::Key(Key::Tab) | Event::Char('l') => {
                self.set_focus(Absolute::Right);
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Left) | Event::Shift(Key::Tab) | Event::Char('h') => {
                self.set_focus(Absolute::Left);
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Up) | Event::Char('k') => {
                self.set_focus(Absolute::Up);
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Down) | Event::Char('j') => {
                self.set_focus(Absolute::Down);
                return EventResult::Consumed(None);
            }
            Event::Char('d') => {
                if self.habits.is_empty() {
                    return EventResult::Consumed(None);
                }
                self.habits.remove(self.focus);
                self.focus = self.focus.checked_sub(1).unwrap_or(0);
                return EventResult::Consumed(None);
            }
            Event::Char('w') => {
                // helper bind to test write to file
                let j = serde_json::to_string_pretty(&self.habits).unwrap();
                let mut file = File::create("foo.txt").unwrap();
                file.write_all(j.as_bytes()).unwrap();
                return EventResult::Consumed(None);
            }
            Event::Char('q') => {
                self.save_state();
                return EventResult::with_cb(|s| s.quit());
            }
            Event::Char('v') => {
                if self.habits.is_empty() {
                    return EventResult::Consumed(None);
                }
                if self.habits[self.focus].view_mode() == ViewMode::Week {
                    self.set_mode(ViewMode::Day)
                } else {
                    self.set_mode(ViewMode::Week)
                }
                return EventResult::Consumed(None);
            }
            Event::Char('V') => {
                for habit in self.habits.iter_mut() {
                    habit.set_view_mode(ViewMode::Week);
                }
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Esc) => {
                for habit in self.habits.iter_mut() {
                    habit.set_view_mode(ViewMode::Day);
                }
                return EventResult::Consumed(None);
            }

            /* We want sifting to be an app level function,
             * that later trickles down into each habit
             * */
            Event::Char(']') => {
                self.sift_forward();
                return EventResult::Consumed(None);
            }
            Event::Char('[') => {
                self.sift_backward();
                return EventResult::Consumed(None);
            }
            Event::Char('}') => {
                self.set_view_month_offset(0);
                return EventResult::Consumed(None);
            }

            /* Every keybind that is not caught by App trickles
             * down to the focused Habit We sift back to today
             * before performing any action, "refocusing" the cursor
             * */
            _ => {
                if self.habits.is_empty() {
                    return EventResult::Ignored;
                }
                self.set_view_month_offset(0);
                self.habits[self.focus].on_event(e)
            }
        }
    }
}
