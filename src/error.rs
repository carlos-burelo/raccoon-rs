use crate::tokens::{Position, Range};
use crate::runtime::CallStack;
use colored::*;
use std::fmt;
use std::fs;

/// Categorías de error para el lenguaje Raccoon
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    // === Errores de Compilación ===
    /// Error de sintaxis en el código fuente
    SyntaxError,
    /// Error semántico (lógica del programa)
    SemanticError,
    /// Error de tipo (incompatibilidad de tipos)
    TypeError,
    /// Referencia no definida
    ReferenceError,
    /// Error al importar módulos
    ImportError,

    // === Errores de Runtime ===
    /// Error general de ejecución
    RuntimeError,
    /// Acceso a valor null o undefined
    NullReferenceError,
    /// División por cero
    DivisionByZeroError,
    /// Índice fuera de rango
    IndexOutOfRangeError,
    /// Operación inválida
    InvalidOperationError,

    // === Errores de Recursos ===
    /// Archivo no encontrado
    FileNotFoundError,
    /// Permiso denegado
    PermissionDeniedError,
    /// Error de entrada/salida
    IOError,
    /// Error al leer
    ReadError,
    /// Error al escribir
    WriteError,

    // === Errores de Sistema ===
    /// Stack overflow
    StackOverflowError,
    /// Sin memoria disponible
    OutOfMemoryError,
    /// Error interno del sistema
    InternalError,

    // === Errores de Concurrencia ===
    /// Deadlock detectado
    DeadlockError,
    /// Condición de carrera
    RaceConditionError,
    /// Error de sincronización
    SynchronizationError,

    // === Errores Numéricos ===
    /// Overflow numérico
    OverflowError,
    /// Underflow numérico
    UnderflowError,
    /// Pérdida de precisión
    PrecisionLossError,

    // === Errores de Configuración ===
    /// Error de validación
    ValidationError,
    /// Error de configuración
    ConfigurationError,
    /// Error de variable de entorno
    EnvironmentVariableError,

    // === Otros ===
    /// Error de lógica
    LogicError,
    /// Error de flujo de control
    ControlFlowError,
    /// Error de timeout
    TimeoutError,
    /// Error de red
    NetworkError,
}

impl ErrorKind {
    /// Obtiene el nombre del tipo de error
    pub fn name(&self) -> &str {
        match self {
            Self::SyntaxError => "SyntaxError",
            Self::SemanticError => "SemanticError",
            Self::TypeError => "TypeError",
            Self::ReferenceError => "ReferenceError",
            Self::ImportError => "ImportError",
            Self::RuntimeError => "RuntimeError",
            Self::NullReferenceError => "NullReferenceError",
            Self::DivisionByZeroError => "DivisionByZeroError",
            Self::IndexOutOfRangeError => "IndexOutOfRangeError",
            Self::InvalidOperationError => "InvalidOperationError",
            Self::FileNotFoundError => "FileNotFoundError",
            Self::PermissionDeniedError => "PermissionDeniedError",
            Self::IOError => "IOError",
            Self::ReadError => "ReadError",
            Self::WriteError => "WriteError",
            Self::StackOverflowError => "StackOverflowError",
            Self::OutOfMemoryError => "OutOfMemoryError",
            Self::InternalError => "InternalError",
            Self::DeadlockError => "DeadlockError",
            Self::RaceConditionError => "RaceConditionError",
            Self::SynchronizationError => "SynchronizationError",
            Self::OverflowError => "OverflowError",
            Self::UnderflowError => "UnderflowError",
            Self::PrecisionLossError => "PrecisionLossError",
            Self::ValidationError => "ValidationError",
            Self::ConfigurationError => "ConfigurationError",
            Self::EnvironmentVariableError => "EnvironmentVariableError",
            Self::LogicError => "LogicError",
            Self::ControlFlowError => "ControlFlowError",
            Self::TimeoutError => "TimeoutError",
            Self::NetworkError => "NetworkError",
        }
    }

    /// Indica si el error es recuperable
    pub fn is_recoverable(&self) -> bool {
        !matches!(
            self,
            Self::StackOverflowError
                | Self::OutOfMemoryError
                | Self::InternalError
        )
    }

    /// Indica si el error es de compilación
    pub fn is_compile_time(&self) -> bool {
        matches!(
            self,
            Self::SyntaxError
                | Self::SemanticError
                | Self::TypeError
                | Self::ImportError
        )
    }

    /// Indica si el error es de runtime
    pub fn is_runtime(&self) -> bool {
        matches!(
            self,
            Self::RuntimeError
                | Self::NullReferenceError
                | Self::DivisionByZeroError
                | Self::IndexOutOfRangeError
                | Self::InvalidOperationError
                | Self::ReferenceError
        )
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone)]
pub struct RaccoonError {
    pub kind: ErrorKind,
    pub message: String,
    pub position: Position,
    pub range: Option<Range>,
    pub file: Option<String>,
    pub call_stack: Option<CallStack>,
}

impl RaccoonError {
    /// Crea un nuevo error con un tipo específico
    pub fn with_kind(
        kind: ErrorKind,
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            position,
            range: None,
            file: file.map(|f| f.into()),
            call_stack: None,
        }
    }

    pub fn with_call_stack(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
        call_stack: CallStack,
    ) -> Self {
        Self {
            message: message.into(),
            position,
            range: None,
            file: file.map(|f| f.into()),
            call_stack: Some(call_stack),
        }
    }

    /// Crea un nuevo error (por defecto RuntimeError para compatibilidad)
    pub fn new(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::RuntimeError, message, position, file)
    }

    /// Crea un error con rango y tipo específico
    pub fn with_kind_and_range(
        kind: ErrorKind,
        message: impl Into<String>,
        range: Range,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            position: range.start,
            range: Some(range),
            file: file.map(|f| f.into()),
            call_stack: None,
        }
    }

    /// Crea un error con rango (por defecto RuntimeError)
    pub fn with_range(
        message: impl Into<String>,
        range: Range,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind_and_range(ErrorKind::RuntimeError, message, range, file)
    }

    pub fn at_position(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::new(message, position, file)
    }

    // === Métodos de conveniencia para errores de compilación ===

    pub fn syntax_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::SyntaxError, message, position, file)
    }

    pub fn semantic_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::SemanticError, message, position, file)
    }

    pub fn type_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::TypeError, message, position, file)
    }

    pub fn reference_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::ReferenceError, message, position, file)
    }

    pub fn import_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::ImportError, message, position, file)
    }

    // === Métodos de conveniencia para errores de runtime ===

    pub fn runtime_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::RuntimeError, message, position, file)
    }

    pub fn null_reference_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::NullReferenceError, message, position, file)
    }

    pub fn division_by_zero_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::DivisionByZeroError, message, position, file)
    }

    pub fn index_out_of_range_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::IndexOutOfRangeError, message, position, file)
    }

    pub fn invalid_operation_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::InvalidOperationError, message, position, file)
    }

    // === Métodos de conveniencia para errores de recursos ===

    pub fn file_not_found_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::FileNotFoundError, message, position, file)
    }

    pub fn permission_denied_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::PermissionDeniedError, message, position, file)
    }

    pub fn io_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::IOError, message, position, file)
    }

    // === Métodos de conveniencia para errores de sistema ===

    pub fn stack_overflow_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::StackOverflowError, message, position, file)
    }

    pub fn out_of_memory_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::OutOfMemoryError, message, position, file)
    }

    pub fn internal_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::InternalError, message, position, file)
    }

    // === Métodos de conveniencia para errores numéricos ===

    pub fn overflow_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::OverflowError, message, position, file)
    }

    pub fn underflow_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::UnderflowError, message, position, file)
    }

    // === Métodos de conveniencia para errores de configuración ===

    pub fn validation_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::ValidationError, message, position, file)
    }

    pub fn configuration_error(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
    ) -> Self {
        Self::with_kind(ErrorKind::ConfigurationError, message, position, file)
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

        let header = self.kind.name().red().bold();
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

                    // Padding: 6 chars for line number + space + "│ " + (column - 1) since columns start at 1
                    let padding = " ".repeat(7 + error_col.saturating_sub(1));
                    output.push_str(&padding);

                    // If we have a range and it's on the same line, show the full range
                    if let Some(range) = &self.range {
                        if range.start.0 == range.end.0 && range.start.0 == error_line {
                            let start_col = range.start.1.saturating_sub(1);
                            let end_col = range.end.1.saturating_sub(1);
                            let length = end_col.saturating_sub(start_col).max(1);
                            output.push_str(
                                &"^".repeat(length)
                                    .bright_red()
                                    .bold()
                                    .to_string(),
                            );
                        } else {
                            // Multi-line range or no range, show single caret
                            output.push_str(&"^".bright_red().bold().to_string());
                        }
                    } else {
                        // No range, show single caret
                        output.push_str(&"^".bright_red().bold().to_string());
                    }
                    output.push_str("\n");
                } else {
                    output.push_str(&line_content.bright_black().to_string());
                    output.push_str("\n");
                }
            }
        }

        // Add stack trace if available
        if let Some(ref stack) = self.call_stack {
            if stack.depth() > 0 {
                output.push_str(&stack.format_stack_trace());
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
            let header = self.kind.name().red().bold();
            let message = self.message.bright_yellow();
            let line = self.position.0.to_string().bright_cyan();
            let column = self.position.1.to_string().bright_cyan();

            write!(f, "{} {}:{} → {}", header, line, column, message)
        }
    }
}

impl std::error::Error for RaccoonError {}
