use std::f64;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use cursive::direction::{Absolute, Direction};
use cursive::event::{Event, EventResult, Key};
use cursive::theme::Color;
use cursive::view::View;
use cursive::{Printer, Vec2};
use notify::DebouncedEvent;

use crate::app::{App, MessageKind};
use crate::habit::{HabitWrapper, ViewMode};
use crate::utils;
use crate::CONFIGURATION;

impl View for App {
    fn draw(&self, printer: &Printer) {
        let grid_width = CONFIGURATION.grid_width;
        let view_width = CONFIGURATION.view_width;
        let view_height = CONFIGURATION.view_height;
        let mut offset = Vec2::zero();
        for (idx, habit) in self.habits.iter().enumerate() {
            if idx >= grid_width && idx % grid_width == 0 {
                offset = offset.map_y(|y| y + view_height).map_x(|_| 0);
            }
            habit.draw(&printer.offset(offset).focused(self.focus == idx));
            offset = offset.map_x(|x| x + view_width + 2);
        }

        offset = offset.map_x(|_| 0).map_y(|_| self.max_size().y - 2);

        let status = self.status();
        printer.print(offset, &status.0); // left status

        let full = self.max_size().x;
        offset = offset.map_x(|_| full - status.1.len());
        printer.print(offset, &status.1); // right status

        offset = offset.map_x(|_| 0).map_y(|_| self.max_size().y - 1);
        printer.with_style(Color::from(self.message.kind()), |p| {
            p.print(offset, self.message.contents())
        });
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let grid_width = CONFIGURATION.grid_width;
        let view_width = CONFIGURATION.view_width;
        let view_height = CONFIGURATION.view_height;
        let width = grid_width * (view_width + 2);
        let height = {
            if self.habits.len() > 0 {
                (view_height as f64 * (self.habits.len() as f64 / grid_width as f64).ceil())
                    as usize
            } else {
                0
            }
        };
        Vec2::new(width, height + 2)
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        false
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        match self.file_event_recv.try_recv() {
            Ok(DebouncedEvent::Write(_)) => {
                let read_from_file = |file: PathBuf| -> Vec<Box<dyn HabitWrapper>> {
                    if let Ok(ref mut f) = File::open(file) {
                        let mut j = String::new();
                        f.read_to_string(&mut j);
                        return serde_json::from_str(&j).unwrap();
                    } else {
                        return Vec::new();
                    }
                };
                let auto = read_from_file(utils::auto_habit_file());
                self.habits.retain(|x| !x.is_auto());
                self.habits.extend(auto);
            }
            _ => {}
        };
        if self.habits.is_empty() {
            return EventResult::Ignored;
        }
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
            Event::CtrlChar('l') => {
                self.message.clear();
                self.message.set_kind(MessageKind::Info);
                return EventResult::Consumed(None);
            }

            /* Every keybind that is not caught by App trickles
             * down to the focused habit. We sift back to today
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
