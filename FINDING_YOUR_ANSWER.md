# Finding Your Answer - Quick Lookup Guide

## Quick Answers to Your 5 Questions

### 1. "What type of language/interpreter is this?"

**Answer in:**
- **STACK_TRACE_QUICK_REFERENCE.md** - First paragraph
- **ARCHITECTURE_ANALYSIS.md** - Section 1
- **ARCHITECTURE_DIAGRAM.txt** - "INPUT FLOW" section

**Key points:**
- Dynamically-typed scripting language
- Tree-walking interpreter (walks AST directly)
- No bytecode/VM compilation
- Supports: async/await, classes, generics, native modules

---

### 2. "How is the error handling currently implemented?"

**Answer in:**
- **ARCHITECTURE_ANALYSIS.md** - Section 2 "CURRENT ERROR HANDLING"
- **STACK_TRACE_QUICK_REFERENCE.md** - "Current Error Handling" section
- **IMPLEMENTATION_GUIDE.md** - "Change 1: Error Structure Enhancement"

**Key files:**
- `src/error.rs` (lines 6-169)
  - RaccoonError struct with position/file tracking
  - format_with_context() shows error + 2 lines of code
  - Display implementation with colored output

**Current features:**
- Shows error location (line, column)
- Shows file path
- Shows source code context with syntax highlighting
- Ranges with underline highlighting

---

### 3. "Where are function calls and execution managed?"

**Answer in:**
- **ARCHITECTURE_ANALYSIS.md** - Section 3 "FUNCTION CALLS & EXECUTION MANAGEMENT"
- **IMPLEMENTATION_GUIDE.md** - "Visual Call Flow During Function Execution"
- **ARCHITECTURE_DIAGRAM.txt** - "EXECUTION FLOW" section

**Key location:**
- `src/interpreter/expressions.rs` (lines 395-595)
  - evaluate_call_expr() - primary function call handler
  - Line 399: Recursion depth check
  - Line 527: recursion_depth increment
  - Line 561: recursion_depth decrement

**Function execution process:**
1. Check recursion depth (line 399)
2. Evaluate callee and arguments
3. Push new scope (line 449)
4. Bind parameters
5. Increment depth (line 527) - PUSH STACK HERE
6. Execute body
7. Decrement depth (line 561) - POP STACK HERE
8. Pop scope
9. Return

---

### 4. "Is there any existing stack/call frame tracking?"

**Answer in:**
- **ARCHITECTURE_ANALYSIS.md** - Section 4 "EXISTING STACK/CALL FRAME TRACKING"
- **STACK_TRACE_QUICK_REFERENCE.md** - "What Already Exists" table
- **ARCHITECTURE_DIAGRAM.txt** - "RUNTIME STATE DURING EXECUTION"

**What EXISTS:**
- recursion_depth counter (0-500) - src/interpreter/mod.rs line 32
- environment scopes - src/runtime/environment.rs
- Position information in every AST node - src/ast/nodes.rs
- Function names in AST - src/ast/nodes.rs line 185 (FnDecl.name)
- File tracking - src/interpreter/mod.rs line 28

**What's MISSING:**
- Call stack data structure
- Function names during execution
- Call location tracking
- Stack trace display

---

### 5. "What is the overall architecture (interpreter, VM, etc.)?"

**Answer in:**
- **ARCHITECTURE_ANALYSIS.md** - Section 5 "OVERALL ARCHITECTURE"
- **ARCHITECTURE_DIAGRAM.txt** - "INPUT FLOW" and "EXECUTION FLOW" sections
- **IMPLEMENTATION_GUIDE.md** - Visual flow diagrams

**Architecture diagram:**

```
Source Code
    ↓
Lexer (tokenize with positions)
    ↓
Parser (build AST with positions)
    ↓
Analyzer (semantic analysis - incomplete)
    ↓
Tree-Walking Interpreter
├─ execute_stmt_internal()
├─ evaluate_expr()
└─ evaluate_call_expr()  ◄─ KEY LOCATION
    ↓
Runtime Values
├─ Environment (scopes)
├─ Recursion depth
└─ Call stack (TO BE ADDED)
    ↓
RaccoonError (with call stack)
    ↓
Display formatted error
```

---

## Where to Find Specific Information

### Architecture Questions

| Question | Document | Section |
|----------|----------|---------|
| What language features? | ARCHITECTURE_ANALYSIS | Section 1 |
| Type system? | DESIGN_TYPE_SYSTEM.md | (external doc) |
| Module system? | STDLIB_ARCHITECTURE.md | (external doc) |
| Async/await? | ARCHITECTURE_DIAGRAM.txt | "RUNTIME STATE" |

### Error Handling Questions

| Question | Document | Section |
|----------|----------|---------|
| Current error structure? | ARCHITECTURE_ANALYSIS | Section 2 |
| Error formatting? | IMPLEMENTATION_GUIDE.md | Change 1 |
| Display logic? | src/error.rs | Lines 152-165 |
| Context rendering? | src/error.rs | Lines 72-149 |

### Function Execution Questions

| Question | Document | Section |
|----------|----------|---------|
| How do calls work? | ARCHITECTURE_ANALYSIS | Section 3 |
| Call flow diagram? | IMPLEMENTATION_GUIDE.md | Visual section |
| Exact code location? | ARCHITECTURE_DIAGRAM.txt | "EXECUTION FLOW" |
| Parameter binding? | IMPLEMENTATION_GUIDE.md | Code example |
| Recursion handling? | ARCHITECTURE_ANALYSIS | Section 4 |

### Stack Trace Implementation Questions

| Question | Document | Section |
|----------|----------|---------|
| What to build? | STACK_TRACE_QUICK_REFERENCE.md | All sections |
| Implementation steps? | IMPLEMENTATION_GUIDE.md | All sections |
| Which files to change? | IMPLEMENTATION_GUIDE.md | Code locations |
| Challenges? | ARCHITECTURE_ANALYSIS | Section 10 |
| Testing strategy? | IMPLEMENTATION_GUIDE.md | Testing section |

---

## File-by-File Reference

### For Understanding

1. **Start here (5 min):**
   - STACK_TRACE_QUICK_REFERENCE.md

2. **Visualize it (5 min):**
   - ARCHITECTURE_DIAGRAM.txt

3. **Deep dive (20 min):**
   - ARCHITECTURE_ANALYSIS.md

### For Implementation

1. **Code locations:**
   - IMPLEMENTATION_GUIDE.md (all sections)

2. **Exact line numbers:**
   - IMPLEMENTATION_GUIDE.md "Key Code Locations"

3. **Before/after examples:**
   - IMPLEMENTATION_GUIDE.md "Change 1-6"

4. **How to test:**
   - IMPLEMENTATION_GUIDE.md "Testing Strategy"

### Navigation Help

- **Need navigation?** → STACK_TRACE_ANALYSIS_INDEX.md
- **Need overview?** → This file (FINDING_YOUR_ANSWER.md)
- **Need quick answer?** → Search this file
- **Need to understand error handling?** → ARCHITECTURE_ANALYSIS.md section 2
- **Need to implement?** → IMPLEMENTATION_GUIDE.md

---

## The 4-Question Summary

### 1. What type is it?
**Tree-walking interpreter** written in Rust
- Directly executes AST
- No bytecode/VM layer
- Dynamically-typed
- Features async/await, classes, generics

### 2. How does error handling work?
**RaccoonError struct** with:
- Position (line, column)
- File path
- Source code context (2 lines)
- Syntax highlighting with colored output
- Range highlighting
- **Missing: Call stack**

### 3. Where are function calls?
**src/interpreter/expressions.rs::evaluate_call_expr()**
- Lines 395-595
- Primary function call handler
- Manages recursion depth, scope, parameter binding
- **Hook point for stack trace: lines 527 & 561**

### 4. Existing call frame tracking?
**Yes, partial:**
- recursion_depth counter
- Environment scopes
- Position information
- **No: Call stack history**

### 5. Overall architecture?
**Lexer → Parser → Analyzer → Interpreter → Runtime**
- Interpreter is tree-walking
- No VM/bytecode compilation
- Recursive execution model
- **Call stack to be added**

---

## Critical Code Sections

### Must Read (to understand stack traces)

1. **RaccoonError** - src/error.rs lines 6-50
2. **evaluate_call_expr()** - src/interpreter/expressions.rs lines 395-410
3. **Function execution** - src/interpreter/expressions.rs lines 527-561
4. **FunctionValue** - src/runtime/values.rs lines 399-430
5. **Environment** - src/runtime/environment.rs lines 6-28

### Must Modify (to implement stack traces)

1. **Create** - src/runtime/call_stack.rs (NEW)
2. **Update** - src/interpreter/mod.rs (add call_stack field)
3. **Update** - src/interpreter/expressions.rs (push/pop frames)
4. **Update** - src/error.rs (add call_stack field + formatting)
5. **Optional** - src/runtime/values.rs (add name field)

---

## Quick Reference Cards

### The 4 Key Structures

```rust
// CURRENT
struct RaccoonError {
    message: String,
    position: Position,
    range: Option<Range>,
    file: Option<String>,
}

// CURRENT
struct Interpreter {
    recursion_depth: usize,
    environment: Environment,
    // ... more fields
}

// TO CREATE
struct StackFrame {
    function_name: String,
    file: Option<String>,
    call_position: Position,
}

// TO CREATE
struct CallStack {
    frames: Vec<StackFrame>,
}
```

### The Function Call Sequence

```
1. evaluate_call_expr() called
2. Check: recursion_depth < 500? (line 399)
3. Evaluate callee + args
4. Push new scope (line 449)
5. Bind parameters
6. recursion_depth += 1 (line 527)        ◄─ PUSH FRAME HERE
7. Execute function body (lines 530-559)
8. recursion_depth -= 1 (line 561)        ◄─ POP FRAME HERE
9. Pop scope
10. Return
```

---

## Files in This Analysis Package

```
FINDING_YOUR_ANSWER.md                   ◄─ This file
STACK_TRACE_ANALYSIS_INDEX.md            ◄─ Navigation
STACK_TRACE_QUICK_REFERENCE.md           ◄─ 5-min overview
ARCHITECTURE_ANALYSIS.md                 ◄─ Deep dive
IMPLEMENTATION_GUIDE.md                  ◄─ Code-level guide
ARCHITECTURE_DIAGRAM.txt                 ◄─ Visual reference
```

Total documentation: 1,300+ lines
Time to understand: 30 minutes
Time to implement: 4-5 hours

---

## Next Steps

1. **Find answer to "What is this?"** → Section 1 above
2. **Find answer to "How does error work?"** → Section 2 above
3. **Find answer to "Where are calls?"** → Section 3 above
4. **Find answer to "What tracking exists?"** → Section 4 above
5. **Find answer to "What's architecture?"** → Section 5 above
6. **Then read STACK_TRACE_QUICK_REFERENCE.md**
7. **Then use IMPLEMENTATION_GUIDE.md to code**

---

**Generated**: 2025-11-07
**For**: Raccoon Language Interpreter Stack Trace Implementation
