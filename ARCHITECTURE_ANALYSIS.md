# Raccoon Language Interpreter - Architecture Analysis

## 1. LANGUAGE TYPE & INTERPRETER DESIGN

**Type**: Dynamically-typed scripting language interpreter written in Rust
**Execution Model**: Tree-walking interpreter with async/await support
**Architecture**: 
- **Lexer**: Tokenizes source code
- **Parser**: Builds AST (Abstract Syntax Tree)
- **Analyzer**: Semantic analysis (not fully integrated yet)
- **Interpreter**: Direct AST execution (tree-walking)
- **Runtime**: Dynamic value system with environment scoping

## 2. CURRENT ERROR HANDLING IMPLEMENTATION

### Error Structure (`src/error.rs`)
```rust
pub struct RaccoonError {
    pub message: String,
    pub position: Position,        // (line, column)
    pub range: Option<Range>,      // (start, end) positions
    pub file: Option<String>,
}
```

### Features:
- **Source-aware errors** with file paths and positions
- **Context rendering**: Shows 2 lines of context with syntax highlighting
- **Range highlighting**: Underlines problematic code regions
- **Colored output**: Uses `colored` crate for terminal formatting
- **Constructor helpers**: `new()`, `with_range()`, `at_position()`
- **Format with context**: `format_with_context()` method

### Limitation:
- **No call stack information**: Errors only show where they occurred, not how execution reached that point
- No function name context
- No nested call information

## 3. FUNCTION CALLS & EXECUTION MANAGEMENT

### Function Value Structure (`src/runtime/values.rs`)
```rust
pub enum RuntimeValue {
    Function(FunctionValue),          // User-defined functions
    NativeFunction(NativeFunctionValue),        // Sync native functions
    NativeAsyncFunction(NativeAsyncFunctionValue), // Async native functions
    // ... other types
}

pub struct FunctionValue {
    pub parameters: Vec<FnParam>,
    pub body: Vec<Stmt>,
    pub is_async: bool,
    pub fn_type: Type,
    pub decorators: Vec<DecoratorDecl>,
}
```

### Function Call Flow (`src/interpreter/expressions.rs`)

**Entry Point**: `evaluate_call_expr()` (line 395)

**Process**:
1. **Recursion Check** (line 399):
   - Checks if `interpreter.recursion_depth >= interpreter.max_recursion_depth`
   - Max depth: 500 calls
   - Returns error: "Maximum recursion depth exceeded"

2. **Evaluation Phase**:
   - Evaluates callee expression
   - Evaluates all arguments and named arguments
   - Handles spread operator unpacking

3. **Execution Phase** (for user-defined functions):
   ```rust
   // Line 527: Increment depth
   interpreter.recursion_depth += 1;
   
   // Execute function body
   for stmt in &func.body {
       match interpreter.execute_stmt_internal(stmt).await? {
           InterpreterResult::Return(v) => {
               interpreter.recursion_depth -= 1;  // Cleanup
               interpreter.environment.pop_scope();
               return Ok(v);
           }
           // ... other cases with cleanup
       }
   }
   
   // Line 561: Decrement depth
   interpreter.recursion_depth -= 1;
   ```

### Helper Function: `call_function()` (`src/interpreter/helpers.rs:176`)
- Alternative call path for internal function calls
- Same parameter binding and execution logic
- Used by method calls and other internal operations

### Key Features:
- **Parameter binding** with destructuring support
- **Default parameter values**
- **Rest parameters** (variadic functions)
- **Named arguments** support
- **Environment scope management** (push/pop scopes)
- **Async function wrapping** in Future values

## 4. EXISTING STACK/CALL FRAME TRACKING

### Current Implementation:
```rust
pub struct Interpreter {
    pub recursion_depth: usize,      // Only simple depth counter
    pub max_recursion_depth: usize,  // Set to 500
    pub environment: Environment,    // Only scopes, no call info
}
```

### What's Being Tracked:
- **Recursion depth counter** only
- **Environment scopes** (for variable resolution)
- **Function names in AST** but not during execution

### What's Missing:
- ❌ Function names during execution
- ❌ Call locations (where function was called from)
- ❌ Parameter values
- ❌ Call stack history
- ❌ File/line information during execution
- ❌ Stack trace rendering

## 5. OVERALL ARCHITECTURE

```
┌─────────────────────────────────────────────────────────────┐
│                      RACCOON INTERPRETER                    │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┼─────────────┐
                │             │             │
                ▼             ▼             ▼
            ┌────────┐   ┌────────┐   ┌─────────┐
            │ Lexer  │──▶│ Parser │──▶│Analyzer │
            └────────┘   └────────┘   └─────────┘
                                           │
                                           ▼
                                    ┌────────────────┐
                                    │  AST (Nodes)   │
                                    │ + Positions    │
                                    └────────────────┘
                                           │
                                           ▼
                        ┌──────────────────────────────────┐
                        │     INTERPRETER (Tree-Walker)    │
                        │  ┌──────────────────────────┐    │
                        │  │ execute_stmt_internal()  │    │
                        │  │ evaluate_expr()          │    │
                        │  │ evaluate_call_expr()     │    │
                        │  └──────────────────────────┘    │
                        │  ┌──────────────────────────┐    │
                        │  │   recursion_depth: 0-500 │    │
                        │  │   environment: scopes    │    │
                        │  └──────────────────────────┘    │
                        └──────────────────────────────────┘
                                           │
                        ┌──────────────────┴──────────────┐
                        │                                  │
                        ▼                                  ▼
            ┌─────────────────────────┐      ┌─────────────────┐
            │   Runtime Values        │      │  RaccoonError   │
            │  - Function values      │      │  + Position     │
            │  - Environment          │      │  + File info    │
            │  - Control flow structs │      │  + Source code  │
            └─────────────────────────┘      └─────────────────┘
```

### Key Components:

1. **Interpreter** (`src/interpreter/mod.rs`):
   - Main execution engine
   - Statement/expression dispatching
   - Recursion depth tracking

2. **Expressions** (`src/interpreter/expressions.rs`):
   - Binary/Unary operations
   - Function calls (main execution)
   - Member/Index access
   - Conditional evaluation

3. **Helpers** (`src/interpreter/helpers.rs`):
   - `call_function()` - core function invocation
   - Pattern destructuring
   - Scope management

4. **Environment** (`src/runtime/environment.rs`):
   - Scope stack
   - Variable declaration/lookup/assignment
   - Simple HashMap-based scoping

5. **Runtime Values** (`src/runtime/values.rs`):
   - Function value wrappers
   - Type information
   - Native function pointers

## 6. WHERE TO IMPLEMENT STACK TRACE FUNCTIONALITY

### Primary Implementation Points:

#### 1. **Create Call Stack Data Structure** ⭐ PRIMARY
**Location**: `src/runtime/` (new file or extend existing)

```rust
// src/runtime/call_stack.rs (NEW)
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub file: Option<String>,
    pub position: Position,      // Where function was called
    pub call_position: Position, // Where in caller it was called
}

pub struct CallStack {
    frames: Vec<StackFrame>,
    max_depth: usize,
}
```

#### 2. **Enhance Interpreter Structure**
**Location**: `src/interpreter/mod.rs` (Interpreter struct)

```rust
pub struct Interpreter {
    // ... existing fields ...
    pub call_stack: CallStack,  // ADD THIS
}
```

#### 3. **Update Function Call Sites**
**Location**: `src/interpreter/expressions.rs::evaluate_call_expr()`

**Changes needed**:
- Before executing function body: push frame to call stack
- After execution: pop frame
- On error: preserve stack for error reporting

#### 4. **Enhance Error Type**
**Location**: `src/error.rs`

```rust
pub struct RaccoonError {
    // ... existing fields ...
    pub call_stack: Option<Vec<StackFrame>>,  // ADD THIS
}
```

#### 5. **Create Stack Trace Formatting**
**Location**: `src/error.rs`

```rust
impl RaccoonError {
    pub fn format_with_stack_trace(&self) -> String {
        // Format error with call stack information
        // Similar to Python/Node.js stack traces
    }
}
```

### Secondary Implementation Points:

#### 6. **Function Information Storage**
**Location**: `src/runtime/values.rs::FunctionValue`

Add optional name field for better tracking:
```rust
pub struct FunctionValue {
    pub name: Option<String>,  // ADD THIS
    // ... existing fields ...
}
```

#### 7. **AST Metadata Preservation**
**Location**: Currently available but underutilized

The AST already has:
- `FnDecl.name` - function name
- `FnDecl.position` - declaration location
- `CallExpr.position` - call location

These should be preserved through to runtime execution.

### Implementation Strategy:

```
Phase 1: Foundation
├─ Create call_stack.rs with StackFrame/CallStack
├─ Add call_stack field to Interpreter
└─ Update mod.rs exports

Phase 2: Integration
├─ Update evaluate_call_expr() to manage stack
├─ Update call_function() in helpers.rs
├─ Store function names in FunctionValue
└─ Track file/position information

Phase 3: Error Integration
├─ Attach call stack to RaccoonError
├─ Implement stack trace formatting
├─ Update error display methods
└─ Add tests

Phase 4: Enhancement
├─ Add better source context in traces
├─ Support native function tracking
├─ Async/await stack frame visualization
└─ Performance optimization
```

## 7. DATA FLOW DURING ERROR

```
User Code Error
        │
        ▼
execute_stmt_internal() catches error
        │
        ▼
Propagates Err(RaccoonError)
        │
        ▼
evaluate_call_expr() catches error
        │
        ├─ Pop function scope
        ├─ Decrement recursion_depth
        │
        ▼ SHOULD: Attach call stack here
        │
Propagates error up the call chain
        │
        ▼
main.rs/REPL displays error
        │
        ▼
format_with_context() renders it
```

## 8. KEY POSITIONS IN CODE

**Recursion check**: `src/interpreter/expressions.rs:399`
**Call start**: `src/interpreter/expressions.rs:527` (depth += 1)
**Function name source**: `FnDecl.name` from AST
**Call position**: `CallExpr.position` from AST
**File info**: Already in `interpreter.file`
**Error creation**: `src/error.rs:15-46`

## 9. ADVANTAGES OF THIS APPROACH

1. ✅ Minimal changes to core interpreter logic
2. ✅ Can leverage existing position data from AST
3. ✅ Function names already available in AST
4. ✅ Recursion depth tracking already in place
5. ✅ Environment/scope management already working
6. ✅ Error handling infrastructure ready for enhancement

## 10. CHALLENGES & CONSIDERATIONS

1. **Async complexity**: Need to track stacks across await points
2. **Native function names**: May not have source positions
3. **Performance**: Stack trace collection on every error
4. **Memory**: Storing frames for deep call stacks
5. **Line number accuracy**: Need proper source position tracking

## Summary

Raccoon is a **tree-walking interpreter** with a **dynamic runtime** that already has excellent error reporting for single-point errors. The foundation for stack traces exists:

- Source positions in AST ✅
- Function names in AST ✅
- File tracking ✅
- Recursion depth management ✅
- Environment scoping ✅

**What's missing**: A call stack data structure to track function invocations during execution and attach this to errors when they occur.

The implementation should focus on:
1. Creating a lightweight `CallStack` structure
2. Pushing frames in `evaluate_call_expr()` 
3. Popping on completion/error
4. Attaching to `RaccoonError`
5. Formatting for display (like Python/Node.js)
