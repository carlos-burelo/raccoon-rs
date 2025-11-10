use crate::tokens::Position;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub call_position: Position,
    pub file: Option<String>,
}

impl StackFrame {
    pub fn new(function_name: String, call_position: Position, file: Option<String>) -> Self {
        Self {
            function_name,
            call_position,
            file,
        }
    }

    pub fn format(&self) -> String {
        let file_info = self
            .file
            .as_ref()
            .map(|f| format!("{}:{}:{}", f, self.call_position.0, self.call_position.1))
            .unwrap_or_else(|| {
                format!(
                    "<unknown>:{}:{}",
                    self.call_position.0, self.call_position.1
                )
            });

        format!("  at {} ({})", self.function_name, file_info)
    }
}

#[derive(Debug, Clone)]
pub struct CallStack {
    frames: Vec<StackFrame>,
    max_display_depth: usize,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            max_display_depth: 10,
        }
    }

    pub fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }

    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    pub fn frames(&self) -> &[StackFrame] {
        &self.frames
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }

    pub fn current_function(&self) -> Option<&str> {
        self.frames.last().map(|f| f.function_name.as_str())
    }

    pub fn format_stack_trace(&self) -> String {
        if self.frames.is_empty() {
            return String::new();
        }

        let mut output = String::from("\nCall stack:\n");

        let frames_to_show = self.frames.len().min(self.max_display_depth);
        let start_index = self.frames.len().saturating_sub(frames_to_show);

        for frame in self.frames[start_index..].iter().rev() {
            output.push_str(&frame.format());
            output.push('\n');
        }

        if self.frames.len() > self.max_display_depth {
            let hidden_count = self.frames.len() - self.max_display_depth;
            output.push_str(&format!("  ... ({} more frames)\n", hidden_count));
        }

        output
    }
}

impl Default for CallStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_stack_basic() {
        let mut stack = CallStack::new();
        assert_eq!(stack.depth(), 0);

        stack.push(StackFrame::new(
            "foo".to_string(),
            (1, 1),
            Some("test.rn".to_string()),
        ));
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current_function(), Some("foo"));

        stack.push(StackFrame::new(
            "bar".to_string(),
            (2, 2),
            Some("test.rn".to_string()),
        ));
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current_function(), Some("bar"));

        stack.pop();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current_function(), Some("foo"));
    }

    #[test]
    fn test_stack_frame_format() {
        let frame = StackFrame::new(
            "myFunction".to_string(),
            (10, 5),
            Some("main.rn".to_string()),
        );
        let formatted = frame.format();
        assert!(formatted.contains("myFunction"));
        assert!(formatted.contains("main.rn"));
        assert!(formatted.contains("10:5"));
    }
}
