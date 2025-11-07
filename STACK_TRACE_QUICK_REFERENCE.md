# Stack Trace Implementation - Quick Reference

## 5-Minute Summary

**What is Raccoon?**
- A dynamically-typed scripting language interpreter written in Rust
- Tree-walking interpreter (executes AST directly)
- Features async/await, classes, generics, native module system

**Current Error Handling:**
- Good: Shows error location with source context and colored highlighting
- Missing: Call stack information showing how code reached the error

**How Function Calls Work:**
1. `evaluate_call_expr()` evaluates the function and arguments
2. Checks recursion depth (max 500)
3. Increments `recursion_depth`
4. Pushes new environment scope
5. Binds parameters
6. Executes function body
7. Pops scope, decrements depth, returns

**The Fix (3 main changes):**

### Change 1: Create Call Stack Structure
**File**: `src/runtime/call_stack.rs` (NEW FILE)
```rust
pub struct StackFrame {
    pub function_name: String,
    pub file: Option<String>,
    pub call_position: Position,
}

pub struct CallStack {
    frames: Vec<StackFrame>,
}
```

### Change 2: Add to Interpreter
**File**: `src/interpreter/mod.rs`
```rust
pub struct Interpreter {
    // ... existing ...
    pub call_stack: CallStack,  // ADD THIS
}
```

### Change 3: Update Function Calls
**File**: `src/interpreter/expressions.rs` (around line 527)
```rust
// Before executing function body:
interpreter.call_stack.push(StackFrame {
    function_name: "myFunc".to_string(),
    file: interpreter.file.clone(),
    call_position: call.position,
});

// After executing (when returning or on error):
interpreter.call_stack.pop();
```

### Change 4: Attach to Errors
**File**: `src/error.rs`
```rust
pub struct RaccoonError {
    // ... existing ...
    pub call_stack: Option<Vec<StackFrame>>,  // ADD THIS
}
```

## What Already Exists

| Feature | Location | Status |
|---------|----------|--------|
| Source positions | AST nodes (every stmt/expr has `.position`) | ✅ Ready |
| File tracking | `interpreter.file` | ✅ Ready |
| Function names | `FnDecl.name` in AST | ✅ Ready |
| Recursion tracking | `interpreter.recursion_depth` | ✅ Ready |
| Error formatting | `RaccoonError::format_with_context()` | ✅ Ready |

## What Needs to Be Added

| Feature | Location | Status |
|---------|----------|--------|
| Call stack structure | `src/runtime/call_stack.rs` | ❌ NEW |
| Stack frame storage | `Interpreter.call_stack` | ❌ NEW |
| Stack management | `evaluate_call_expr()` | ❌ NEW |
| Stack in errors | `RaccoonError.call_stack` | ❌ NEW |
| Stack formatting | `format_with_stack_trace()` | ❌ NEW |
| Function names at runtime | `FunctionValue.name` | ❌ NEW |

## Critical Code Locations

```
src/
├── error.rs                           (Lines 6-165)  - Error handling
├── interpreter/
│   ├── mod.rs                         (Lines 27-37)  - Interpreter struct
│   └── expressions.rs                 (Lines 395-595) - Function calls
├── runtime/
│   ├── mod.rs                         (Lines 1-41)   - Runtime exports
│   ├── environment.rs                 (Lines 6-28)   - Scope management
│   └── values.rs                      (Lines 399-430) - Function values
└── ast/nodes.rs                       (Lines 184-194) - Function AST
```

## Implementation Checklist

- [ ] Create `src/runtime/call_stack.rs`
  - [ ] Define `StackFrame` struct
  - [ ] Define `CallStack` struct
  - [ ] Implement `push()`, `pop()`, `get_frames()`

- [ ] Update `src/interpreter/mod.rs`
  - [ ] Add `call_stack` field to Interpreter
  - [ ] Initialize in `Interpreter::new()`
  - [ ] Export CallStack in mod.rs

- [ ] Update `src/interpreter/expressions.rs`
  - [ ] Import CallStack and StackFrame
  - [ ] Push frame at start of function execution (line ~527)
  - [ ] Pop frame on all exit paths (returns, errors)
  - [ ] Handle error case properly

- [ ] Update `src/error.rs`
  - [ ] Add `call_stack` field to RaccoonError
  - [ ] Create `with_stack()` constructor
  - [ ] Implement `format_with_stack_trace()` method
  - [ ] Update `Display` impl to use new formatter

- [ ] Update error returns in `expressions.rs`
  - [ ] Capture call stack when creating errors
  - [ ] Use `with_stack()` constructor

- [ ] Update `src/runtime/values.rs`
  - [ ] Add optional `name` field to FunctionValue (optional)
  - [ ] Update constructors

- [ ] Add to `src/runtime/mod.rs`
  - [ ] Export CallStack and StackFrame

## Expected Behavior

**Before**:
```
Error main.rs 5:10 -> Variable 'x' is not defined
    5 │ print(x);
        │       ^
```

**After**:
```
Error main.rs 5:10 -> Variable 'x' is not defined
    5 │ print(x);
        │       ^

Stack trace:
  at greet (main.rs:5)
    at sayHello (main.rs:9)
      at main (main.rs:13)
```

## Potential Challenges

1. **Function Names**: `FunctionValue` doesn't store names - need to add this
2. **Async Functions**: Call stack must persist across `await` points
3. **Native Functions**: May not have source position information
4. **Performance**: Every error will now collect/format full stack
5. **Memory**: Deep call stacks consume more memory

## Testing

Create `test_stack_trace.rcc`:
```raccoon
fn level3() {
    throw "Error at level 3";
}

fn level2() {
    level3();
}

fn level1() {
    level2();
}

level1();
```

Should show stack: level1 → level2 → level3 → error

## References

- **Current Error**: `/home/user/raccoon-rs/src/error.rs` (lines 6-169)
- **Function Calls**: `/home/user/raccoon-rs/src/interpreter/expressions.rs` (lines 395-595)
- **Interpreter**: `/home/user/raccoon-rs/src/interpreter/mod.rs` (lines 27-73)
- **Function Values**: `/home/user/raccoon-rs/src/runtime/values.rs` (lines 399-430)

## Files Generated

1. `ARCHITECTURE_ANALYSIS.md` - Complete architecture overview
2. `IMPLEMENTATION_GUIDE.md` - Detailed code changes needed
3. `STACK_TRACE_QUICK_REFERENCE.md` - This file
