use cursive::theme::{BaseColor, Color};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

pub struct AppConfig {
    pub true_chr: char,
    pub false_chr: char,
    pub future_chr: char,

    // view dimensions
    pub view_width: usize,
    pub view_height: usize,

    // app dimensions
    pub grid_width: usize,

    pub reached_color: Color,
    pub todo_color: Color,
    pub future_color: Color,
}

pub fn load_configuration_file() -> AppConfig {
    return AppConfig {
        true_chr: '·',
        false_chr: '·',
        future_chr: '·',
        view_width: 25,
        view_height: 8,
        grid_width: 3,
        reached_color: Color::Dark(BaseColor::Cyan),
        todo_color: Color::Dark(BaseColor::Magenta),
        future_color: Color::Light(BaseColor::Black),
    };
}

pub fn data_file() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("rs", "nerdypepper", "dijo") {
        let mut data_file = PathBuf::from(proj_dirs.data_dir());
        data_file.push("habit_record.json");
        return data_file;
    } else {
        panic!("Invalid home directory!")
    };
}
