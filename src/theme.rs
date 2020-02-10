use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::{BaseColor, BorderStyle, Palette, Theme};

pub fn pallete_gen() -> Palette {
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

pub fn theme_gen() -> Theme {
    let mut t = Theme::default();
    t.shadow = false;
    t.borders = BorderStyle::Simple;
    t.palette = pallete_gen();
    return t;
}
