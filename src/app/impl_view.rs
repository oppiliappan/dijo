use std::f64;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use cursive::direction::{Absolute, Direction};
use cursive::event::{Event, EventResult, Key};
use cursive::theme::Color;
use cursive::view::{CannotFocus, View};
use cursive::{Printer, Vec2};
use notify::DebouncedEvent;

use crate::app::{App, MessageKind};
use crate::habit::{HabitWrapper, ViewMode};
use crate::utils::{self, GRID_WIDTH, VIEW_HEIGHT, VIEW_WIDTH};
use crate::CONFIGURATION;

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

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Err(CannotFocus)
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

        // TODO: Using a match statment, it won't be able to match on a dynamic variable
        let move_up    = e == Event::Key(Key::Up)    || e == Event::Char(CONFIGURATION.move_up());
        let move_down  = e == Event::Key(Key::Down)  || e == Event::Char(CONFIGURATION.move_down());
        let move_left  = e == Event::Key(Key::Left)  || e == Event::Shift(Key::Tab) || e == Event::Char(CONFIGURATION.move_left());
        let move_right = e == Event::Key(Key::Right) || e == Event::Key(Key::Tab)   || e == Event::Char(CONFIGURATION.move_right());

        let move_prev_day   = e == Event::Char(CONFIGURATION.move_prev_day());
        let move_next_day   = e == Event::Char(CONFIGURATION.move_next_day());
        let move_prev_week  = e == Event::Char(CONFIGURATION.move_prev_week());
        let move_next_week  = e == Event::Char(CONFIGURATION.move_next_week());
        let move_prev_month = e == Event::Char(CONFIGURATION.move_prev_month());
        let move_next_month = e == Event::Char(CONFIGURATION.move_next_month());

        let weekly_stats  = e == Event::Char(CONFIGURATION.show_weekly_stats());
        let monthly_stats = e == Event::Char(CONFIGURATION.show_monthly_stats());
        let clear_message = e == Event::CtrlChar(CONFIGURATION.clear_msg());
        let escape        = e == Event::Key(Key::Esc);

        if move_up {
            self.set_focus(Absolute::Up);
            return EventResult::Consumed(None);
        } else if move_down {
            self.set_focus(Absolute::Down);
            return EventResult::Consumed(None);
        } else if move_left {
            self.set_focus(Absolute::Left);
            return EventResult::Consumed(None);
        } else if move_right {
            self.set_focus(Absolute::Right);
            return EventResult::Consumed(None);
        } else if move_prev_day {
            self.move_cursor(Absolute::Left);
            return EventResult::Consumed(None);
        } else if move_next_day {
            self.move_cursor(Absolute::Right);
            return EventResult::Consumed(None);
        } else if move_prev_week {
            self.move_cursor(Absolute::Up);
            return EventResult::Consumed(None);
        } else if move_next_week {
            self.move_cursor(Absolute::Down);
            return EventResult::Consumed(None);
        } else if move_prev_month {
            self.sift_backward();
            return EventResult::Consumed(None);
        } else if move_next_month {
            self.sift_forward();
            return EventResult::Consumed(None);
        } else if clear_message {
            self.message.clear();
            self.message.set_kind(MessageKind::Info);
            return EventResult::Consumed(None);
        } else if weekly_stats {
            if self.habits.is_empty() {
                return EventResult::Consumed(None);
            }
            if self.habits[self.focus].inner_data_ref().view_mode() == ViewMode::Week {
                self.set_mode(ViewMode::Day)
            } else {
                self.set_mode(ViewMode::Week)
            }
            return EventResult::Consumed(None);

        } else if monthly_stats {
            for habit in self.habits.iter_mut() {
                habit.inner_data_mut_ref().set_view_mode(ViewMode::Week);
            }
            return EventResult::Consumed(None);
        } else if escape {
            for habit in self.habits.iter_mut() {
                habit.inner_data_mut_ref().set_view_mode(ViewMode::Day);
            }
            self.reset_cursor();
            return EventResult::Consumed(None);
        }
        else {
            if self.habits.is_empty() { return EventResult::Ignored; }
            else { self.habits[self.focus].on_event(e) }
        }
        //     Event::Char('}') => {
        //         self.reset_cursor();
        //         return EventResult::Consumed(None);
        //     }
    }
}
