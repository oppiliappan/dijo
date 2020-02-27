#![allow(unused_must_use)]

use chrono::NaiveDate;
use std::f64;

use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::view::{View, ViewWrapper};
//use cursive::views::{Dialog, EditView, LinearLayout, ListView, SelectView};
use cursive::Cursive;
use cursive::{Printer, Vec2};

mod habit;
use crate::habit::{Habit, HabitTrait, HabitType};

mod views;
use crate::views::habitview::HabitView;

mod theme;

#[derive(PartialEq)]
enum ViewMode {
    Day,
    Month,
    Year,
}

struct App {
    habits: Vec<HabitView>,
    view_mode: ViewMode,
    focus: usize,
    grid_width: usize,

    padding: usize,
}

impl App {
    fn new() -> Self {
        return App {
            habits: vec![],
            view_mode: ViewMode::Day,
            focus: 0,
            grid_width: 3,
            padding: 12,
        };
    }
    fn add_habit(mut self, h: HabitView) -> Self {
        self.habits.push(h);
        return self;
    }
    fn set_mode(mut self, set_mode: ViewMode) -> Self {
        if set_mode != self.view_mode {
            self.view_mode = set_mode
        }
        return self;
    }
    fn focus_right(&mut self) {
        if self.focus != self.habits.len() - 1 {
            self.focus += 1;
        }
    }
    fn focus_left(&mut self) {
        if self.focus != 0 {
            self.focus -= 1;
        }
    }
    fn focus_down(&mut self) {
        if self.focus + self.grid_width < self.habits.len() - 1 {
            self.focus += self.grid_width;
        } else {
            self.focus = self.habits.len() - 1;
        }
    }
    fn focus_up(&mut self) {
        if self.focus as isize - self.grid_width as isize >= 0 {
            self.focus -= self.grid_width;
        } else {
            self.focus = 0;
        }
    }
    fn status(&self) -> String {
        return format!(
            "{} total, {} remaining",
            self.habits.iter().map(|h| h.total()).sum::<u32>(),
            self.habits.iter().map(|h| h.remaining()).sum::<u32>()
        );
    }
    fn max_size(&self) -> Vec2 {
        let grid_width = self.grid_width;
        let width = {
            if self.habits.len() > 0 {
                grid_width * self.habits[0].get_size().x
            } else {
                0
            }
        };
        let height = {
            if self.habits.len() > 0 {
                (self.habits[0].get_size().y as f64
                    * (self.habits.len() as f64 / grid_width as f64).ceil())
                    as usize
            } else {
                0
            }
        };
        Vec2::new(width, height)
    }
}

impl cursive::view::View for App {
    fn draw(&self, printer: &Printer) {
        let grid_width = self.grid_width;
        let mut offset = Vec2::zero();
        for (idx, i) in self.habits.iter().enumerate() {
            if idx >= grid_width && idx % grid_width == 0 {
                offset = offset.map_y(|y| y + i.get_size().y).map_x(|_| 0);
            }
            i.draw(&printer.offset(offset).focused(self.focus == idx));
            offset = offset.map_x(|x| x + i.get_size().x);
        }
        offset = offset.map_x(|_| 0).map_y(|_| self.max_size().y - 2);
        printer.print(offset, &self.status());
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let grid_width = self.grid_width;
        let width = {
            if self.habits.len() > 0 {
                grid_width * self.habits[0].get_size().x
            } else {
                0
            }
        };
        let height = {
            if self.habits.len() > 0 {
                (self.habits[0].get_size().y as f64
                    * (self.habits.len() as f64 / grid_width as f64).ceil())
                    as usize
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
                self.focus_right();
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Left) | Event::Shift(Key::Tab) | Event::Char('h') => {
                self.focus_left();
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Up) | Event::Char('k') => {
                self.focus_up();
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Down) | Event::Char('j') => {
                self.focus_down();
                return EventResult::Consumed(None);
            }
            _ => self.habits[self.focus].on_event(e),
        }
    }
}

fn main() {
    let mut s = Cursive::default();

    let mut gymming = Habit::new("gym", HabitType::Count(5));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 11), HabitType::Count(7));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 12), HabitType::Count(8));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 13), HabitType::Count(9));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 14), HabitType::Count(10));
    gymming.insert_entry(NaiveDate::from_ymd(2020, 2, 15), HabitType::Count(11));

    let mut reading = Habit::new("read", HabitType::Bit(true));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 11), HabitType::Bit(true));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 12), HabitType::Bit(false));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 13), HabitType::Bit(true));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 14), HabitType::Bit(false));
    reading.insert_entry(NaiveDate::from_ymd(2020, 2, 15), HabitType::Bit(true));

    let mut walking = Habit::new("walk", HabitType::Bit(true));
    walking.insert_entry(NaiveDate::from_ymd(2020, 2, 11), HabitType::Bit(true));
    walking.insert_entry(NaiveDate::from_ymd(2020, 2, 12), HabitType::Bit(false));
    walking.insert_entry(NaiveDate::from_ymd(2020, 2, 13), HabitType::Bit(true));
    walking.insert_entry(NaiveDate::from_ymd(2020, 2, 14), HabitType::Bit(false));
    walking.insert_entry(NaiveDate::from_ymd(2020, 2, 15), HabitType::Bit(true));

    let gym_view = HabitView::new(gymming);
    let read_view = HabitView::new(reading);
    let walk_view = HabitView::new(walking);

    s.add_global_callback('q', |a| a.quit());
    let app = App::new()
        .add_habit(gym_view)
        .add_habit(read_view)
        .add_habit(walk_view)
        .set_mode(ViewMode::Month);

    s.add_layer(app);

    s.set_theme(theme::theme_gen());
    s.run();
}
