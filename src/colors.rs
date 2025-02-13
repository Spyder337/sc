use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static!(
    /// Color codes for the terminal.
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
    pub static ref COLORS: HashMap<&'static str, &'static str> = colors_init();
);

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
fn colors_init() -> HashMap<&'static str, &'static str> {
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
/// If an invalid color is given, the text is returned as is.
///
/// Note: Reset is called automatically after the text.
fn apply_color(color: &str, text: &str) -> String {
    if COLORS.contains_key(color) {
        format!("{}{}{}", COLORS[color], text, COLORS["reset"])
    } else {
        text.to_string()
    }
}

/// Trait for applying colors to strings.
pub trait Colorize {
    /// Applies the black color to a string.
    fn black(&self) -> String;
    /// Applies the red color to a string.
    fn red(&self) -> String;
    /// Applies the bright red color to a string.
    fn red_bright(&self) -> String;
    /// Applies the green color to a string.
    fn green(&self) -> String;
    /// Applies the bright green color to a string.
    fn green_bright(&self) -> String;
    /// Applies the yellow color to a string.
    fn yellow(&self) -> String;
    /// Applies the bright yellow color to a string.
    fn yellow_bright(&self) -> String;
    /// Applies the blue color to a string.
    fn blue(&self) -> String;
    /// Applies the bright blue color to a string.
    fn blue_bright(&self) -> String;
    /// Applies the magenta color to a string.
    fn magenta(&self) -> String;
    /// Applies the bright magenta color to a string.
    fn magenta_bright(&self) -> String;
    /// Applies the cyan color to a string.
    fn cyan(&self) -> String;
    /// Applies the bright cyan color to a string.
    fn cyan_bright(&self) -> String;
    /// Applies the white color to a string.
    fn white(&self) -> String;
}

impl Colorize for String {
    fn black(&self) -> String {
        apply_color("black", self)
    }

    fn red(&self) -> String {
        apply_color("red", self)
    }

    fn red_bright(&self) -> String {
        apply_color("red_bright", self)
    }

    fn green(&self) -> String {
        apply_color("green", self)
    }

    fn green_bright(&self) -> String {
        apply_color("green_bright", self)
    }

    fn yellow(&self) -> String {
        apply_color("yellow", self)
    }

    fn yellow_bright(&self) -> String {
        apply_color("yellow_bright", self)
    }

    fn blue(&self) -> String {
        apply_color("blue", self)
    }

    fn blue_bright(&self) -> String {
        apply_color("blue_bright", self)
    }

    fn magenta(&self) -> String {
        apply_color("magenta", self)
    }

    fn magenta_bright(&self) -> String {
        apply_color("magenta_bright", self)
    }

    fn cyan(&self) -> String {
        apply_color("cyan", self)
    }

    fn cyan_bright(&self) -> String {
        apply_color("cyan_bright", self)
    }

    fn white(&self) -> String {
        apply_color("white", self)
    }
}

impl Colorize for &str {
    fn black(&self) -> String {
        apply_color("black", self)
    }

    fn red(&self) -> String {
        apply_color("red", self)
    }

    fn red_bright(&self) -> String {
        apply_color("red_bright", self)
    }

    fn green(&self) -> String {
        apply_color("green", self)
    }

    fn green_bright(&self) -> String {
        apply_color("green_bright", self)
    }

    fn yellow(&self) -> String {
        apply_color("yellow", self)
    }

    fn yellow_bright(&self) -> String {
        apply_color("yellow_bright", self)
    }

    fn blue(&self) -> String {
        apply_color("blue", self)
    }

    fn blue_bright(&self) -> String {
        apply_color("blue_bright", self)
    }

    fn magenta(&self) -> String {
        apply_color("magenta", self)
    }

    fn magenta_bright(&self) -> String {
        apply_color("magenta_bright", self)
    }

    fn cyan(&self) -> String {
        apply_color("cyan", self)
    }

    fn cyan_bright(&self) -> String {
        apply_color("cyan_bright", self)
    }

    fn white(&self) -> String {
        apply_color("white", self)
    }
}
