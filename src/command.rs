use cursive::view::Resizable;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

use crate::app::App;

pub fn open_command_window(s: &mut Cursive) {
    let command_window = Dialog::around(EditView::new().on_submit(call_on_app).fixed_width(40));
    s.add_layer(command_window);
}

fn call_on_app(s: &mut Cursive, input: &str) {
    s.call_on_name("Main", |view: &mut App| {
        view.parse_command(input);
    });
    s.pop_layer();
}

pub enum Command {
    Add(String, String, Option<u32>), // habit name, habit type, optional goal
    MonthPrev,
    MonthNext,
    Delete(String),
    Quit,
    Blank,
}

impl Command {
    pub fn from_string<P: AsRef<str>>(input: P) -> Self {
        let mut strings: Vec<&str> = input.as_ref().trim().split(' ').collect();
        if strings.is_empty() {
            return Command::Blank;
        }

        let first = strings.first().unwrap().to_string();
        let mut args: Vec<String> = strings.iter_mut().skip(1).map(|s| s.to_string()).collect();
        match first.as_ref() {
            "add" | "a" => {
                if args.len() < 2 {
                    return Command::Blank;
                }
                let goal = args.get(2).map(|g| g.parse::<u32>().ok()).flatten();
                return Command::Add(
                    args.get_mut(0).unwrap().to_string(),
                    args.get_mut(1).unwrap().to_string(),
                    goal,
                );
            }
            "delete" | "d" => {
                if args.len() < 1 {
                    return Command::Blank;
                }
                return Command::Delete(args[0].to_string());
            }
            "mprev" | "month-prev" => return Command::MonthPrev,
            "mnext" | "month-next" => return Command::MonthNext,
            "q" | "quit" => return Command::Quit,
            _ => return Command::Blank,
        }
    }
}
