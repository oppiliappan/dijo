use cursive::theme::{BaseColor, Color};
use directories::ProjectDirs;
use serde::Deserialize;
use std;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(default = "default_config")]
pub struct AppConfig {
    pub true_chr: char,
    pub false_chr: char,
    pub future_chr: char,

    // view dimensions
    pub view_width: usize,
    pub view_height: usize,

    // app dimensions
    pub grid_width: usize,
}

impl AppConfig {
    // TODO: implement string parsing from config.json
    pub fn reached_color(&self) -> Color {
        return Color::Dark(BaseColor::Cyan);
    }
    pub fn todo_color(&self) -> Color {
        return Color::Dark(BaseColor::Magenta);
    }
    pub fn future_color(&self) -> Color {
        return Color::Dark(BaseColor::Magenta);
    }
}

pub fn load_configuration_file() -> AppConfig {
    let config_f = config_file();
    if let Ok(ref mut f) = File::open(config_f) {
        let mut j = String::new();
        f.read_to_string(&mut j);
        return serde_json::from_str(&j).unwrap();
    } else {
        return default_config();
    }
}

pub fn default_config() -> AppConfig {
    return AppConfig {
        true_chr: '·',
        false_chr: '·',
        future_chr: '·',
        view_width: 25,
        view_height: 8,
        grid_width: 3,
    };
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("rs", "nerdypepper", "dijo")
        .unwrap_or_else(|| panic!("Invalid home directory!"))
}

pub fn config_file() -> PathBuf {
    let proj_dirs = project_dirs();
    let mut data_file = PathBuf::from(proj_dirs.data_dir());
    fs::create_dir_all(&data_file);
    data_file.push("config.json");
    return data_file;
}

pub fn habit_file() -> PathBuf {
    let proj_dirs = project_dirs();
    let mut data_file = PathBuf::from(proj_dirs.data_dir());
    fs::create_dir_all(&data_file);
    data_file.push("habit_record.json");
    return data_file;
}

pub fn auto_habit_file() -> PathBuf {
    let proj_dirs = project_dirs();
    let mut data_file = PathBuf::from(proj_dirs.data_dir());
    fs::create_dir_all(&data_file);
    data_file.push("habit_record[auto].json");
    return data_file;
}
