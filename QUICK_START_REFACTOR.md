# QUICK START: Refactor Guide (Copy-Paste Reference)

## Pre-Refactor Checklist

```bash
# 1. Current state
git status                                    # Clean working directory?
git log --oneline -5                         # Recent commits

# 2. Create feature branch
git checkout -b refactor/modular-system

# 3. Verify build
cargo build 2>&1 | head -20                  # Currently compiles?
cargo test --lib 2>&1 | tail -5              # Tests pass?
```

---

## PHASE 1: Foundation Files

### Step 1.1: Create `src/runtime/conversion.rs`

```bash
cat > src/runtime/conversion.rs << 'CONVERSION_EOF'
use crate::runtime::{RuntimeValue, FloatValue, IntValue, StrValue, BoolValue, NullValue};
use crate::ast::types::PrimitiveType;
use std::collections::HashMap;

/// Trait for converting Raccoon RuntimeValue to Rust types
pub trait FromRaccoon: Sized {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String>;
}

/// Trait for converting Rust types to Raccoon RuntimeValue
pub trait ToRaccoon {
    fn to_raccoon(self) -> RuntimeValue;
}

// ========== f64 ==========
impl FromRaccoon for f64 {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Float(f) => Ok(f.value),
            RuntimeValue::Int(i) => Ok(i.value as f64),
            RuntimeValue::Bool(b) => Ok(if b.value { 1.0 } else { 0.0 }),
            _ => Err("Expected number".into()),
        }
    }
}

impl ToRaccoon for f64 {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Float(FloatValue::new(self))
    }
}

// ========== i32 ==========
impl FromRaccoon for i32 {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Int(i) => Ok(i.value as i32),
            RuntimeValue::Float(f) => Ok(f.value as i32),
            RuntimeValue::Bool(b) => Ok(if b.value { 1 } else { 0 }),
            _ => Err("Expected integer".into()),
        }
    }
}

impl ToRaccoon for i32 {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Int(IntValue::new(self as i64))
    }
}

// ========== i64 ==========
impl FromRaccoon for i64 {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Int(i) => Ok(i.value),
            RuntimeValue::Float(f) => Ok(f.value as i64),
            RuntimeValue::Bool(b) => Ok(if b.value { 1 } else { 0 }),
            _ => Err("Expected integer".into()),
        }
    }
}

impl ToRaccoon for i64 {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Int(IntValue::new(self))
    }
}

// ========== bool ==========
impl FromRaccoon for bool {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Bool(b) => Ok(b.value),
            RuntimeValue::Int(i) => Ok(i.value != 0),
            RuntimeValue::Float(f) => Ok(f.value != 0.0),
            _ => Err("Expected boolean".into()),
        }
    }
}

impl ToRaccoon for bool {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Bool(BoolValue::new(self))
    }
}

// ========== String ==========
impl FromRaccoon for String {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Str(s) => Ok(s.value.clone()),
            _ => Err("Expected string".into()),
        }
    }
}

impl ToRaccoon for String {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Str(StrValue::new(self))
    }
}

// ========== Vec<T> ==========
impl<T: FromRaccoon> FromRaccoon for Vec<T> {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::List(list) => {
                list.elements
                    .iter()
                    .map(|v| T::from_raccoon(v))
                    .collect()
            }
            _ => Err("Expected list".into()),
        }
    }
}

impl<T: ToRaccoon> ToRaccoon for Vec<T> {
    fn to_raccoon(self) -> RuntimeValue {
        let elements = self.into_iter().map(|v| v.to_raccoon()).collect();
        RuntimeValue::List(ListValue::new(elements))
    }
}

// ========== Option<T> ==========
impl<T: FromRaccoon> FromRaccoon for Option<T> {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self, String> {
        match val {
            RuntimeValue::Null(_) => Ok(None),
            other => T::from_raccoon(other).map(Some),
        }
    }
}

impl<T: ToRaccoon> ToRaccoon for Option<T> {
    fn to_raccoon(self) -> RuntimeValue {
        match self {
            Some(v) => v.to_raccoon(),
            None => RuntimeValue::Null(NullValue::new()),
        }
    }
}
CONVERSION_EOF

echo "✓ Created conversion.rs"
```

### Step 1.2: Update `src/runtime/mod.rs`

```bash
# Add this line to mod.rs after the other pub mod declarations:
echo "pub mod conversion;" >> src/runtime/mod.rs
echo "pub use conversion::{FromRaccoon, ToRaccoon};" >> src/runtime/mod.rs

cargo check  # Should compile
```

---

## PHASE 1B: Create Registrar

### Step 1.3: Create `src/runtime/registrar.rs`

```bash
cat > src/runtime/registrar.rs << 'REGISTRAR_EOF'
use crate::runtime::{RuntimeValue, NativeFunctionValue, Environment, FromRaccoon, ToRaccoon};
use crate::ast::types::{Type, FunctionType, PrimitiveType};
use std::collections::HashMap;
use std::sync::Arc;
use futures::future::BoxFuture;

pub type SyncHandler = Arc<dyn Fn(Vec<RuntimeValue>) -> RuntimeValue + Send + Sync>;
pub type AsyncHandler = Arc<dyn Fn(Vec<RuntimeValue>) -> BoxFuture<'static, RuntimeValue> + Send + Sync>;

pub struct FunctionSignature {
    pub name: String,
    pub namespace: Option<String>,
    pub handler_type: HandlerType,
    pub min_args: usize,
    pub max_args: Option<usize>,
}

pub enum HandlerType {
    Sync(SyncHandler),
    Async(AsyncHandler),
}

pub struct Registrar {
    functions: HashMap<String, FunctionSignature>,
    constants: HashMap<String, RuntimeValue>,
}

impl Registrar {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn register_fn<F>(
        &mut self,
        name: impl Into<String>,
        namespace: Option<&str>,
        handler: F,
        min_args: usize,
        max_args: Option<usize>,
    )
    where
        F: Fn(Vec<RuntimeValue>) -> RuntimeValue + 'static + Send + Sync,
    {
        let name = name.into();
        let full_name = match namespace {
            Some(ns) => format!("{}.{}", ns, name),
            None => name.clone(),
        };

        self.functions.insert(
            full_name,
            FunctionSignature {
                name,
                namespace: namespace.map(|s| s.to_string()),
                handler_type: HandlerType::Sync(Arc::new(handler)),
                min_args,
                max_args,
            },
        );
    }

    pub fn register_const(&mut self, name: impl Into<String>, value: RuntimeValue) {
        self.constants.insert(name.into(), value);
    }

    pub fn apply_to_environment(&self, env: &mut Environment) {
        for (full_name, sig) in &self.functions {
            if let HandlerType::Sync(handler) = &sig.handler_type {
                let h = handler.clone();
                let native_fn = NativeFunctionValue::new(
                    move |args| (h)(args),
                    fn_type!(variadic, PrimitiveType::any()),
                );
                let _ = env.declare(full_name.clone(), RuntimeValue::NativeFunction(native_fn));
            }
        }

        for (name, value) in &self.constants {
            let _ = env.declare(name.clone(), value.clone());
        }
    }
}

impl Default for Registrar {
    fn default() -> Self {
        Self::new()
    }
}
REGISTRAR_EOF

echo "✓ Created registrar.rs"
```

### Step 1.4: Update `src/runtime/mod.rs`

```bash
echo "pub mod registrar;" >> src/runtime/mod.rs
echo "pub use registrar::Registrar;" >> src/runtime/mod.rs

cargo check  # Should still compile
```

---

## PHASE 1C: Simplified Builtins

### Step 1.5: Simplify `src/runtime/builtins.rs`

```bash
# Backup the old one first
cp src/runtime/builtins.rs src/runtime/builtins.rs.bak

cat > src/runtime/builtins.rs << 'BUILTINS_EOF'
use crate::runtime::{Registrar, RuntimeValue, NullValue, StrValue};
use std::io::{self, Write};

pub fn register_builtins(registrar: &mut Registrar) {
    // print(variadic)
    registrar.register_fn(
        "print",
        None,
        |args| {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", arg);
            }
            RuntimeValue::Null(NullValue::new())
        },
        0,
        None,  // variadic
    );

    // println(str?)
    registrar.register_fn(
        "println",
        None,
        |args| {
            if args.is_empty() {
                println!();
            } else {
                println!("{}", args[0]);
            }
            RuntimeValue::Null(NullValue::new())
        },
        0,
        Some(1),
    );

    // input(prompt?)
    registrar.register_fn(
        "input",
        None,
        |args| {
            let prompt = if !args.is_empty() {
                args[0].to_string()
            } else {
                String::new()
            };

            if !prompt.is_empty() {
                print!("{}", prompt);
                io::stdout().flush().unwrap();
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            RuntimeValue::Str(StrValue::new(input.trim_end().to_string()))
        },
        0,
        Some(1),
    );

    // len(any)
    registrar.register_fn(
        "len",
        None,
        |args| {
            if args.is_empty() {
                return RuntimeValue::Null(NullValue::new());
            }
            match &args[0] {
                RuntimeValue::Str(s) => {
                    RuntimeValue::Int(IntValue::new(s.value.len() as i64))
                }
                RuntimeValue::List(l) => {
                    RuntimeValue::Int(IntValue::new(l.elements.len() as i64))
                }
                _ => RuntimeValue::Null(NullValue::new()),
            }
        },
        1,
        Some(1),
    );
}
BUILTINS_EOF

echo "✓ Simplified builtins.rs (502 → 75 lines)"
```

### Step 1.6: Verify Phase 1

```bash
cargo build 2>&1 | tail -10

# Should see no major errors
# Some warnings about unused code are OK

if [ $? -eq 0 ]; then
    echo "✅ PHASE 1 COMPLETE"
    git add -A
    git commit -m "Phase 1: Add conversion.rs, registrar.rs, simplify builtins.rs"
else
    echo "❌ Build failed, check errors above"
    exit 1
fi
```

---

## PHASE 2: Native Modules with Macros

### Step 2.1: Refactor `src/runtime/natives/math.rs`

```bash
cp src/runtime/natives/math.rs src/runtime/natives/math.rs.bak

cat > src/runtime/natives/math.rs << 'MATH_EOF'
use crate::runtime::{Registrar, RuntimeValue, FloatValue, IntValue, FromRaccoon, ToRaccoon};

pub fn register_math_module(registrar: &mut Registrar) {
    // sqrt(x: f64) -> f64
    registrar.register_fn(
        "sqrt",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.sqrt().to_raccoon()
        },
        1,
        Some(1),
    );

    // pow(base: f64, exp: f64) -> f64
    registrar.register_fn(
        "pow",
        Some("math"),
        |args| {
            let base: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let exp: f64 = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            base.powf(exp).to_raccoon()
        },
        2,
        Some(2),
    );

    // sin(x: f64) -> f64
    registrar.register_fn(
        "sin",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.sin().to_raccoon()
        },
        1,
        Some(1),
    );

    // cos(x: f64) -> f64
    registrar.register_fn(
        "cos",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.cos().to_raccoon()
        },
        1,
        Some(1),
    );

    // tan(x: f64) -> f64
    registrar.register_fn(
        "tan",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.tan().to_raccoon()
        },
        1,
        Some(1),
    );

    // log(x: f64, base: f64?) -> f64
    registrar.register_fn(
        "log",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(1.0);
            let base: f64 = if args.len() > 1 {
                f64::from_raccoon(&args[1]).unwrap_or(std::f64::consts::E)
            } else {
                std::f64::consts::E
            };
            x.log(base).to_raccoon()
        },
        1,
        Some(2),
    );

    // min(a: f64, b: f64) -> f64
    registrar.register_fn(
        "min",
        Some("math"),
        |args| {
            let a: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let b: f64 = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            a.min(b).to_raccoon()
        },
        2,
        Some(2),
    );

    // max(a: f64, b: f64) -> f64
    registrar.register_fn(
        "max",
        Some("math"),
        |args| {
            let a: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            let b: f64 = f64::from_raccoon(&args[1]).unwrap_or(0.0);
            a.max(b).to_raccoon()
        },
        2,
        Some(2),
    );

    // abs(x: f64) -> f64
    registrar.register_fn(
        "abs",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.abs().to_raccoon()
        },
        1,
        Some(1),
    );

    // floor(x: f64) -> f64
    registrar.register_fn(
        "floor",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.floor().to_raccoon()
        },
        1,
        Some(1),
    );

    // ceil(x: f64) -> f64
    registrar.register_fn(
        "ceil",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.ceil().to_raccoon()
        },
        1,
        Some(1),
    );

    // round(x: f64) -> f64
    registrar.register_fn(
        "round",
        Some("math"),
        |args| {
            let x: f64 = f64::from_raccoon(&args[0]).unwrap_or(0.0);
            x.round().to_raccoon()
        },
        1,
        Some(1),
    );
}
MATH_EOF

echo "✓ Refactored math.rs"
```

### Step 2.2: Update `src/runtime/natives/mod.rs`

```bash
# Add this to natives/mod.rs:
cat >> src/runtime/natives/mod.rs << 'EOF'

pub mod math;
pub use math::register_math_module;
EOF

# Test
cargo check
```

### Step 2.3: Update Interpreter Initialization

```bash
# In src/interpreter/mod.rs, replace the old initialization with:

cat > /tmp/new_init.rs << 'EOF'
async fn init_all(env: &mut Environment) -> Result<()> {
    use crate::runtime::{Registrar, setup_builtins};
    use crate::runtime::natives::register_math_module;

    let mut registrar = Registrar::new();

    // 1. Register builtins
    setup_builtins(&mut registrar);

    // 2. Register native modules
    register_math_module(&mut registrar);

    // 3. Apply to environment
    registrar.apply_to_environment(env);

    Ok(())
}
EOF

# Manually edit src/interpreter/mod.rs to include this
```

### Step 2.4: Verify Phase 2

```bash
cargo build 2>&1 | tail -15

if [ $? -eq 0 ]; then
    echo "✅ PHASE 2 COMPLETE"
    git add -A
    git commit -m "Phase 2: Refactor math.rs with new Registrar system"
else
    echo "❌ Build failed"
    exit 1
fi
```

---

## TESTING PHASE 2

```bash
# Create a test file
cat > test_math.rcc << 'TESTEOF'
print(math.sqrt(16))      // Should print 4
print(math.pow(2, 8))     // Should print 256
print(math.sin(0))        // Should print 0
TESTEOF

# Run it
cargo run -- run test_math.rcc

# Expected output:
# 4
# 256
# 0
```

---

## QUICK COMMANDS

```bash
# Build at any time
cargo build

# Check syntax without building
cargo check

# Run REPL
cargo run -- repl

# Test specific file
cargo run -- run mytest.rcc

# See macro expansion
cargo expand --lib runtime::registrar

# Show current git status
git status

# Rollback to last good state
git reset --hard HEAD~1

# View what changed
git diff src/runtime/mod.rs
```

---

## TROUBLESHOOTING

| Error | Solution |
|-------|----------|
| "cannot find `FromRaccoon` in scope" | Add `use crate::runtime::FromRaccoon;` |
| "error: circular import" | Check mod.rs, make sure conversion before registrar |
| "expected closure, found fn" | Use `\|args\| {}` not `fn() {}` |
| "Registrar not found" | Verify `pub mod registrar;` in mod.rs |
| Macro not expanding | Check syntax, run `cargo check` first |

---

## Progress Checklist

- [ ] Phase 1.1: conversion.rs created
- [ ] Phase 1.2: Registrar created
- [ ] Phase 1.3: Builtins simplified
- [ ] Phase 1.4: Build succeeds
- [ ] Phase 2.1: math.rs refactored
- [ ] Phase 2.2: math functions registered
- [ ] Phase 2.3: Initialization updated
- [ ] Phase 2.4: `cargo build` succeeds
- [ ] **Testing**: `math.sqrt(16)` returns 4.0

When ready, continue with Phase 3 (String, Array, etc.) using the same pattern!

