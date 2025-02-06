
use crate::COLORS;
use std::collections::HashMap;

/// Generates a map of color codes for the terminal.
/// 
/// Colors are stored as ANSI escape codes.
/// 
/// Format: \<PREFIX\>[\<COLOR\>;\<STYLE\>m
/// 
/// \<PREFIX\> is the escape character. In rust this is '\x1b'.
/// \<COLOR\> is the color code. This can be a number from 30 to 37.
/// \<STYLE\> is the style code. This can be 0 for normal, 1 for bright, 2 for dim, 3 for italic, 
/// 4 for underline, 5 for blink, 7 for reverse, 8 for hidden, and 9 for strikethrough.
/// 
/// Currently supported codes:
/// - black
/// - red, red_bright
/// - green, green_bright
/// - yellow, yellow_bright
/// - blue, blue_bright
/// - magenta, magenta_bright
/// - cyan, cyan_bright
/// - white
/// - reset
pub(crate) fn colors_init() -> HashMap<&'static str, &'static str> {
    let mut colors = HashMap::new();
    colors.insert("black", "\x1b[30m");
    colors.insert("red", "\x1b[31m");
    colors.insert("red_bright", "\x1b[31;1m");
    colors.insert("green", "\x1b[32m");
    colors.insert("green_bright", "\x1b[32;1m");
    colors.insert("yellow", "\x1b[33m");
    colors.insert("yellow_bright", "\x1b[33;1m");
    colors.insert("blue", "\x1b[34m");
    colors.insert("blue_bright", "\x1b[34;1m");
    colors.insert("magenta", "\x1b[35m");
    colors.insert("magenta_bright", "\x1b[35;1m");
    colors.insert("cyan", "\x1b[36m");
    colors.insert("cyan_bright", "\x1b[36;1m");
    colors.insert("white", "\x1b[37m");
    colors.insert("reset", "\x1b[0m");
    colors
}

/// Applies an ANSI color code to a string.
/// 
/// Note: Reset is called automatically after the text.
pub(crate) fn apply_color(color: &str, text: &str) -> String {
    format!("{}{}{}", COLORS[color], text, COLORS["reset"])
}