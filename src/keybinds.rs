use std::convert::From;

use cursive::event::Event as CursiveEvent;
use serde::ser;
use serde::{self, Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq)]
struct Event(CursiveEvent);

macro_rules! event {
    ($thing:expr) => {
        Event { 0: $thing };
    };
}

impl<T> From<T> for Event
where
    T: AsRef<str>,
{
    fn from(key: T) -> Self {
        let key = key.as_ref();
        if key.len() == 1 {
            // single key
            return event!(CursiveEvent::Char(key.chars().nth(0).unwrap()));
        } else if (key.starts_with("c-") || key.starts_with("C-")) && key.len() == 3 {
            // ctrl-key
            return event!(CursiveEvent::CtrlChar(key.chars().nth(2).unwrap()));
        } else {
            panic!(
                r"Invalid keybind in configuration!
                    (I intend to handle this error gracefully in the near future)"
            );
        }
    }
}

enum Bind {
    Char(char),
    CtrlChar(char),
    AltChar(char),
}

impl Serialize for Bind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Bind::Char(c) => serializer.serialize_newtype_variant("bind", 0, "regular", &c),
            Bind::CtrlChar(c) => serializer.serialize_newtype_variant("bind", 0, "ctrl", &c),
            Bind::AltChar(c) => serializer.serialize_newtype_variant("bind", 0, "alt", &c),
        }
    }
}

impl Deserialize for Bind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        eprintln!("hell = {:#?}", hell);
    }
}

impl From<Bind> for CursiveEvent {
    fn from(key: Bind) -> Self {
        match key {
            Bind::Char(c) => CursiveEvent::Char(c),
            Bind::CtrlChar(c) => CursiveEvent::Char(c),
            Bind::AltChar(c) => CursiveEvent::AltChar(c),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyBinds {
    grid: Movement,
    cursor: Movement,
    week_mode: Bind,
    global_week_mode: Bind,
}

#[derive(Serialize, Deserialize)]
pub struct Movement {
    up: Bind,
    down: Bind,
    left: Bind,
    right: Bind,
}

impl Movement {
    pub fn new(left: char, down: char, up: char, right: char) -> Self {
        return Movement {
            up: Bind::Char(up),
            down: Bind::Char(down),
            left: Bind::Char(left),
            right: Bind::Char(right),
        };
    }
}

impl std::default::Default for KeyBinds {
    fn default() -> Self {
        let grid = Movement::new('h', 'j', 'k', 'l');
        let cursor = Movement::new('H', 'J', 'K', 'L');
        return KeyBinds {
            grid,
            cursor,
            week_mode: Bind::Char('v'),
            global_week_mode: Bind::Char('V'),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_keybind() {
        let bind = "X";
        let expected = CursiveEvent::Char('X');
        assert_eq!(Event::from(bind), event!(expected));
    }

    #[test]
    fn control_keybind() {
        let bind = "C-x";
        let expected = CursiveEvent::CtrlChar('x');
        assert_eq!(Event::from(bind), event!(expected));
    }

    #[test]
    fn lower_case_control_keybind() {
        let bind = "c-x";
        let expected = CursiveEvent::CtrlChar('x');
        assert_eq!(Event::from(bind), event!(expected));
    }

    #[test]
    #[should_panic]
    fn very_long_and_wrong_keybind() {
        let bind = "alksdjfalkjdf";
        Event::from(bind);
    }
}
