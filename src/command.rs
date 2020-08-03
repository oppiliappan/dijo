use std::fmt;

use cursive::event::{Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::Resizable;
use cursive::views::{EditView, LinearLayout, OnEventView, TextView};
use cursive::Cursive;

use crate::app::App;
use crate::utils::{GRID_WIDTH, VIEW_WIDTH};

static COMMANDS: &'static [&'static str] = &[
    "add",
    "add-auto",
    "delete",
    "track-up",
    "track-down",
    "month-prev",
    "month-next",
    "quit",
    "write",
    "help",
];

fn get_command_completion(prefix: &str) -> Option<String> {
    let first_match = COMMANDS.iter().filter(|&x| x.starts_with(prefix)).next();
    return first_match.map(|&x| x.into());
}

fn get_habit_completion(prefix: &str, habit_names: &[String]) -> Option<String> {
    let first_match = habit_names.iter().filter(|&x| x.starts_with(prefix)).next();
    eprintln!("{:?}| {:?}", prefix, first_match);
    return first_match.map(|x| x.into());
}

pub fn open_command_window(s: &mut Cursive) {
    let habit_list: Vec<String> = s
        .call_on_name("Main", |view: &mut App| {
            return view.list_habits();
        })
        .unwrap();
    let style = ColorStyle::new(Color::Dark(BaseColor::Black), Color::Dark(BaseColor::White));
    let command_window = OnEventView::new(
        EditView::new()
            .filler(" ")
            .on_submit(call_on_app)
            .style(style),
    )
    .on_event_inner(
        Event::Key(Key::Tab),
        move |view: &mut EditView, _: &Event| {
            let contents = view.get_content();
            if !contents.contains(" ") {
                let completion = get_command_completion(&*contents);
                if let Some(c) = completion {
                    let cb = view.set_content(c);
                    return Some(EventResult::Consumed(Some(cb)));
                };
                return None;
            } else {
                let word = contents.split(' ').last().unwrap();
                let completion = get_habit_completion(word, &habit_list);
                eprintln!("{:?} | {:?}", completion, contents);
                if let Some(c) = completion {
                    let cb = view.set_content(format!("{}", contents) + &c[word.len()..]);
                    return Some(EventResult::Consumed(Some(cb)));
                };
                return None;
            }
        },
    )
    .fixed_width(VIEW_WIDTH * GRID_WIDTH);
    s.call_on_name("Frame", |view: &mut LinearLayout| {
        let mut commandline = LinearLayout::horizontal()
            .child(TextView::new(":"))
            .child(command_window);
        commandline.set_focus_index(1);
        view.add_child(commandline);
        view.set_focus_index(1);
    });
}

fn call_on_app(s: &mut Cursive, input: &str) {
    // things to do after recieving the command
    // 1. parse the command
    // 2. clean existing command messages
    // 3. remove the command window
    // 4. handle quit command
    s.call_on_name("Main", |view: &mut App| {
        let cmd = Command::from_string(input);
        view.clear_message();
        view.parse_command(cmd);
    });
    s.call_on_name("Frame", |view: &mut LinearLayout| {
        view.set_focus_index(0);
        view.remove_child(view.get_focus_index());
    });

    // special command that requires access to
    // our main cursive object, has to be parsed again
    // here
    // TODO: fix this somehow
    if let Ok(Command::Quit) = Command::from_string(input) {
        s.quit();
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Add(String, Option<u32>, bool),
    MonthPrev,
    MonthNext,
    Delete(String),
    TrackUp(String),
    TrackDown(String),
    Help(Option<String>),
    Write,
    Quit,
    Blank,
}

#[derive(PartialEq, Debug)]
enum CommandName {
    Add,
    AddAuto,
    MonthPrev,
    MonthNext,
    Delete,
    TrackUp,
    TrackDown,
    Help,
    Write,
    Quit,
    Blank,
}

#[derive(PartialEq, Debug)]
pub enum CommandLineError {
    InvalidCommand(String),     // command name
    InvalidArg(u32),            // position
    NotEnoughArgs(String, u32), // command name, required no. of args
}

impl std::error::Error for CommandLineError {}

impl fmt::Display for CommandLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandLineError::InvalidCommand(s) => write!(f, "Invalid command: `{}`", s),
            CommandLineError::InvalidArg(p) => write!(f, "Invalid argument at position {}", p),
            CommandLineError::NotEnoughArgs(s, n) => {
                write!(f, "Command `{}` requires atleast {} argument(s)!", s, n)
            }
        }
    }
}

type Result<T> = std::result::Result<T, CommandLineError>;

impl Command {
    pub fn from_string<P: AsRef<str>>(input: P) -> Result<Command> {
        let input_str = input.as_ref().trim();
        let parsed = parse_command_name(input_str);
        if let Ok((command_name, rest)) = parsed {
            match command_name {
                CommandName::Add => return parse_add(rest),
                CommandName::AddAuto => return parse_add_auto(rest),
                CommandName::Delete => return parse_delete(rest),
                CommandName::TrackUp => return parse_track_up(rest),
                CommandName::TrackDown => return parse_track_down(rest),
                CommandName::Help => return parse_help(rest),
                CommandName::MonthPrev => return Ok(Command::MonthPrev),
                CommandName::MonthNext => return Ok(Command::MonthNext),
                CommandName::Quit => return Ok(Command::Quit),
                CommandName::Write => return Ok(Command::Write),
                CommandName::Blank => return Ok(Command::Blank),
            }
        } else {
            return Err(parsed.err().unwrap());
        }
    }
}

fn parse_command_name(input: &str) -> Result<(CommandName, &str)> {
    let pieces: Vec<&str> = input.trim().splitn(2, ' ').collect();

    let command = pieces.first().unwrap();
    let rest = pieces.iter().skip(1).next().map(|&x| x).unwrap_or("");

    match command.as_ref() {
        "add" | "a" => Ok((CommandName::Add, rest)),
        "add-auto" | "aa" => Ok((CommandName::AddAuto, rest)),
        "delete" | "d" => Ok((CommandName::Delete, rest)),
        "track-up" | "tup" => Ok((CommandName::TrackUp, rest)),
        "track-down" | "tdown" => Ok((CommandName::TrackDown, rest)),
        "h" | "?" | "help" => Ok((CommandName::Help, rest)),
        "mprev" => Ok((CommandName::MonthPrev, "")),
        "mnext" => Ok((CommandName::MonthNext, "")),
        "quit" | "q" => Ok((CommandName::Quit, "")),
        "write" | "w" => Ok((CommandName::Write, "")),
        "" => Ok((CommandName::Blank, "")),
        c => Err(CommandLineError::InvalidCommand(c.to_string())),
    }
}

fn parse_name(input: &str) -> (String, &str) {
    let chars = input.trim().chars();
    let mut name = "".to_owned();
    let mut pos = 0;
    let mut parenthesis = false;

    for c in chars {
        pos = pos + 1;
        if c == '"' || c == '\"' {
            if parenthesis {
                return (name, &input[pos..]);
            } else {
                parenthesis = true;
                continue;
            }
        }

        if parenthesis {
            name.push(c);
            continue;
        }

        if c == ' ' {
            break;
        }

        name.push(c);
    }

    (name, &input[pos..])
}

fn parse_goal(input: &str) -> Option<(Option<u32>, &str)> {
    let chars = input.trim().chars();
    let mut goal_string = "".to_owned();
    let mut pos = 0;

    if input.is_empty() {
        return Some((None, input));
    }

    for c in chars {
        pos = pos + 1;
        if c == ' ' {
            break;
        }

        goal_string.push(c);
    }

    let parsed = goal_string.parse::<u32>();

    if parsed.is_err() {
        return None;
    }

    if pos + 1 > input.len() {
        return Some((parsed.ok(), ""));
    }

    Some((parsed.ok(), &input[pos..]))
}

fn parse_add(input: &str) -> Result<Command> {
    let (name, rest) = parse_name(input);

    if name.is_empty() {
        return Err(CommandLineError::NotEnoughArgs("add".to_owned(), 2));
    }

    let parsed_goal = parse_goal(rest);

    if parsed_goal.is_none() {
        return Err(CommandLineError::InvalidArg(2));
    }

    Ok(Command::Add(name, parsed_goal.unwrap().0, false))
}

fn parse_add_auto(input: &str) -> Result<Command> {
    let (name, rest) = parse_name(input);

    if name.is_empty() {
        return Err(CommandLineError::NotEnoughArgs("add-auto".to_owned(), 2));
    }

    let parsed_goal = parse_goal(rest);

    if parsed_goal.is_none() {
        return Err(CommandLineError::InvalidArg(2));
    }

    Ok(Command::Add(name, parsed_goal.unwrap().0, true))
}

fn parse_delete(input: &str) -> Result<Command> {
    let (name, _) = parse_name(input);

    if name.is_empty() {
        return Err(CommandLineError::NotEnoughArgs("delete".to_owned(), 1));
    }

    Ok(Command::Delete(name))
}

fn parse_track_up(input: &str) -> Result<Command> {
    let (name, _) = parse_name(input);

    if name.is_empty() {
        return Err(CommandLineError::NotEnoughArgs("track-up".to_owned(), 1));
    }

    Ok(Command::TrackUp(name))
}

fn parse_track_down(input: &str) -> Result<Command> {
    let (name, _) = parse_name(input);

    if name.is_empty() {
        return Err(CommandLineError::NotEnoughArgs("track-down".to_owned(), 1));
    }

    Ok(Command::TrackDown(name))
}

fn parse_help(input: &str) -> Result<Command> {
    let (name, _) = parse_name(input);

    if name.is_empty() {
        return Ok(Command::Help(None));
    }

    Ok(Command::Help(Some(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_command() {
        let inputs = ["add eat 2", "a eat 2"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Add(name, goal, auto) => {
                    assert_eq!(name, "eat");
                    assert_eq!(goal.unwrap(), 2);
                    assert_eq!(auto, false);
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_add_command_without_goal() {
        let inputs = ["add eat", "a eat"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Add(name, goal, auto) => {
                    assert_eq!(name, "eat");
                    assert!(goal.is_none());
                    assert_eq!(auto, false);
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_add_command_with_long_name() {
        let inputs = ["add \"eat healthy\" 5", "a \"eat healthy\" 5"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Add(name, goal, auto) => {
                    assert_eq!(name, "eat healthy");
                    assert_eq!(goal.unwrap(), 5);
                    assert_eq!(auto, false);
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_add_auto_command() {
        let inputs = ["add-auto eat 2", "aa eat 2"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Add(name, goal, auto) => {
                    assert_eq!(name, "eat");
                    assert_eq!(goal.unwrap(), 2);
                    assert_eq!(auto, true);
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_delete_command() {
        let inputs = ["delete eat", "d eat"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Delete(name) => {
                    assert_eq!(name, "eat");
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_delete_long_name_command() {
        let inputs = ["delete \"eat healthy\"", "d \"eat healthy\""];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Delete(name) => {
                    assert_eq!(name, "eat healthy");
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_track_up_command() {
        let inputs = ["track-up eat", "tup eat"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::TrackUp(name) => {
                    assert_eq!(name, "eat");
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_track_down_command() {
        let inputs = ["track-down eat", "tdown eat"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::TrackDown(name) => {
                    assert_eq!(name, "eat");
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_help_command() {
        let inputs = ["help add", "? add", "h add"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Help(name) => {
                    assert_eq!(name.unwrap(), "add");
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_help_global_command() {
        let inputs = ["help", "?", "h"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            match result.unwrap() {
                Command::Help(name) => {
                    assert!(name.is_none());
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn parse_month_prev_command() {
        let result = Command::from_string("mprev");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::MonthPrev);
    }

    #[test]
    fn parse_month_next_command() {
        let result = Command::from_string("mnext");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::MonthNext);
    }

    #[test]
    fn parse_quit_command() {
        let inputs = ["q", "quit"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Command::Quit);
        }
    }

    #[test]
    fn parse_write_command() {
        let inputs = ["w", "write"];

        for input in inputs.iter() {
            let result = Command::from_string(input);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Command::Write);
        }
    }

    #[test]
    fn parse_no_command() {
        let result = Command::from_string("");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Blank);
    }

    #[test]
    fn parse_error_invalid_command() {
        let input = "non-existing-command".to_owned();
        let result = Command::from_string(&input);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            CommandLineError::InvalidCommand(input)
        );
    }

    #[test]
    fn parse_error_not_enough_args() {
        let test_cases = [
            ("add", "add", 2),
            ("add-auto", "add-auto", 2),
            ("delete", "delete", 1),
            ("track-up", "track-up", 1),
            ("track-down", "track-down", 1),
        ]
        .iter()
        .map(|(a, b, c)| (a.to_owned(), b.to_owned(), c));

        for test_case in test_cases {
            let result = Command::from_string(test_case.0);

            assert!(result.is_err());
            assert_eq!(
                result.err().unwrap(),
                CommandLineError::NotEnoughArgs(test_case.1.to_string(), *test_case.2)
            );
        }
    }

    #[test]
    fn parse_error_invalid_arg() {
        let test_cases = [
            ("add habit n".to_owned(), 2),
            ("add-auto habit n".to_owned(), 2),
        ];

        for test_case in test_cases.iter() {
            let result = Command::from_string(&test_case.0);

            assert!(result.is_err());
            assert_eq!(
                result.err().unwrap(),
                CommandLineError::InvalidArg(test_case.1)
            );
        }
    }
}
