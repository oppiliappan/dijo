use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key};
use cursive::theme::{ColorStyle, Effect, Style};
use cursive::view::{CannotFocus, View};
use cursive::{Printer, Vec2};

// use chrono::prelude::*;
use chrono::{Local, NaiveDate, Datelike};

use crate::habit::{Bit, Count, Float, Habit, TrackEvent, ViewMode};
// use crate::theme::cursor_bg;
use crate::utils::VIEW_WIDTH;

use crate::CONFIGURATION;

pub trait ShadowView {
    fn draw(&self, printer: &Printer);
    fn required_size(&mut self, _: Vec2) -> Vec2;
    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus>;
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
        // Actual day of the month
        let actual_day = Local::now().naive_local().day();
        // This is the date at the bottom right. Not the actual date
        let now   = self.inner_data_ref().cursor().0;
        let year  = now.year();
        let month = now.month();
        let day   = now.day();

        let is_today      = now == Local::now().naive_local().date();
        let is_this_month = now.month() == Local::now().month();
        let is_this_year  = now.year() == Local::now().year();

        let goal_reached_style  = Style::from(CONFIGURATION.reached_color());
        let mut future_style    = Style::from(CONFIGURATION.future_color());
        let mut today_style     = Style::from(CONFIGURATION.today_color());
        let mut past_style      = Style::from(CONFIGURATION.inactive_color());
        let stats_bar_bg_style     = Style::from(CONFIGURATION.stats_bar_bg_color());
        let stats_bar_fg_style     = Style::from(CONFIGURATION.stats_bar_fg_color());

        let strikethrough = Style::from(Effect::Strikethrough);
        let bold          = Style::from(Effect::Bold);

        if CONFIGURATION.look.bold_future { future_style = Style::merge(&[future_style, bold]);}
        if CONFIGURATION.look.bold_today  { today_style  = Style::merge(&[today_style, bold]); }
        if CONFIGURATION.look.bold_past   { past_style   = Style::merge(&[past_style, bold]);  }

        let goal_status = is_today && self.reached_goal(Local::now().naive_local().date());
        let marked_incomplete = is_today && self.goal_not_reached(Local::now().naive_local().date());

        printer.with_style(
            Style::merge(&[
                if goal_status || marked_incomplete{ strikethrough } 
                else { Style::none() },

                if !printer.focused { past_style } 
                else { Style::none() },
            ]),
            |p| {
                p.print(
                    (0, 0),
                    &format!(" {:.width$} ", self.name(), width = VIEW_WIDTH - 6),
                );
            },
        );

        let draw_week = |printer: &Printer| {
            let days = (1..31)
                .map(|i| NaiveDate::from_ymd_opt(year, month, i))
                .flatten() // dates 28-31 may not exist, ignore them if they don't
                .collect::<Vec<_>>();
            for (week, line_nr) in days.chunks(7).zip(2..) {
                let weekly_goal = self.goal() * week.len() as u32;
                let is_this_week = week.contains(&Local::now().naive_local().date());
                let remaining = week.iter().map(|&i| self.remaining(i)).sum::<u32>();
                let completions = weekly_goal - remaining;
                let full = VIEW_WIDTH - 8;
                let bars_to_fill = if weekly_goal > 0 {
                    (completions * full as u32) / weekly_goal
                } else {
                    0
                };
                let percentage = if weekly_goal > 0 {
                    (completions as f64 * 100.) / weekly_goal as f64
                } else {
                    0.0
                };
                printer.with_style(stats_bar_bg_style, |p| {
                    p.print((4, line_nr), &"─".repeat(full));
                });
                printer.with_style(stats_bar_fg_style, |p| {
                    p.print((4, line_nr), &"─".repeat(bars_to_fill as usize));
                });
                printer.with_style(
                    if is_this_week {
                        future_style
                    } else {
                        Style::none()
                    },
                    |p| {
                        p.print((0, line_nr), &format!("{:2.0}% ", percentage));
                    },
                );
            }
        };

        let draw_day = |printer: &Printer| {
            let mut i = 0;
            while let Some(d) = NaiveDate::from_ymd_opt(year, month, i + 1) {
                let mut day_style = Style::none();
                let mut tds       = today_style;
                let mut ps        = past_style;
                let fs            = future_style;

                let grs = ColorStyle::front(CONFIGURATION.reached_color());
                let ts  = ColorStyle::front(CONFIGURATION.todo_color());
                let cs  = ColorStyle::back(CONFIGURATION.cursor_color());

                if self.reached_goal(d) {
                    day_style = day_style.combine(Style::from(grs));
                } 
                else {
                    day_style = day_style.combine(Style::from(ts));
                }
                if d == now && printer.focused {
                    day_style = day_style.combine(cs);
                    tds       = tds.combine(cs);
                    ps        = ps.combine(cs);
                }

                let coords: Vec2 = ((i % 7) * 3, i / 7 + 2).into();
                if let Some(c) = self.get_by_date(d) {
                    printer.with_style(day_style, |p| {
                        p.print(coords, &format!("{:^3}", c));
                    });
                } else {
                    if i == (actual_day -1) && is_this_month && is_this_year {
                        printer.with_style(tds, |p| {
                            p.print(coords, &format!("{:^3}", CONFIGURATION.look.future_chr));
                        });
                    }
                    else if i < day || !is_this_month || !is_this_year {
                        printer.with_style(ps, |p| {
                            p.print(coords, &format!("{:^3}", CONFIGURATION.look.future_chr));
                        });
                    } 
                    else {
                        printer.with_style(fs, |p| {
                            p.print(coords, &format!("{:^3}", CONFIGURATION.look.future_chr));
                        });
                    }
                }
                i += 1;
            }
        };

        match self.inner_data_ref().view_mode() {
            ViewMode::Day => draw_day(printer),
            ViewMode::Week => draw_week(printer),
            _ => draw_day(printer),
        };
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (25, 6).into()
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::consumed())
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        let now = self.inner_data_mut_ref().cursor().0;
        if self.is_auto() {
            return EventResult::Ignored;
        }
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
            fn take_focus(&mut self, d: Direction) -> Result<EventResult, CannotFocus> {
                ShadowView::take_focus(self, d)
            }
            fn on_event(&mut self, e: Event) -> EventResult {
                ShadowView::on_event(self, e)
            }
        }
    };
}

macro_rules! generate_view_impls {
    ($($x:ident),*) => (
        $(
            auto_view_impl!($x);
        )*
    );
}

generate_view_impls!(Count, Bit, Float);
