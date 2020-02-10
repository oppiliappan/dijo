use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::view::View;
use cursive::{Printer, Vec2};

use chrono::prelude::*;
use chrono::{Local, NaiveDate};

use crate::habit::Habit;

pub struct BitView {
    habit: Habit<bool>,
    true_chr: char,
    false_chr: char,
    future_chr: char,

    view_width: u32,
    view_height: u32,
    // color config
}

impl BitView {
    pub fn new(habit: Habit<bool>) -> Self {
        return BitView {
            habit,
            true_chr: '·',
            false_chr: '·',
            future_chr: '·',
            view_width: 21,
            view_height: 9,
        };
    }
    pub fn get_title(&self) -> String {
        return self.habit.get_name().to_owned();
    }
}

impl View for BitView {
    fn draw(&self, printer: &Printer) {
        let now = Local::now();
        let year = now.year();
        let month = now.month();

        let true_style = Style::from(Color::Dark(BaseColor::Cyan));
        let false_style = Style::from(Color::Dark(BaseColor::Magenta));
        let future_style = Style::from(Color::Light(BaseColor::Black));

        for i in 1..=31 {
            let day = NaiveDate::from_ymd_opt(year, month, i);
            let day_style;

            if let Some(d) = day {
                let day_status = self.habit.get_by_date(d).unwrap_or(&false);
                let coords = ((i % 7) * 3, i / 7 + 2);
                let day_chr;
                if d <= now.naive_utc().date() {
                    if *day_status {
                        day_chr = self.true_chr;
                        day_style = true_style;
                    } else {
                        day_chr = self.false_chr;
                        day_style = false_style;
                    }
                } else {
                    day_chr = self.future_chr;
                    day_style = future_style;
                }

                printer.with_style(day_style, |p| {
                    p.print(coords, &format!("{:^3}", day_chr));
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
            Event::Key(Key::Enter) => {
                self.habit.toggle(Local::now().naive_utc().date());
                return EventResult::Consumed(None);
            }
            _ => return EventResult::Ignored,
        }
    }
}
