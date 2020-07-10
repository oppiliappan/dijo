use cursive::theme::{BaseColor, Color};

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
