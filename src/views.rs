use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::theme::Style;
use cursive::view::View;
use cursive::{Printer, Vec2};

use chrono::prelude::*;
use chrono::{Duration, Local, NaiveDate};

use crate::habit::{Bit, Count, Habit, TrackEvent};
use crate::CONFIGURATION;

pub trait ShadowView {
    fn draw(&self, printer: &Printer);
    fn required_size(&mut self, _: Vec2) -> Vec2;
    fn take_focus(&mut self, _: Direction) -> bool;
    fn on_event(&mut self, e: Event) -> EventResult;
}

// the only way to not rewrite each View implementation for trait
// objects of Habit is to rewrite the View trait itself.
impl<T> ShadowView for T
where
    T: Habit,
    T::HabitType: std::fmt::Display,
{
    fn draw(&self, printer: &Printer) {
        let now = if self.view_month_offset() == 0 {
            Local::today()
        } else {
            Local::today()
                .checked_sub_signed(Duration::weeks(4 * self.view_month_offset() as i64))
                .unwrap()
        };
        let year = now.year();
        let month = now.month();

        let goal_reached_style = Style::from(CONFIGURATION.reached_color);
        let todo_style = Style::from(CONFIGURATION.todo_color);
        let future_style = Style::from(CONFIGURATION.future_color);

        let goal_reached_today = self.reached_goal(Local::now().naive_utc().date());
        if goal_reached_today {
            printer.with_style(goal_reached_style, |p| p.print((0, 0), "[x]"));
        } else {
            printer.with_style(todo_style, |p| p.print((0, 0), "[ ]"));
        }

        printer.with_style(
            if !printer.focused {
                future_style
            } else {
                Style::none()
            },
            |p| {
                p.print(
                    (4, 0),
                    &format!("{:width$}", self.name(), width = CONFIGURATION.view_width),
                )
            },
        );

        let mut i = 1;
        while let Some(d) = NaiveDate::from_ymd_opt(year, month, i) {
            let day_style;
            if self.reached_goal(d) {
                day_style = goal_reached_style;
            } else {
                day_style = todo_style;
            }
            let coords: Vec2 = ((i % 7) * 3, i / 7 + 2).into();
            if let Some(c) = self.get_by_date(d) {
                printer.with_style(day_style, |p| {
                    p.print(coords, &format!("{:^3}", c));
                });
            } else {
                printer.with_style(future_style, |p| {
                    p.print(coords, &format!("{:^3}", CONFIGURATION.future_chr));
                });
            }
            i += 1;
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (25, 6).into()
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        let now = Local::now().naive_utc().date();
        match e {
            Event::Key(Key::Enter) | Event::Char('n') => {
                self.modify(now, TrackEvent::Increment);
                return EventResult::Consumed(None);
            }
            Event::Key(Key::Backspace) | Event::Char('p') => {
                self.modify(now, TrackEvent::Decrement);
                return EventResult::Consumed(None);
            }
            _ => return EventResult::Ignored,
        }
    }
}

macro_rules! auto_view_impl {
    ($struct_name:ident) => {
        impl View for $struct_name {
            fn draw(&self, printer: &Printer) {
                ShadowView::draw(self, printer);
            }
            fn required_size(&mut self, x: Vec2) -> Vec2 {
                ShadowView::required_size(self, x)
            }
            fn take_focus(&mut self, d: Direction) -> bool {
                ShadowView::take_focus(self, d)
            }
            fn on_event(&mut self, e: Event) -> EventResult {
                ShadowView::on_event(self, e)
            }
        }
    };
}

auto_view_impl!(Count);
auto_view_impl!(Bit);
