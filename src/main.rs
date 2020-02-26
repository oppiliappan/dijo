#![allow(unused_must_use)]

use chrono::NaiveDate;
use std::f64;

use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::view::View;
use cursive::views::{Dialog, EditView, LinearLayout, ListView, SelectView};
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
    status: String,
    view_mode: ViewMode,
    focus: usize,

    padding: usize,
}

impl App {
    fn new() -> Self {
        return App {
            habits: vec![],
            status: "".to_string(),
            view_mode: ViewMode::Day,
            focus: 0,
            padding: 2,
        };
    }
    fn add_habit(&mut self, h: HabitView) -> &mut Self {
        self.habits.push(h);
        return self;
    }
    fn set_mode(&mut self, set_mode: ViewMode) -> &mut Self {
        if set_mode != self.view_mode {
            self.view_mode = set_mode
        }
        return self;
    }
}

impl View for App {
    fn draw(&self, printer: &Printer) {
        let grid_width = 3;
        let width = {
            if self.habits.len() > 0 {
                grid_width * self.habits[0].get_size().x
            } else {
                0
            }
        };
        let height = {
            if self.habits.len() > 0 {
                (self.habits[0].get_size().y as f64 / grid_width as f64).ceil() as usize
            } else {
                0
            }
        };
        let mut offset = Vec2::new(width + self.padding, height + self.padding);
        for (idx, i) in self.habits.iter().enumerate() {
            i.draw(&printer.offset(offset).focused(self.focus == idx));
            offset = offset.saturating_add(i.get_size());
        }
    }
    fn required_size(&mut self, _: Vec2) -> Vec2 {
        todo!()
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        todo!()
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        todo!()
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

    let gym_title = gymming.get_name();
    let gym_view = HabitView::new(gymming);

    let read_title = reading.get_name();
    let read_view = HabitView::new(reading);

    s.add_global_callback('q', |a| a.quit());
    s.add_layer(
        LinearLayout::horizontal()
            .child(Dialog::around(gym_view).title(gym_title))
            .child(Dialog::around(read_view).title(read_title)),
    );

    s.set_theme(theme::theme_gen());
    s.run();
}
