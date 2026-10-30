use ratatui::style::{Color, Style,};

pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub title: Color,
    pub input: Color,
    pub user: Color,
    pub assistant: Color,
    pub error: Color,
}

pub const THEMES: [Theme; 3] = [
    Theme {
        name: "default",
        background: Color::Reset,
        foreground: Color::White,
        border: Color::DarkGray,
        title: Color::Cyan,
        input: Color::White,
        user: Color::Green,
        assistant: Color::Cyan,
        error: Color::Red,
    },
    Theme {
        name: "light",
        background: Color::White,
        foreground: Color::Black,
        border: Color::Gray,
        title: Color::Blue,
        input: Color::Black,
        user: Color::Green,
        assistant: Color::Blue,
        error: Color::Red,
    },
    Theme {
        name: "amber",
        background: Color::Reset,
        foreground: Color::Yellow,
        border: Color::DarkGray,
        title: Color::Yellow,
        input: Color::Yellow,
        user: Color::LightYellow,
        assistant: Color::Yellow,
        error: Color::Red,
    },
];
