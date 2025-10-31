/// Output styling module for Raccoon
/// Provides colored and formatted output for console

/// ANSI color codes
pub struct Colors;

impl Colors {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const ITALIC: &'static str = "\x1b[3m";
    pub const UNDERLINE: &'static str = "\x1b[4m";

    // Foreground colors
    pub const BLACK: &'static str = "\x1b[30m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";

    // Bright colors
    pub const BRIGHT_BLACK: &'static str = "\x1b[90m";
    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
    pub const BRIGHT_WHITE: &'static str = "\x1b[97m";

    // Background colors
    pub const BG_RED: &'static str = "\x1b[41m";
    pub const BG_GREEN: &'static str = "\x1b[42m";
    pub const BG_YELLOW: &'static str = "\x1b[43m";
    pub const BG_BLUE: &'static str = "\x1b[44m";
}

/// Style presets for different message types
pub struct Styles;

impl Styles {
    /// Success message style (green)
    pub fn success(text: &str) -> String {
        format!("{}✓{} {}", Colors::GREEN, Colors::RESET, text)
    }

    /// Error message style (red with bold)
    pub fn error(text: &str) -> String {
        format!("{}✗ Error:{} {}", Colors::RED, Colors::RESET, text)
    }

    /// Warning message style (yellow)
    pub fn warning(text: &str) -> String {
        format!("{}⚠ Warning:{} {}", Colors::YELLOW, Colors::RESET, text)
    }

    /// Info message style (cyan)
    pub fn info(text: &str) -> String {
        format!("{}ℹ Info:{} {}", Colors::CYAN, Colors::RESET, text)
    }

    /// Header style (bright cyan with bold)
    pub fn header(text: &str) -> String {
        format!(
            "{}{}{}{}\n",
            Colors::BRIGHT_CYAN,
            Colors::BOLD,
            text,
            Colors::RESET
        )
    }

    /// Keyword style (bright blue)
    pub fn keyword(text: &str) -> String {
        format!("{}{}{}", Colors::BRIGHT_BLUE, text, Colors::RESET)
    }

    /// String style (bright green)
    pub fn string(text: &str) -> String {
        format!("{}{}{}", Colors::BRIGHT_GREEN, text, Colors::RESET)
    }

    /// Number style (bright yellow)
    pub fn number(text: &str) -> String {
        format!("{}{}{}", Colors::BRIGHT_YELLOW, text, Colors::RESET)
    }

    /// Path style (cyan)
    pub fn path(text: &str) -> String {
        format!("{}{}{}", Colors::CYAN, text, Colors::RESET)
    }

    /// Error code style (red background with white text)
    pub fn error_code(code: &str) -> String {
        format!("{} {} {}", Colors::BG_RED, code, Colors::RESET)
    }

    /// Separator line
    pub fn separator() -> String {
        format!("{}{}{}", Colors::DIM, "─".repeat(80), Colors::RESET)
    }

    /// Section title
    pub fn section_title(text: &str) -> String {
        format!(
            "{}{}▶ {}{}\n",
            Colors::BRIGHT_MAGENTA,
            Colors::BOLD,
            text,
            Colors::RESET
        )
    }
}

/// Format error message with context
pub fn format_error_message(
    error_msg: &str,
    file: Option<&str>,
    line: Option<usize>,
    column: Option<usize>,
) -> String {
    let mut output = format!("{}Error:{} {}\n", Colors::RED, Colors::RESET, error_msg);

    if let Some(file) = file {
        output.push_str(&format!(
            "  {} {}:{}:{}\n",
            Colors::DIM,
            file,
            line.unwrap_or(0),
            column.unwrap_or(0),
        ));
        output.push_str(Colors::RESET);
    }

    output
}

/// Clear screen
pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

/// Print welcome banner
pub fn print_welcome_banner() {
    println!();
    println!(
        "{}{}🦝 Raccoon Language - Interactive Shell{}",
        Colors::BRIGHT_CYAN,
        Colors::BOLD,
        Colors::RESET
    );
    println!(
        "{}v1.0.0 | Type 'help' for assistance | 'exit' to quit{}\n",
        Colors::DIM,
        Colors::RESET
    );
}

/// Print goodbye message
pub fn print_goodbye() {
    println!(
        "\n{}{}👋 Goodbye!{}\n",
        Colors::BRIGHT_CYAN,
        Colors::BOLD,
        Colors::RESET
    );
}

/// Format a value with syntax highlighting based on type
pub fn format_value(value: &str) -> String {
    colorize_json_like(value)
}

/// Colorize JSON-like output based on syntax
fn colorize_json_like(input: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        // Handle strings (double-quoted)
        if ch == '"' {
            let string_content = extract_string(&chars, &mut i);
            result.push_str(&format!(
                "{}{}{}",
                Colors::BRIGHT_GREEN,
                string_content,
                Colors::RESET
            ));
            continue;
        }

        // Handle single-quoted strings
        if ch == '\'' {
            let string_content = extract_single_quoted_string(&chars, &mut i);
            result.push_str(&format!(
                "{}{}{}",
                Colors::BRIGHT_GREEN,
                string_content,
                Colors::RESET
            ));
            continue;
        }

        // Handle numbers
        if ch.is_ascii_digit()
            || (ch == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
        {
            let num = extract_number(&chars, &mut i);
            result.push_str(&format!(
                "{}{}{}",
                Colors::BRIGHT_YELLOW,
                num,
                Colors::RESET
            ));
            continue;
        }

        // Handle identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            let word = extract_word(&chars, &mut i);
            match word.as_str() {
                "true" | "false" | "null" => {
                    result.push_str(&format!(
                        "{}{}{}",
                        Colors::BRIGHT_YELLOW,
                        word,
                        Colors::RESET
                    ));
                }
                _ => {
                    // Regular identifiers (object keys) - use cyan
                    result.push_str(&format!("{}{}{}", Colors::CYAN, word, Colors::RESET));
                }
            }
            continue;
        }

        // Handle structural characters
        match ch {
            '{' | '}' | '[' | ']' => {
                result.push_str(&format!("{}{}{}", Colors::WHITE, ch, Colors::RESET));
            }
            ':' | ',' => {
                result.push_str(&format!("{}{}{}", Colors::WHITE, ch, Colors::RESET));
            }
            ' ' | '\n' | '\t' => {
                result.push(ch);
            }
            _ => {
                result.push(ch);
            }
        }

        i += 1;
    }

    result
}

fn extract_string(chars: &[char], i: &mut usize) -> String {
    let mut result = String::new();
    result.push(chars[*i]); // Opening quote
    *i += 1;

    while *i < chars.len() {
        let ch = chars[*i];
        result.push(ch);

        if ch == '\\' && *i + 1 < chars.len() {
            *i += 1;
            result.push(chars[*i]);
        } else if ch == '"' {
            *i += 1;
            break;
        }

        *i += 1;
    }

    result
}

fn extract_single_quoted_string(chars: &[char], i: &mut usize) -> String {
    let mut result = String::new();
    result.push(chars[*i]); // Opening quote
    *i += 1;

    while *i < chars.len() {
        let ch = chars[*i];
        result.push(ch);

        if ch == '\\' && *i + 1 < chars.len() {
            *i += 1;
            result.push(chars[*i]);
        } else if ch == '\'' {
            *i += 1;
            break;
        }

        *i += 1;
    }

    result
}

fn extract_number(chars: &[char], i: &mut usize) -> String {
    let mut result = String::new();

    // Handle negative sign
    if chars[*i] == '-' {
        result.push('-');
        *i += 1;
    }

    // Extract digits before decimal point
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        result.push(chars[*i]);
        *i += 1;
    }

    // Handle decimal point and fractional part
    if *i < chars.len() && chars[*i] == '.' {
        result.push('.');
        *i += 1;
        while *i < chars.len() && chars[*i].is_ascii_digit() {
            result.push(chars[*i]);
            *i += 1;
        }
    }

    // Handle scientific notation
    if *i < chars.len() && (chars[*i] == 'e' || chars[*i] == 'E') {
        result.push(chars[*i]);
        *i += 1;
        if *i < chars.len() && (chars[*i] == '+' || chars[*i] == '-') {
            result.push(chars[*i]);
            *i += 1;
        }
        while *i < chars.len() && chars[*i].is_ascii_digit() {
            result.push(chars[*i]);
            *i += 1;
        }
    }

    result
}

fn extract_word(chars: &[char], i: &mut usize) -> String {
    let mut result = String::new();

    while *i < chars.len() && (chars[*i].is_alphanumeric() || chars[*i] == '_') {
        result.push(chars[*i]);
        *i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_number() {
        let result = format_value("42");
        assert!(result.contains("\x1b[93m")); // Should contain BRIGHT_YELLOW
        assert!(result.contains("\x1b[0m")); // Should contain RESET
        assert!(result.contains("42"));
    }

    #[test]
    fn test_colorize_string() {
        let result = format_value("\"hello\"");
        assert!(result.contains("\x1b[92m")); // Should contain BRIGHT_GREEN
        assert!(result.contains("\x1b[0m")); // Should contain RESET
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_colorize_boolean() {
        let result = format_value("true");
        assert!(result.contains("\x1b[93m")); // Should contain BRIGHT_YELLOW for boolean
        assert!(result.contains("true"));
    }
}
