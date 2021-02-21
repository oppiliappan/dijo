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
use crate::utils::{self, GRID_WIDTH, VIEW_HEIGHT, VIEW_WIDTH};

impl View for App {
    fn draw(&self, printer: &Printer) {
        let mut offset = Vec2::zero();
        for (idx, habit) in self.habits.iter().enumerate() {
            if idx >= GRID_WIDTH && idx % GRID_WIDTH == 0 {
                offset = offset.map_y(|y| y + VIEW_HEIGHT).map_x(|_| 0);
            }
            habit.draw(&printer.offset(offset).focused(self.focus == idx));
            offset = offset.map_x(|x| x + VIEW_WIDTH + 2);
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
        let width = GRID_WIDTH * (VIEW_WIDTH + 2);
        let height = {
            if self.habits.len() > 0 {
                (VIEW_HEIGHT as f64 * (self.habits.len() as f64 / GRID_WIDTH as f64).ceil())
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

            Event::Char('K') => {
                self.move_cursor(Absolute::Up);
                return EventResult::Consumed(None);
            }
            Event::Char('H') => {
                self.move_cursor(Absolute::Left);
                return EventResult::Consumed(None);
            }
            Event::Char('J') => {
                self.move_cursor(Absolute::Down);
                return EventResult::Consumed(None);
            }
            Event::Char('L') => {
                self.move_cursor(Absolute::Right);
                return EventResult::Consumed(None);
            }

            Event::Char('v') => {
                if self.habits.is_empty() {
                    return EventResult::Consumed(None);
                }
                if self.habits[self.focus].inner_data_ref().view_mode() == ViewMode::Week {
                    self.set_mode(ViewMode::Day)
                } else {
                    self.set_mode(ViewMode::Week)
                }
                return EventResult::Consumed(None);
            }
            Event::Char('V') => {
                for habit in self.habits.iter_mut() {
                    habit.inner_data_mut_ref().set_view_mode(ViewMode::Week);
                }
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Esc) => {
                for habit in self.habits.iter_mut() {
                    habit.inner_data_mut_ref().set_view_mode(ViewMode::Day);
                }
                self.reset_cursor();
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
                self.reset_cursor();
                return EventResult::Consumed(None);
            }
            Event::CtrlChar('l') => {
                self.message.clear();
                self.message.set_kind(MessageKind::Info);
                return EventResult::Consumed(None);
            }

            /* Every keybind that is not caught by App trickles
             * down to the focused habit.
             * */
            _ => {
                if self.habits.is_empty() {
                    return EventResult::Ignored;
                }
                self.habits[self.focus].on_event(e)
            }
        }
    }
}
