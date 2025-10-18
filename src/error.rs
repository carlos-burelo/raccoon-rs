use crate::tokens::Position;
use colored::*;
use std::fmt;
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
}
impl fmt::Display for RaccoonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = "RaccoonError".red().bold();
        let message = self.message.bright_yellow();
        let line = self.position.0.to_string().bright_green();
        let column = self.position.1.to_string().bright_green();

        write!(
            f,
            "{} {} {}:{} → {}",
            header,
            self.file.as_ref().unwrap_or(&"<unknown file>".into()),
            line,
            column,
            message
        )
    }
}
impl std::error::Error for RaccoonError {}
