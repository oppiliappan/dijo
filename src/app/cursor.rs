use chrono::{Duration, Local, NaiveDate};
use cursive::direction::Absolute;

#[derive(Debug, Copy, Clone)]
pub struct Cursor(pub NaiveDate);

impl std::default::Default for Cursor {
    fn default() -> Self {
        Cursor::new()
    }
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            0: Local::now().naive_local().date(),
        }
    }
    pub fn small_seek(&mut self, d: Absolute) {
        let today = Local::now().naive_local().date();
        let cursor = self.0;
        match d {
            Absolute::Right => {
                // forward by 1 day
                let next = cursor.succ_opt().unwrap_or(cursor);
                if next <= today {
                    self.0 = next;
                }
            }
            Absolute::Left => {
                // backward by 1 day
                // assumes an infinite past
                self.0 = cursor.pred_opt().unwrap_or(cursor);
            }
            Absolute::Down => {
                // forward by 1 week
                let next = cursor.checked_add_signed(Duration::weeks(1)).unwrap();
                if next <= today {
                    self.0 = next;
                }
            }
            Absolute::Up => {
                // backward by 1 week
                // assumes an infinite past
                let next = cursor.checked_sub_signed(Duration::weeks(1)).unwrap();
                self.0 = next;
            }
            Absolute::None => {}
        }
    }
    fn long_seek(&mut self, offset: Duration) {
        let cursor = self.0;
        let today = Local::now().naive_local().date();
        let next = cursor.checked_add_signed(offset).unwrap_or(cursor);

        if next <= today {
            self.0 = next;
        } else {
            self.0 = today;
        }
    }
    pub fn month_forward(&mut self) {
        self.long_seek(Duration::weeks(4));
    }
    pub fn month_backward(&mut self) {
        self.long_seek(Duration::weeks(-4));
    }
    pub fn reset(&mut self) {
        self.0 = Local::now().naive_local().date();
    }
}
