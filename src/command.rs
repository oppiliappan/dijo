use std::fmt;
use std::str::FromStr;

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
    "writeandquit",
];

fn get_command_completion(prefix: &str) -> Option<String> {
    let first_match = COMMANDS.iter().filter(|&x| x.starts_with(prefix)).next();
    return first_match.map(|&x| x.into());
}

fn get_habit_completion(prefix: &str, habit_names: &[String]) -> Option<String> {
    let first_match = habit_names.iter().filter(|&x| x.starts_with(prefix)).next();
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
    match Command::from_string(input) {
        Ok(Command::Quit) | Ok(Command::WriteAndQuit) => s.quit(),
        _ => {}
    }
}

#[derive(Debug, PartialEq)]
pub enum GoalKind {
    Count(u32),
    Bit,
    Float(u32, u8),
    Addiction(u32),
}

impl FromStr for GoalKind {
    type Err = CommandLineError;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(n) = s.strip_prefix("<") {
            return n
                .parse::<u32>()
                .map_err(|_| CommandLineError::InvalidGoal(s.into()))
                .map(GoalKind::Addiction);
        } else if s.contains(".") {
            let value = s
                .chars()
                .filter(|x| x.is_digit(10))
                .collect::<String>()
                .parse::<u32>()
                .map_err(|_| CommandLineError::InvalidCommand(s.into()))?;
            let precision = s.chars().skip_while(|&x| x != '.').count() - 1;
            return Ok(GoalKind::Float(value, precision as u8));
        }
        if let Ok(v) = s.parse::<u32>() {
            if v == 1 {
                return Ok(GoalKind::Bit);
            } else {
                return Ok(GoalKind::Count(v));
            }
        }
        return Err(CommandLineError::InvalidCommand(s.into()));
    }
}

#[derive(PartialEq)]
pub enum Command {
    Add(String, Option<GoalKind>, bool),
    MonthPrev,
    MonthNext,
    Delete(String),
    TrackUp(String),
    TrackDown(String),
    Help(Option<String>),
    Write,
    Quit,
    Blank,
    WriteAndQuit,
}

#[derive(Debug)]
pub enum CommandLineError {
    InvalidCommand(String),     // command name
    InvalidArg(u32),            // position
    NotEnoughArgs(String, u32), // command name, required no. of args
    InvalidGoal(String),        // goal expression
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
            CommandLineError::InvalidGoal(s) => write!(f, "Invalid goal expression: `{}`", s),
        }
    }
}

type Result<T> = std::result::Result<T, CommandLineError>;

impl Command {
    pub fn from_string<P: AsRef<str>>(input: P) -> Result<Command> {
        let mut strings: Vec<&str> = input.as_ref().trim().split(' ').collect();
        if strings.is_empty() {
            return Ok(Command::Blank);
        }

        let first = strings.first().unwrap().to_string();
        let mut args: Vec<String> = strings.iter_mut().skip(1).map(|s| s.to_string()).collect();
        let mut _add = |auto: bool, first: String| {
            if args.is_empty() {
                return Err(CommandLineError::NotEnoughArgs(first, 1));
            }
            let goal = args.get(1).map(|x| GoalKind::from_str(x)).transpose()?;
            return Ok(Command::Add(
                args.get_mut(0).unwrap().to_string(),
                goal,
                auto,
            ));
        };

        match first.as_ref() {
            "add" | "a" => _add(false, first),
            "add-auto" | "aa" => _add(true, first),
            "delete" | "d" => {
                if args.is_empty() {
                    return Err(CommandLineError::NotEnoughArgs(first, 1));
                }
                return Ok(Command::Delete(args[0].to_string()));
            }
            "track-up" | "tup" => {
                if args.is_empty() {
                    return Err(CommandLineError::NotEnoughArgs(first, 1));
                }
                return Ok(Command::TrackUp(args[0].to_string()));
            }
            "track-down" | "tdown" => {
                if args.is_empty() {
                    return Err(CommandLineError::NotEnoughArgs(first, 1));
                }
                return Ok(Command::TrackDown(args[0].to_string()));
            }
            "h" | "?" | "help" => {
                if args.is_empty() {
                    return Ok(Command::Help(None));
                }
                return Ok(Command::Help(Some(args[0].to_string())));
            }
            "mprev" | "month-prev" => return Ok(Command::MonthPrev),
            "mnext" | "month-next" => return Ok(Command::MonthNext),
            "wq" | "writeandquit" => return Ok(Command::WriteAndQuit),
            "q" | "quit" => return Ok(Command::Quit),
            "w" | "write" => return Ok(Command::Write),
            "" => return Ok(Command::Blank),
            s => return Err(CommandLineError::InvalidCommand(s.into())),
        }
    }
}
