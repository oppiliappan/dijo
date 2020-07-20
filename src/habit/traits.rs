use chrono::NaiveDate;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult};
use cursive::{Printer, Vec2};

use typetag;

use crate::habit::{Bit, Count, TrackEvent, ViewMode};
use crate::views::ShadowView;

pub trait Habit {
    type HabitType;

    fn set_name(&mut self, name: impl AsRef<str>);
    fn set_goal(&mut self, goal: Self::HabitType);
    fn name(&self) -> String;
    fn get_by_date(&self, date: NaiveDate) -> Option<&Self::HabitType>;
    fn insert_entry(&mut self, date: NaiveDate, val: Self::HabitType);
    fn reached_goal(&self, date: NaiveDate) -> bool;
    fn remaining(&self, date: NaiveDate) -> u32;
    fn goal(&self) -> u32;
    fn modify(&mut self, date: NaiveDate, event: TrackEvent);

    fn set_view_month_offset(&mut self, offset: u32);
    fn view_month_offset(&self) -> u32;

    fn set_view_mode(&mut self, mode: ViewMode);
    fn view_mode(&self) -> ViewMode;

    fn is_auto(&self) -> bool;
}

#[typetag::serde(tag = "type")]
pub trait HabitWrapper: erased_serde::Serialize {
    fn remaining(&self, date: NaiveDate) -> u32;
    fn goal(&self) -> u32;
    fn modify(&mut self, date: NaiveDate, event: TrackEvent);
    fn draw(&self, printer: &Printer);
    fn on_event(&mut self, event: Event) -> EventResult;
    fn required_size(&mut self, _: Vec2) -> Vec2;
    fn take_focus(&mut self, _: Direction) -> bool;
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);

    fn set_view_month_offset(&mut self, offset: u32);
    fn view_month_offset(&self) -> u32;

    fn set_view_mode(&mut self, mode: ViewMode);
    fn view_mode(&self) -> ViewMode;

    fn is_auto(&self) -> bool;
}

// typetag dosen't support generics yet, we have to resort to dollar store generics,
// aka macros
macro_rules! auto_habit_impl {
    ($struct_name:ident) => {
        #[typetag::serde]
        impl HabitWrapper for $struct_name {
            // ShadowView
            fn draw(&self, printer: &Printer) {
                ShadowView::draw(self, printer)
            }
            fn on_event(&mut self, event: Event) -> EventResult {
                ShadowView::on_event(self, event)
            }
            fn required_size(&mut self, x: Vec2) -> Vec2 {
                ShadowView::required_size(self, x)
            }
            fn take_focus(&mut self, d: Direction) -> bool {
                ShadowView::take_focus(self, d)
            }

            // Habit
            fn remaining(&self, date: NaiveDate) -> u32 {
                Habit::remaining(self, date)
            }
            fn goal(&self) -> u32 {
                Habit::goal(self)
            }
            fn modify(&mut self, date: NaiveDate, event: TrackEvent) {
                Habit::modify(self, date, event);
            }
            fn name(&self) -> String {
                Habit::name(self)
            }
            fn set_name(&mut self, n: String) {
                Habit::set_name(self, &n)
            }
            fn set_view_month_offset(&mut self, offset: u32) {
                Habit::set_view_month_offset(self, offset)
            }
            fn view_month_offset(&self) -> u32 {
                Habit::view_month_offset(self)
            }
            fn set_view_mode(&mut self, mode: ViewMode) {
                Habit::set_view_mode(self, mode)
            }
            fn view_mode(&self) -> ViewMode {
                Habit::view_mode(self)
            }
            fn is_auto(&self) -> bool {
                Habit::is_auto(self)
            }
        }
    };
}

auto_habit_impl!(Count);
auto_habit_impl!(Bit);
