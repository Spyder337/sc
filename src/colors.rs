
use crate::COLORS;
use std::collections::HashMap;

pub(crate) fn colors_init() -> HashMap<&'static str, &'static str> {
    let mut colors = HashMap::new();
    colors.insert("black", "\x1b[30m");
    colors.insert("red", "\x1b[31m");
    colors.insert("green", "\x1b[32m");
    colors.insert("yellow", "\x1b[33m");
    colors.insert("blue", "\x1b[34m");
    colors.insert("magenta", "\x1b[35m");
    colors.insert("cyan", "\x1b[36m");
    colors.insert("white", "\x1b[37m");
    colors.insert("reset", "\x1b[0m");
    colors
}

pub(crate) fn apply_color(color: &str, text: &str) -> String {
    format!("{}{}{}", COLORS[color], text, COLORS["reset"])
}