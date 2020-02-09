#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use chrono::prelude::*;
use chrono::{Local, NaiveDate};
use serde::Serialize;

use cursive::direction::{Direction, Orientation};
use cursive::event::{Event, EventResult, Key};
use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::{BaseColor, BorderStyle, Palette, Theme};
use cursive::view::View;
use cursive::views::{Dialog, DummyView, LinearLayout, TextView};
use cursive::{Cursive, Printer, Vec2};

use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

#[derive(Debug)]
struct Habit<T> {
    name: String,
    stats: HashMap<NaiveDate, T>,
}

impl<T> Habit<T>
where
    T: Copy,
{
    fn new(name: &str) -> Habit<T> {
        return Habit {
            name: name.to_owned(),
            stats: HashMap::new(),
        };
    }

    fn get_by_date(&self, date: NaiveDate) -> Option<&T> {
        self.stats.get(&date)
    }

    fn insert_entry(&mut self, date: NaiveDate, val: T) {
        *self.stats.entry(date).or_insert(val) = val;
    }
}

impl Habit<bool> {
    fn toggle(&mut self, date: NaiveDate) {
        if let Some(v) = self.stats.get_mut(&date) {
            *v ^= true
        } else {
            self.insert_entry(date, true);
        }
    }
}

impl Habit<u32> {
    fn increment(&mut self, date: NaiveDate) {
        if let Some(v) = self.stats.get_mut(&date) {
            *v += 1
        }
    }
    fn decrement(&mut self, date: NaiveDate) {
        if let Some(v) = self.stats.get_mut(&date) {
            if *v >= 1 {
                *v -= 1;
            } else {
                *v = 0;
            };
        }
    }
    fn set(&mut self, date: NaiveDate, val: u32) {
        *self.stats.entry(date).or_insert(val) = val;
    }
}

enum ViewMode {
    Daily,
    Monthly,
}

struct BitView {
    habit: Habit<bool>,
    true_chr: char,
    false_chr: char,
    future_chr: char,

    view_width: u32,
    view_height: u32,
    // color config
}

impl BitView {
    fn new(habit: Habit<bool>) -> Self {
        return BitView {
            habit,
            true_chr: 'x',
            false_chr: 'o',
            future_chr: '.',
            view_width: 21,
            view_height: 9,
        };
    }
    fn get_title(&self) -> String {
        return self.habit.name.to_owned();
        // return format!(
        //     "{:^width$.max$}",
        //     self.habit.name,
        //     width = self.view_width as usize,
        //     max = self.view_width as usize - 3
        // );
    }
}

impl View for BitView {
    fn draw(&self, printer: &Printer) {
        let now = Local::now();
        let year = now.year();
        let month = now.month();

        for i in 1..=31 {
            let day = NaiveDate::from_ymd_opt(year, month, i);
            let d = day.unwrap();
            let day_stat = self.habit.get_by_date(d).unwrap_or(&false);
            let coords = ((i % 7) * 3, i / 7 + 2);

            if d < now.naive_utc().date() {
                printer.print(coords, &format!("{:^3}", self.false_chr))
            } else if d > now.naive_utc().date() {
                printer.print(coords, &format!("{:^3}", self.future_chr))
            } else {
                printer.print(coords, &format!("{:^3}", self.future_chr))
            }
            // if let Some(d) = day {
            //     let day_status = self.habit.get_by_date(d).unwrap_or(&false);
            //     let coords = ((i % 7) * 3, i / 7 + 2);

            //     if d <= now.naive_utc().date() {
            //         if *day_status {
            //             printer.print(coords, &format!("{:^3}", self.true_chr))
            //         } else {
            //             printer.print(coords, &format!("{:^3}", self.false_chr))
            //         }
            //     } else {
            //         printer.print(coords, &format!("{:^3}", self.future_chr))
            //     }
            // }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (20, 9).into()
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

fn pallete_gen() -> Palette {
    let mut p = Palette::default();
    p[Background] = Dark(BaseColor::Black);
    p[Shadow] = Light(BaseColor::Black);
    p[View] = Dark(BaseColor::Black);
    p[Primary] = Dark(BaseColor::White);
    p[Secondary] = Light(BaseColor::Black);
    p[Tertiary] = Dark(BaseColor::Green);
    p[TitlePrimary] = Light(BaseColor::White);
    p[Highlight] = Dark(BaseColor::Red);
    p[HighlightInactive] = Dark(BaseColor::Black);

    return p;
}

fn theme_gen() -> Theme {
    let mut t = Theme::default();
    t.shadow = false;
    t.borders = BorderStyle::Simple;
    t.palette = pallete_gen();
    return t;
}

fn main() {
    let mut work_out: Habit<bool> = Habit::new("gymming");
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 4), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 2), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 3), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 1), true);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 5), false);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 8), false);
    work_out.insert_entry(NaiveDate::from_ymd(2020, 2, 11), false);

    let mut again: Habit<bool> = Habit::new("reading");
    again.insert_entry(NaiveDate::from_ymd(2020, 2, 4), true);
    again.insert_entry(NaiveDate::from_ymd(2020, 2, 2), true);

    let mut s = Cursive::default();

    let gym_view = BitView::new(work_out);
    let gym_title = gym_view.get_title();

    let reading_view = BitView::new(again);
    let reading_title = reading_view.get_title();

    s.add_global_callback('q', |a| a.quit());
    s.add_layer(
        LinearLayout::horizontal()
            .child(Dialog::around(gym_view).title(gym_title))
            .child(Dialog::around(reading_view).title(reading_title)),
    );

    s.set_theme(theme_gen());
    s.run();
}
