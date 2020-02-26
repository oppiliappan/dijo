use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, Style};
use cursive::view::View;
use cursive::{Printer, Vec2};

use chrono::prelude::*;
use chrono::{Local, NaiveDate};

use crate::habit::{Habit, HabitTrait, HabitType, TrackEvent};

pub struct HabitView {
    habit: Habit,
    // characters to use
    true_chr: char,
    false_chr: char,
    future_chr: char,
    // view dimensions
    view_width: u32,
    view_height: u32,
    // color config
    reached_color: Color,
    todo_color: Color,
    future_color: Color,
}

impl HabitView {
    pub fn new(habit: Habit) -> Self {
        return HabitView {
            habit,
            true_chr: '·',
            false_chr: '·',
            future_chr: '·',
            view_width: 21,
            view_height: 9,
            reached_color: Color::Dark(BaseColor::Cyan),
            todo_color: Color::Dark(BaseColor::Magenta),
            future_color: Color::Light(BaseColor::Black),
        };
    }
    pub fn get_title(&self) -> String {
        return self.habit.get_name().to_owned();
    }
    pub fn get_size(&self) -> Vec2 {
        (self.view_width, self.view_height).into()
    }
}

impl View for HabitView {
    fn draw(&self, printer: &Printer) {
        let now = Local::now();
        let year = now.year();
        let month = now.month();

        let goal_reached_style = Style::from(self.reached_color);
        let todo_style = Style::from(self.todo_color);
        let future_style = Style::from(self.future_color);

        for i in 1..=31 {
            let day = NaiveDate::from_ymd_opt(year, month, i);
            let mut day_style;

            if let Some(d) = day {
                if self.habit.reached_goal(d) {
                    day_style = goal_reached_style;
                } else {
                    day_style = todo_style;
                }
                let coords = ((i % 7) * 3, i / 7 + 2);
                let day_chr: Box<dyn std::fmt::Display> = match self.habit.get_by_date(d) {
                    Some(val) => match val {
                        HabitType::Bit(b) => {
                            if *b {
                                Box::new(self.true_chr)
                            } else {
                                Box::new(self.false_chr)
                            }
                        }
                        HabitType::Count(c) => Box::new(c.to_string()),
                    },
                    None => {
                        day_style = future_style;
                        Box::new(self.future_chr)
                    }
                };
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
        let now = Local::now().naive_utc().date();
        match e {
            Event::Key(Key::Enter) | Event::Char('n') => {
                self.habit.modify(now, TrackEvent::Increment);
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Backspace) | Event::Char('p') => {
                self.habit.modify(now, TrackEvent::Decrement);
                return EventResult::Consumed(None);
            }
            _ => return EventResult::Ignored,
        }
    }
}
