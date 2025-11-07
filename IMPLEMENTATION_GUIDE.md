# Stack Trace Implementation Guide

## Visual Call Flow During Function Execution

```
main.rs run_file()
    │
    ▼
Interpreter::interpret(&program)
    │ (recursion_depth = 0)
    │
    ├─ execute_stmt_internal(stmt) 
    │    │
    │    └─► Stmt::ExprStmt(expr_stmt)
    │        │
    │        └─► evaluate_expr(&expr_stmt.expression)
    │            │
    │            └─► Expr::Call(call)
    │
    ▼ (recursion_depth check at line 399)
evaluate_call_expr(interpreter, call)
    │
    ├─ Evaluate callee: Identifier("myFunc")
    │  └─ Result: RuntimeValue::Function(func)
    │
    ├─ Evaluate arguments
    │
    ├─ Push new environment scope
    │ (interpreter.environment.push_scope() - line 449)
    │
    ├─ Bind parameters to arguments
    │
    ├─ INCREMENT recursion_depth (line 527)
    │ (recursion_depth = 1) ◄─── PUSH CALL STACK HERE
    │
    ├─ Execute function body
    │  │
    │  ├─ execute_stmt_internal(stmt)
    │  │  │
    │  │  └─ (nested function calls increment depth further)
    │  │     ├─ recursion_depth = 2
    │  │     ├─ recursion_depth = 3
    │  │     └─ recursion_depth = 2 (on return)
    │  │
    │  └─ InterpreterResult::Return(value)
    │
    ├─ DECREMENT recursion_depth (line 534 or 561)
    │ (recursion_depth = 0) ◄─── POP CALL STACK HERE
    │
    ├─ Pop environment scope (line 535 or 562)
    │
    └─ Return Ok(value)

    ON ERROR:
    ├─ Some error occurs (e.g., line 503: "Missing required argument")
    │
    ├─ DECREMENT recursion_depth (line 534/550)
    │ ◄─── MUST POP STACK ON ERROR
    │
    ├─ Pop environment scope
    │
    └─ Propagate Err(RaccoonError) ◄─── ATTACH STACK HERE
       │
       ▼
    execute_stmt_internal() catches
       │
       └─ Returns Err(...) up chain
```

## Key Code Locations for Changes

### 1. Error Structure Enhancement (`src/error.rs` - lines 6-12)

```rust
// CURRENT:
#[derive(Debug, Clone)]
pub struct RaccoonError {
    pub message: String,
    pub position: Position,
    pub range: Option<Range>,
    pub file: Option<String>,
}

// SHOULD ADD:
#[derive(Debug, Clone)]
pub struct RaccoonError {
    pub message: String,
    pub position: Position,
    pub range: Option<Range>,
    pub file: Option<String>,
    pub call_stack: Option<Vec<StackFrame>>,  // ◄─── NEW
}
```

### 2. Call Stack Structure (NEW FILE: `src/runtime/call_stack.rs`)

```rust
use crate::tokens::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct StackFrame {
    /// Name of the function being executed
    pub function_name: String,
    
    /// File where function was called
    pub file: Option<String>,
    
    /// Line and column where function was called from
    pub call_position: Position,
    
    /// Line and column where function is defined
    pub definition_position: Option<Position>,
}

pub struct CallStack {
    frames: Vec<StackFrame>,
    max_depth: usize,
}

impl CallStack {
    pub fn new(max_depth: usize) -> Self {
        Self {
            frames: Vec::new(),
            max_depth,
        }
    }
    
    pub fn push(&mut self, frame: StackFrame) -> Result<(), String> {
        if self.frames.len() >= self.max_depth {
            return Err("Call stack overflow".to_string());
        }
        self.frames.push(frame);
        Ok(())
    }
    
    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
    
    pub fn get_frames(&self) -> &[StackFrame] {
        &self.frames
    }
    
    pub fn clear(&mut self) {
        self.frames.clear();
    }
}
```

### 3. Interpreter Enhancement (`src/interpreter/mod.rs` - lines 27-37)

```rust
// CURRENT Interpreter struct:
pub struct Interpreter {
    pub file: Option<String>,
    pub environment: Environment,
    pub type_registry: TypeRegistry,
    pub stdlib_loader: std::sync::Arc<crate::runtime::StdLibLoader>,
    pub recursion_depth: usize,
    pub max_recursion_depth: usize,
    pub decorator_registry: DecoratorRegistry,
    pub registrar: std::sync::Arc<std::sync::Mutex<Registrar>>,
    pub module_registry: std::sync::Arc<ModuleRegistry>,
}

// ADD:
pub struct Interpreter {
    // ... existing fields ...
    pub call_stack: CallStack,  // ◄─── NEW
}

// In Interpreter::new():
Self {
    // ...
    call_stack: CallStack::new(500),  // ◄─── NEW
}
```

### 4. Function Call Update (`src/interpreter/expressions.rs` - lines 395-595)

```rust
// AROUND LINE 527-561: Current recursion depth management
// CHANGE FROM:
interpreter.recursion_depth += 1;

let mut result = RuntimeValue::Null(NullValue::new());
for stmt in &func.body {
    match interpreter.execute_stmt_internal(stmt).await? {
        InterpreterResult::Return(v) => {
            interpreter.recursion_depth -= 1;
            interpreter.environment.pop_scope();
            return Ok(v);
        }
        // ...
    }
}

interpreter.recursion_depth -= 1;

// CHANGE TO:
interpreter.recursion_depth += 1;

// ◄─── PUSH CALL STACK
let frame = StackFrame {
    function_name: func_name.clone(),  // Need to extract from somewhere
    file: interpreter.file.clone(),
    call_position: call.position,
    definition_position: None,
};
interpreter.call_stack.push(frame)?;

let mut result = RuntimeValue::Null(NullValue::new());
for stmt in &func.body {
    match interpreter.execute_stmt_internal(stmt).await? {
        InterpreterResult::Return(v) => {
            interpreter.recursion_depth -= 1;
            interpreter.call_stack.pop();  // ◄─── POP
            interpreter.environment.pop_scope();
            return Ok(v);
        }
        InterpreterResult::Break | InterpreterResult::Continue => {
            interpreter.recursion_depth -= 1;
            interpreter.call_stack.pop();  // ◄─── POP
            interpreter.environment.pop_scope();
            return Err(...);
        }
    }
}

interpreter.recursion_depth -= 1;
interpreter.call_stack.pop();  // ◄─── POP

// Normal return case...
```

### 5. Error Creation with Stack (`src/error.rs` - constructor update)

```rust
// Add new constructor that captures stack:
impl RaccoonError {
    pub fn with_stack(
        message: impl Into<String>,
        position: Position,
        file: Option<impl Into<String>>,
        call_stack: Option<Vec<StackFrame>>,
    ) -> Self {
        Self {
            message: message.into(),
            position,
            range: None,
            file: file.map(|f| f.into()),
            call_stack,
        }
    }
}

// In evaluate_call_expr(), when returning error:
return Err(RaccoonError::with_stack(
    "Missing required argument for parameter 'x'",
    call.position,
    interpreter.file.clone(),
    Some(interpreter.call_stack.get_frames().to_vec()),  // ◄─── ATTACH
));
```

### 6. Stack Trace Formatting (`src/error.rs` - new method)

```rust
impl RaccoonError {
    pub fn format_with_stack_trace(&self) -> String {
        let mut output = String::new();
        
        // Format the main error first
        output.push_str(&self.format_with_context());
        
        // Add stack trace if available
        if let Some(frames) = &self.call_stack {
            output.push_str("\nStack trace:\n");
            
            for (i, frame) in frames.iter().rev().enumerate() {
                let indent = "  ".repeat(i);
                output.push_str(&format!(
                    "{}at {} ({}:{})\n",
                    indent,
                    frame.function_name,
                    frame.file.as_ref().unwrap_or(&"unknown".to_string()),
                    frame.call_position.0,
                ));
            }
        }
        
        output
    }
}

// Update Display impl to use new formatting:
impl fmt::Display for RaccoonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.file.is_some() {
            write!(f, "{}", self.format_with_stack_trace())  // ◄─── USE NEW METHOD
        } else {
            write!(f, "{}", self.format_with_context())
        }
    }
}
```

## Challenge: Getting Function Names

The current `FunctionValue` doesn't store the function name:

```rust
// src/runtime/values.rs - current
pub struct FunctionValue {
    pub parameters: Vec<FnParam>,
    pub body: Vec<Stmt>,
    pub is_async: bool,
    pub fn_type: Type,
    pub decorators: Vec<DecoratorDecl>,
    // ◄─── MISSING: pub name: String,
}
```

**Solutions**:

1. **Extract from FnDecl during declaration** (easiest):
   - In `declarations::execute_fn_decl()`, capture `decl.name`
   - Store it when creating `FunctionValue`
   - Or store in environment variable name

2. **Look up from environment**:
   - During execution, search environment for variable with this function value
   - May be slow and unreliable

3. **Track in AST**:
   - Store anonymous function location info

## Challenge: Async Functions

For async functions, the call stack needs to be preserved across await:

```rust
// Current behavior:
RuntimeValue::NativeAsyncFunction(fn_val) => {
    let result = (fn_val.implementation)(args).await;  // ◄─── STACK NOT TRACKED
    let return_type = match &fn_val.fn_type {
        crate::ast::types::Type::Function(fn_type) => fn_type.return_type.clone(),
        _ => PrimitiveType::any(),
    };
    Ok(RuntimeValue::Future(FutureValue::new_resolved(result, return_type)))
}
```

Possible solution: Store call stack frame in Future value itself.

## Expected Output Example

```
Error src/test.rcc 5:10 -> Variable 'x' is not declared

    5 │ print(x);
        │         ^

Stack trace:
  at myFunction (src/test.rcc:4)
    at main (src/test.rcc:8)
```

## Testing Strategy

Create test file:
```raccoon
fn a() {
    fn b() {
        throw "error in b";
    }
    b();
}

a();
```

Expected output shows:
- Error message and location
- Call stack: a() → b() → error location
