use crate::tokens::Position;

/// Represents a single frame in the call stack
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Name of the function being called
    pub function_name: String,
    /// Position in source code where the function was called
    pub call_position: Position,
    /// Optional file path where the call occurred
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

    /// Format this stack frame for display
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

/// Manages the call stack for tracking function calls
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

    /// Push a new stack frame
    pub fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }

    /// Pop the top stack frame
    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }

    /// Get the current depth of the call stack
    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    /// Get a reference to all frames
    pub fn frames(&self) -> &[StackFrame] {
        &self.frames
    }

    /// Clear all frames from the stack
    pub fn clear(&mut self) {
        self.frames.clear();
    }

    /// Get the current function name (top of stack)
    pub fn current_function(&self) -> Option<&str> {
        self.frames.last().map(|f| f.function_name.as_str())
    }

    /// Format the call stack for display
    pub fn format_stack_trace(&self) -> String {
        if self.frames.is_empty() {
            return String::new();
        }

        let mut output = String::from("\nCall stack:\n");

        // Display frames in reverse order (most recent first)
        let frames_to_show = self.frames.len().min(self.max_display_depth);
        let start_index = self.frames.len().saturating_sub(frames_to_show);

        for frame in self.frames[start_index..].iter().rev() {
            output.push_str(&frame.format());
            output.push('\n');
        }

        // If there are more frames than we're showing, add a note
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
