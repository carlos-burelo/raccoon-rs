# Stack Trace Implementation - Documentation Index

## Overview

This directory contains a comprehensive analysis of the Raccoon language interpreter architecture and implementation guidance for adding stack trace functionality to error reporting.

## Generated Documents

### 1. **STACK_TRACE_QUICK_REFERENCE.md** (START HERE!)
**Purpose**: Quick 5-minute summary for busy developers
**Contents**:
- What is Raccoon?
- How function calls work
- The 4-step fix (high-level)
- What's already implemented vs what needs to be added
- Implementation checklist
- Critical code locations
- Expected behavior before/after

**Best for**: Getting up to speed quickly, understanding scope

---

### 2. **ARCHITECTURE_ANALYSIS.md** (COMPREHENSIVE REFERENCE)
**Purpose**: Complete architecture documentation
**Contents**:
- Language type and design (tree-walking interpreter)
- Current error handling implementation details
- Function call execution flow
- Existing stack/call frame tracking
- Overall interpreter architecture diagrams
- Where to implement stack trace functionality (detailed)
- Data flow during errors
- Key code positions
- Advantages and challenges

**Best for**: Deep understanding, detailed implementation planning

---

### 3. **IMPLEMENTATION_GUIDE.md** (HANDS-ON GUIDE)
**Purpose**: Detailed code-level implementation instructions
**Contents**:
- Visual call flow during function execution
- Exact code locations for changes
- Before/after code comparisons
- Call stack data structure design
- Error structure enhancement
- Stack trace formatting implementation
- Challenges and solutions
- Testing strategy
- Expected output examples

**Best for**: Actually writing the code, debugging issues

---

### 4. **ARCHITECTURE_DIAGRAM.txt** (VISUAL REFERENCE)
**Purpose**: ASCII diagrams for visual learners
**Contents**:
- Input flow (compilation pipeline)
- Execution flow (tree-walking interpreter)
- Error handling flow (current vs proposed)
- Runtime state diagram
- Data structure changes (before/after)
- Key files and their roles
- Call stack lifecycle

**Best for**: Understanding flow visually, presentations

---

## Quick Start Path

**First time?** Follow this order:

1. Read: `STACK_TRACE_QUICK_REFERENCE.md` (5 min)
2. View: `ARCHITECTURE_DIAGRAM.txt` (5 min)
3. Read: `ARCHITECTURE_ANALYSIS.md` section 6 (10 min)
4. Use: `IMPLEMENTATION_GUIDE.md` as reference while coding

**Total: ~30 minutes to understand everything**

---

## Key Findings Summary

### What is Raccoon?
- Dynamically-typed scripting language
- Tree-walking interpreter (walks AST directly)
- Runs in Rust with async/await support
- Features: classes, generics, native modules, type system

### Current State
**Good:**
- ✅ Excellent error location reporting (file, line, column)
- ✅ Source code context with syntax highlighting
- ✅ Position information in every AST node
- ✅ Function names available in AST
- ✅ Recursion depth tracking

**Missing:**
- ❌ Call stack history
- ❌ Function name context in execution
- ❌ "How did we get here?" information
- ❌ Stack trace display

### Implementation Scope

| Phase | Component | Files | Effort |
|-------|-----------|-------|--------|
| 1 | Call stack structure | `src/runtime/call_stack.rs` (NEW) | Small |
| 2 | Interpreter integration | `src/interpreter/mod.rs` | Small |
| 3 | Function call hooks | `src/interpreter/expressions.rs` | Medium |
| 4 | Error attachment | `src/error.rs` | Medium |
| 5 | Formatting/display | `src/error.rs` | Small |
| 6 | Function name tracking | `src/runtime/values.rs` | Small |

**Total effort**: ~4-6 hours for experienced Rust developer

---

## Critical Code Locations

### Must-Know Files

```
src/error.rs
  ├─ Lines 6-12: RaccoonError struct definition
  ├─ Lines 72-149: format_with_context() method
  └─ Lines 152-165: Display implementation

src/interpreter/mod.rs
  ├─ Lines 27-37: Interpreter struct definition
  ├─ Lines 40-73: Interpreter::new()
  └─ Lines 147-166: interpret() method

src/interpreter/expressions.rs
  ├─ Lines 395-408: evaluate_call_expr() start
  ├─ Lines 527-561: Function body execution with recursion depth
  └─ Lines 589-594: Error handling in calls

src/runtime/environment.rs
  ├─ Lines 8-28: Scope management

src/runtime/values.rs
  ├─ Lines 399-430: FunctionValue struct
```

---

## Architecture Quick Reference

```
LEXER (tokenize)
    ↓
PARSER (build AST)
    ↓
INTERPRETER (execute AST)
    ├─ evaluate_call_expr()  ◄─ PRIMARY CHANGE POINT
    ├─ environment (scopes)
    └─ recursion_depth
    
ON ERROR:
    ├─ Create RaccoonError  ◄─ ATTACH CALL STACK HERE
    └─ Display with format_with_context()  ◄─ UPDATE THIS
```

---

## The Core Changes

### Change 1: New Data Structure
```rust
// src/runtime/call_stack.rs
pub struct StackFrame {
    function_name: String,
    file: Option<String>,
    call_position: Position,
}

pub struct CallStack {
    frames: Vec<StackFrame>,
}
```

### Change 2: Track During Execution
```rust
// src/interpreter/expressions.rs line ~527
interpreter.call_stack.push(frame);  // Before executing
// ... execute function ...
interpreter.call_stack.pop();        // After executing
```

### Change 3: Attach to Errors
```rust
// src/error.rs
pub struct RaccoonError {
    // ... existing ...
    call_stack: Option<Vec<StackFrame>>,  // NEW
}
```

### Change 4: Display
```rust
// src/error.rs
pub fn format_with_stack_trace(&self) -> String {
    // Show error + stack trace
}
```

---

## Testing

Create test file `test_stack_trace.rcc`:
```raccoon
fn level3() {
    throw "Error!";
}

fn level2() {
    level3();
}

fn level1() {
    level2();
}

level1();
```

Expected output:
```
Error test_stack_trace.rcc 2:5 -> Error!
    2 │ throw "Error!";
        │       ^

Stack trace:
  at level3 (test_stack_trace.rcc:2)
    at level2 (test_stack_trace.rcc:6)
      at level1 (test_stack_trace.rcc:10)
```

---

## Challenges & Solutions

| Challenge | Solution |
|-----------|----------|
| Getting function names | Store in FunctionValue or lookup in environment |
| Async functions | Store stack in Future value, preserve across awaits |
| Native functions | Add metadata, or skip if not available |
| Performance | Cache stack traces, don't create on every error |
| Memory | Limit stack depth, clean up old stacks |

---

## Related Documentation

Other docs in this repository:
- `IMPLEMENTATION_SUMMARY.md` - Shorter summary
- `DESIGN_TYPE_SYSTEM.md` - Type system details
- `STDLIB_ARCHITECTURE.md` - Standard library design
- Previous branch documentation (existing docs)

---

## Questions & Answers

**Q: Why tree-walking interpreter?**
A: Simpler to implement than VM, good for prototyping languages

**Q: Why no function names in FunctionValue?**
A: Anonymous functions can be assigned, design choice

**Q: Will this slow down the interpreter?**
A: Only when errors occur, negligible impact on success path

**Q: How deep can call stacks get?**
A: Max recursion depth is 500, stack is bounded

**Q: What about native functions?**
A: Can be added later, currently skip them

---

## Implementation Checklist

- [ ] Read QUICK_REFERENCE.md
- [ ] Review ARCHITECTURE_ANALYSIS.md section 6
- [ ] Study IMPLEMENTATION_GUIDE.md code examples
- [ ] Create src/runtime/call_stack.rs
- [ ] Update src/interpreter/mod.rs Interpreter struct
- [ ] Initialize CallStack in Interpreter::new()
- [ ] Export CallStack in src/runtime/mod.rs
- [ ] Update src/interpreter/expressions.rs evaluate_call_expr()
- [ ] Handle all error paths (push/pop correctly)
- [ ] Update src/error.rs with call_stack field
- [ ] Implement format_with_stack_trace()
- [ ] Update error construction to capture stack
- [ ] Test with test_stack_trace.rcc
- [ ] Handle function names (FunctionValue or environment)
- [ ] Document any challenges found
- [ ] Create commit with changes

---

## File Manifest

All analysis files are in `/home/user/raccoon-rs/`:

```
STACK_TRACE_ANALYSIS_INDEX.md       ◄─ This file (navigation)
STACK_TRACE_QUICK_REFERENCE.md      ◄─ Start here (5 min)
ARCHITECTURE_ANALYSIS.md             ◄─ Deep dive (detailed)
IMPLEMENTATION_GUIDE.md              ◄─ Code-level guidance
ARCHITECTURE_DIAGRAM.txt             ◄─ Visual reference
```

---

## Getting Help

If stuck:
1. Review relevant section in IMPLEMENTATION_GUIDE.md
2. Check exact line numbers mentioned in each document
3. Look at existing error handling patterns
4. Study how recursion_depth is currently managed
5. Compare before/after in IMPLEMENTATION_GUIDE.md

---

## Success Criteria

Stack trace implementation is complete when:
1. Errors show function call chain
2. Each frame shows: function name, file, line number
3. Stack is in correct order (innermost last)
4. All error paths capture the stack
5. No performance regression
6. Works with nested calls
7. Clear, readable output format

---

**Generated**: 2025-11-07
**For**: Raccoon Language Interpreter
**Branch**: claude/implement-stack-trace-011CUtzNik6JuFVXxLWc3eRW

