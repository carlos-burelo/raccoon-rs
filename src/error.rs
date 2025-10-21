use crate::tokens::Position;
use colored::*;
use std::fmt;
use std::fs;

#[derive(Debug, Clone)]
pub struct RaccoonError {
    pub message: String,
    pub position: Position,
    pub file: Option<String>,
}

impl RaccoonError {
    pub fn new(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self {
            message: message.into(),
            position,
            file: file.map(|f| f.into()),
        }
    }

    fn get_code_context(&self, context_lines: usize) -> Option<Vec<(usize, String)>> {
        let file_path = self.file.as_ref()?;
        let content = fs::read_to_string(file_path).ok()?;
        let lines: Vec<&str> = content.lines().collect();

        let error_line = self.position.0;
        if error_line == 0 || error_line > lines.len() {
            return None;
        }

        let start = error_line.saturating_sub(context_lines).max(1);
        let end = (error_line + context_lines).min(lines.len());

        let mut result = Vec::new();
        for i in start..=end {
            if i > 0 && i <= lines.len() {
                result.push((i, lines[i - 1].to_string()));
            }
        }

        Some(result)
    }

    pub fn format_with_context(&self) -> String {
        let mut output = String::new();

        let header = "Error".red().bold();
        let file_name = self
            .file
            .as_ref()
            .map(|f| f.as_str())
            .unwrap_or("<unknown file>");

        let line = self.position.0.to_string().bright_cyan();
        let column = self.position.1.to_string().bright_cyan();
        let msg = self.message.bright_yellow();
        output.push_str(&format!(
            "\n{} {} {}:{} -> {}",
            header,
            file_name.bright_white(),
            line,
            column,
            msg
        ));

        if let Some(context) = self.get_code_context(2) {
            output.push_str("\n");
            let error_line = self.position.0;
            let error_col = self.position.1;

            for (line_num, line_content) in context {
                let is_error_line = line_num == error_line;

                let line_num_str = format!("{:4} ", line_num);
                if is_error_line {
                    output.push_str(&format!("{} ", line_num_str.bright_red().bold()));
                    output.push_str(&"│ ".bright_red().bold().to_string());
                } else {
                    output.push_str(&format!("{} ", line_num_str.bright_black()));
                    output.push_str(&"│ ".bright_black().to_string());
                }

                if is_error_line {
                    output.push_str(&line_content.bright_white().to_string());
                    output.push_str("\n");

                    let padding = " ".repeat(6 + error_col);
                    output.push_str(&padding);
                    output.push_str(
                        &"^".repeat(1.max(line_content.len().saturating_sub(error_col).min(10)))
                            .bright_red()
                            .bold()
                            .to_string(),
                    );
                    output.push_str(&" aquí\n".bright_red().to_string());
                } else {
                    output.push_str(&line_content.bright_black().to_string());
                    output.push_str("\n");
                }
            }
        }

        output.push_str("\n");
        output
    }
}

impl fmt::Display for RaccoonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.file.is_some() {
            write!(f, "{}", self.format_with_context())
        } else {
            let header = "RaccoonError".red().bold();
            let message = self.message.bright_yellow();
            let line = self.position.0.to_string().bright_cyan();
            let column = self.position.1.to_string().bright_cyan();

            write!(f, "{} {}:{} → {}", header, line, column, message)
        }
    }
}

impl std::error::Error for RaccoonError {}
