use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, Style};
use cursive::view::View;
use cursive::{Printer, Vec2};

use chrono::prelude::*;
use chrono::{Local, NaiveDate};

use crate::habit::Habit;

pub struct CountView {
    habit: Habit<u32>,
    future_chr: char,

    view_width: u32,
    view_height: u32,
    // color config
}

impl CountView {
    pub fn new(habit: Habit<u32>) -> Self {
        return CountView {
            habit,
            future_chr: '·',
            view_width: 21,
            view_height: 9,
        };
    }
    pub fn get_title(&self) -> String {
        return self.habit.get_name().to_owned();
    }
}

impl View for CountView {
    fn draw(&self, printer: &Printer) {
        let now = Local::now();
        let year = now.year();
        let month = now.month();

        let goal_reached_style = Style::from(Color::Dark(BaseColor::Cyan));
        let not_reached_style = Style::from(Color::Dark(BaseColor::Magenta));
        let future_style = Style::from(Color::Light(BaseColor::Black));

        for i in 1..=31 {
            let day = NaiveDate::from_ymd_opt(year, month, i);
            let day_style;

            if let Some(d) = day {
                let coords = ((i % 7) * 3, i / 7 + 2);
                let mut day_count = self.habit.get_by_date(d).unwrap_or(&0).to_string();

                if d <= now.naive_utc().date() {
                    if self.habit.reached_goal(d) {
                        day_style = goal_reached_style;
                    } else {
                        day_style = not_reached_style;
                    }
                } else {
                    day_count = format!("{:^3}", self.future_chr);
                    day_style = future_style;
                }

                printer.with_style(day_style, |p| {
                    p.print(coords, &format!("{:^3}", day_count));
                });
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (self.view_width, self.view_height).into()
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        match e {
            Event::Key(Key::Enter) | Event::Char('n') => {
                self.habit.increment(Local::now().naive_utc().date());
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Backspace) | Event::Char('p') => {
                self.habit.decrement(Local::now().naive_utc().date());
                return EventResult::Consumed(None);
            }
            _ => return EventResult::Ignored,
        }
    }
}
