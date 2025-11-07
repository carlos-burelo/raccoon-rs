# ARQUITECTURA NUEVA: Módulos, Builtins y FFI (Minimal Boilerplate)

## Visión General

**Objetivo**: Un sistema de registro único, declarativo y escalable con **CERO boilerplate**.

```
┌─────────────────────────────────────────────────┐
│         Environment (Single Source)             │
│                                                 │
│  scope: HashMap<String, RuntimeValue>          │
│  type_registry: TypeRegistry                    │
│  decorator_registry: DecoratorRegistry          │
└────────────────┬────────────────────────────────┘
                 ▲
                 │
     ┌───────────┴───────────────────┬──────────────┐
     │                               │              │
     │                               │              │
┌────┴──────────────┐    ┌──────────┴──────┐   ┌───┴──────┐
│  ModuleRegistry   │    │ FunctionRegistry │   │ Namespaces
│                   │    │                  │   │
│  Load & Cache     │    │ Register Funcs   │   │ math.*
│  Module Exports   │    │ Auto-invoke      │   │ string.*
│                   │    │                  │   │ array.*
└───────────────────┘    └──────────────────┘   └──────────┘
```

---

## 1. CORE ARCHITECTURE: Single Registration System

### 1.1 `src/runtime/registrar.rs` (NEW - 150 líneas)

**Concepto**: Un único lugar donde TODO se registra - builtins, nativos, FFI, módulos.

```rust
pub struct Registrar {
    functions: HashMap<String, FunctionSignature>,
    constants: HashMap<String, RuntimeValue>,
    modules: HashMap<String, Module>,
}

pub struct FunctionSignature {
    name: String,
    namespace: Option<String>,  // "math", "string", "array"
    handler: FunctionHandler,
    arity: (usize, Option<usize>),  // (min_args, max_args) - None = variadic
}

pub enum FunctionHandler {
    Sync(Arc<dyn Fn(Vec<RuntimeValue>) -> RuntimeValue + Send + Sync>),
    Async(Arc<dyn Fn(Vec<RuntimeValue>) -> BoxFuture<'static, RuntimeValue> + Send + Sync>),
}

impl Registrar {
    pub fn register_fn<F>(
        &mut self,
        name: impl Into<String>,
        namespace: Option<&str>,
        handler: F,
        arity: (usize, Option<usize>),
    )
    where
        F: Fn(Vec<RuntimeValue>) -> RuntimeValue + 'static + Send + Sync,
    {
        // Deduce signature from handler call
        // Store with full path: "math.sqrt" or "print"
    }

    pub fn register_async_fn<F>(
        &mut self,
        name: impl Into<String>,
        namespace: Option<&str>,
        handler: F,
        arity: (usize, Option<usize>),
    )
    where
        F: Fn(Vec<RuntimeValue>) -> BoxFuture<'static, RuntimeValue> + 'static + Send + Sync,
    { }

    pub fn register_const(
        &mut self,
        name: impl Into<String>,
        value: RuntimeValue,
    ) { }

    pub fn apply_to_environment(&self, env: &mut Environment) {
        // Un-fold todas las funciones registradas en env
    }
}
```

---

## 2. MACRO SYSTEM: Zero Boilerplate Registration

### 2.1 `src/runtime/register_macros.rs` (NEW - 200 líneas)

**Objetivo**: Macros que convierten funciones Rust normales en registros.

```rust
// MACRO 1: Funciones síncronas
#[register_native]
mod math {
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    pub fn pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }

    pub fn sin(x: f64) -> f64 {
        x.sin()
    }
}
// Genera automáticamente:
// - Conversión RuntimeValue -> f64
// - Conversión f64 -> RuntimeValue
// - Arity info
// - Namespace: "math"
// - Registro en Registrar


// MACRO 2: Funciones asíncronas
#[register_native_async]
mod http {
    pub async fn fetch(url: String) -> String {
        // async impl
    }
}


// MACRO 3: Constantes
#[register_const]
pub const PI: f64 = 3.14159;
pub const MAX_INT: i64 = i64::MAX;
// Genera automáticamente:
// - Conversión a RuntimeValue
// - Registro en Registrar


// MACRO 4: Métodos de tipos
#[register_method]
impl List {
    pub fn push(&mut self, value: RuntimeValue) {
        // auto-register como "push" en List type
    }
}
```

**Expansión de ejemplo**:
```rust
#[register_native]
mod math {
    pub fn sqrt(x: f64) -> f64 { x.sqrt() }
}

// Se expande a:
pub fn register_math_module(registrar: &mut Registrar) {
    registrar.register_fn(
        "sqrt",
        Some("math"),
        |args: Vec<RuntimeValue>| -> RuntimeValue {
            let x: f64 = args[0].try_into().expect("Expected f64");
            RuntimeValue::Float(x.sqrt())
        },
        (1, Some(1)),  // min=1, max=1 args
    );
}
```

---

## 3. MODULE LOADING: Integrated with Registrar

### 3.2 `src/runtime/module_loader.rs` (REFACTORED - 200 líneas)

**Cambio clave**: Unificar `ModuleSystem` + `StdLibLoader` + `Registrar`

```rust
pub struct ModuleLoader {
    cache: Arc<RwLock<HashMap<String, Module>>>,
    registrar: Arc<RwLock<Registrar>>,
    type_registry: Arc<RwLock<TypeRegistry>>,
}

impl ModuleLoader {
    pub async fn load_module(&self, path: &str) -> Result<Module> {
        // 1. Check cache
        if let Some(m) = self.cache.read().unwrap().get(path) {
            return Ok(m.clone());
        }

        // 2. Load from filesystem
        let source = fs::read_to_string(path)?;

        // 3. Parse & analyze
        let mut lexer = Lexer::new(source, Some(path.into()));
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens, Some(path.into()));
        let program = parser.parse()?;

        // 4. Extract exports
        let module = Module::from_ast(&program, path)?;

        // 5. Cache & return
        self.cache.write().unwrap().insert(path.to_string(), module.clone());
        Ok(module)
    }

    pub async fn load_stdlib_module(&self, name: &str) -> Result<Module> {
        // Load from src/stdlib/<name>.rcc
        self.load_module(&format!("stdlib/{}", name))
    }

    pub fn register_native_module<F>(&self, name: &str, f: F)
    where
        F: FnOnce(&mut Registrar),
    {
        let mut registrar = self.registrar.write().unwrap();
        f(&mut registrar);
    }
}
```

---

## 4. INTERPRETER INITIALIZATION (SINGLE CALL)

### 4.1 Updated `src/interpreter/mod.rs` initialization

```rust
impl Interpreter {
    pub async fn new() -> Self {
        let mut env = Environment::new();
        let registrar = Registrar::new();
        let loader = ModuleLoader::new(registrar.clone());

        // SINGLE CALL: Load everything
        Self::init_all(&mut env, &loader).await.unwrap();

        Self {
            environment: env,
            loader,
            // ...
        }
    }

    async fn init_all(env: &mut Environment, loader: &ModuleLoader) -> Result<()> {
        // 1. Register builtins (print, println, len, etc)
        Self::register_builtins(env);

        // 2. Register all native modules
        loader.register_native_module("math", register_math_module);
        loader.register_native_module("string", register_string_module);
        loader.register_native_module("array", register_array_module);
        loader.register_native_module("json", register_json_module);
        loader.register_native_module("time", register_time_module);
        loader.register_native_module("random", register_random_module);
        loader.register_native_module("io", register_io_module);
        loader.register_native_module("http", register_http_module);

        // 3. Load stdlib modules
        loader.load_stdlib_module("future").await?;
        loader.load_stdlib_module("object").await?;
        // ...

        // 4. Apply all registrations to environment
        registrar.apply_to_environment(env);

        Ok(())
    }

    fn register_builtins(env: &mut Environment) {
        let print = NativeFunctionValue::new(
            |args| { /* ... */ },
            fn_type!(variadic, PrimitiveType::void()),
        );
        env.declare("print", RuntimeValue::NativeFunction(print));
        // ... (solo print, println, input, len - 30 líneas total)
    }
}
```

---

## 5. NATIVE MODULES: Declarative Style

### 5.1 `src/runtime/natives/math.rs` (REFACTORED)

**Antes**: 150 líneas de boilerplate

```rust
// OLD
pub fn register(functions: &mut HashMap<String, NativeFunctionValue>) {
    functions.insert(
        "native_sqrt".to_string(),
        NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                let x: f64 = match &args[0] {
                    RuntimeValue::Float(f) => f.value,
                    RuntimeValue::Int(i) => i.value as f64,
                    _ => return RuntimeValue::Null(NullValue::new()),
                };
                RuntimeValue::Float(FloatValue::new(x.sqrt()))
            },
            Type::Function(Box::new(FunctionType {
                params: vec![Type::Primitive(PrimitiveType::float())],
                return_type: Type::Primitive(PrimitiveType::float()),
                is_variadic: false,
            })),
        ),
    );
    // ... Repeat 15 times
}
```

**Después**: 40 líneas sin boilerplate

```rust
// NEW
#[register_native]
pub mod math {
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    pub fn pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }

    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    pub fn log(x: f64, base: Option<f64>) -> f64 {
        match base {
            Some(b) => x.log(b),
            None => x.ln(),
        }
    }

    pub fn min(a: f64, b: f64) -> f64 {
        if a < b { a } else { b }
    }

    pub fn max(a: f64, b: f64) -> f64 {
        if a > b { a } else { b }
    }

    pub fn abs(x: f64) -> f64 {
        x.abs()
    }

    pub fn floor(x: f64) -> f64 {
        x.floor()
    }

    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }

    pub fn round(x: f64) -> f64 {
        x.round()
    }

    pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
        x.max(min).min(max)
    }
}
```

**Ventajas**:
- ✅ Tipos exactos (Rust compiler lo verifica)
- ✅ Auto-conversión RuntimeValue ↔ tipos Rust
- ✅ Namespace automático: `math.*`
- ✅ Arity deducida del signature
- ✅ Documentación natural en doc-comments
- ✅ Sin tipos manuales

---

## 6. MODULE DEPENDENCIES & CIRCULAR IMPORTS

### 6.1 `src/runtime/dependency_resolver.rs` (NEW - 100 líneas)

```rust
pub struct DependencyResolver {
    dependency_graph: HashMap<String, Vec<String>>,
    load_order: Vec<String>,
}

impl DependencyResolver {
    pub fn resolve(module_path: &str, source: &str) -> Result<Vec<String>> {
        // Extract: import { x } from "./..."
        // Build dependency graph
        // Detect cycles
        // Return load order (topological sort)

        // If circular: Error with cycle path
        // "module_a -> module_b -> module_a"
    }
}

// Usage in ModuleLoader:
async fn load_module(&self, path: &str) -> Result<Module> {
    let source = fs::read_to_string(path)?;
    let deps = DependencyResolver::resolve(path, &source)?;  // Error on cycle!

    // Load deps first (topological order)
    for dep_path in deps {
        self.load_module(&dep_path).await?;
    }

    // Then load this module
    // ...
}
```

---

## 7. TYPE CONVERSION: Auto-Generated Traits

### 7.1 `src/runtime/conversion_macros.rs` (NEW - 150 líneas)

**Macro que genera FromRaccoon/ToRaccoon automáticamente**:

```rust
// Eliminates 120 lines of duplication from native.rs + rust_ffi.rs

#[auto_convert]
impl FromRaccoon for f64 {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self> {
        match val {
            RuntimeValue::Float(f) => Ok(f.value),
            RuntimeValue::Int(i) => Ok(i.value as f64),
            RuntimeValue::Bool(b) => Ok(if b.value { 1.0 } else { 0.0 }),
            _ => Err("Expected number".into()),
        }
    }
}

#[auto_convert]
impl ToRaccoon for f64 {
    fn to_raccoon(self) -> RuntimeValue {
        RuntimeValue::Float(FloatValue::new(self))
    }
}

// Generates these for all primitive types:
// - i32, i64, f32, f64, bool, String
// - Optional types: Option<T>
// - Collections: Vec<T>, HashMap<K, V>
```

---

## 8. COMPARISON: Before vs After

| Aspect | Before | After | Reduction |
|--------|--------|-------|-----------|
| **Boilerplate per native function** | 15 lines | 1-2 lines | **87%** |
| **Total native module code** | 1,500 lines | 300 lines | **80%** |
| **Registration sites** | 3 (builtins, plugins, FFI) | 1 (Registrar) | **66%** |
| **Module system files** | 6 (module_system, stdlib_loader, builtin_plugins, plugin_system, rust_ffi, rust_ffi_modules) | 2 (module_loader, dependency_resolver) | **66%** |
| **Initialization complexity** | Multiple calls + ordering issues | Single `init_all()` | **Simpler** |
| **Type safety** | Runtime type coercion | Compile-time verified | **Better** |
| **Circular import detection** | None | Automatic | **Better** |
| **Duplication** | 120 lines (native.rs + rust_ffi.rs) | 0 lines | **Eliminated** |

---

## 9. IMPLEMENTATION PLAN (Phase-by-phase)

### Phase 1: Core Infrastructure (2-3 hours)
1. Create `registrar.rs` - Single registration point
2. Create `conversion_macros.rs` - Auto FromRaccoon/ToRaccoon
3. Create `register_macros.rs` - Native module registration macros
4. Update `mod.rs` exports

### Phase 2: Module Loading (2 hours)
1. Create `module_loader.rs` - Unified loader
2. Create `dependency_resolver.rs` - Circular import detection
3. Update `stdlib_loader.rs` to use module_loader
4. Remove old `module_system.rs` duplications

### Phase 3: Native Modules Refactor (3-4 hours)
1. Refactor `natives/math.rs` with macros
2. Refactor `natives/string.rs` with macros
3. Refactor `natives/array.rs` with macros
4. Refactor `natives/json.rs`, `time.rs`, `random.rs`, `io.rs`, `http.rs`
5. Delete boilerplate registration functions

### Phase 4: Builtins & Initialization (1 hour)
1. Update `builtins.rs` - only core 5 functions (~30 lines)
2. Update `interpreter/mod.rs` - single `init_all()` call
3. Delete old registration infrastructure

### Phase 5: FFI Cleanup (1 hour)
1. Remove duplicate traits from `rust_ffi.rs`
2. Register FFI modules using new system
3. Delete unused `plugin_system.rs`

---

## 10. FILE STRUCTURE AFTER REFACTOR

```
src/runtime/
├── mod.rs                          (updated exports)
├── environment.rs                  (unchanged)
├── values.rs                       (unchanged)
├── type_system/                    (unchanged)
│
├── # NEW CORE SYSTEM
├── registrar.rs                    (150 lines) ✨ NEW
├── conversion_macros.rs            (150 lines) ✨ NEW
├── register_macros.rs              (200 lines) ✨ NEW
├── module_loader.rs                (200 lines) - unified
├── dependency_resolver.rs          (100 lines) ✨ NEW
│
├── # BUILTINS (simplified)
├── builtins.rs                     (30 lines)  - SHRUNK 94%
│
├── # NATIVE MODULES (macro-based)
├── natives/
│   ├── mod.rs
│   ├── math.rs                     (40 lines)  - was 150
│   ├── string.rs                   (50 lines)  - was 200
│   ├── array.rs                    (45 lines)  - was 150
│   ├── json.rs                     (30 lines)  - was 138
│   ├── time.rs                     (35 lines)  - was 150+
│   ├── random.rs                   (30 lines)  - was 100+
│   ├── io.rs                       (80 lines)  - was 226
│   └── http.rs                     (100 lines) - was 330
│
├── # FFI (cleaned)
├── rust_ffi.rs                     (100 lines) - was 257 (removed duplication)
├── rust_ffi_modules.rs             (150 lines) - refactored
│
├── # DEPRECATED (DELETE)
├── [DELETE] builtin_plugins.rs     (was 94 lines)
├── [DELETE] plugin_system.rs       (was 177 lines)
├── [DELETE] native.rs              (traits moved to conversion_macros.rs)
└── [DELETE] stdlib_loader.rs       (merged into module_loader.rs)

Total before: ~3,500 lines
Total after: ~1,200 lines (-65%)
```

---

## 11. EXAMPLE: Complete Flow

**User writes in Raccoon**:
```raccoon
import { sqrt, pow } from "math"
import { upper } from "string"

let x = sqrt(16)        // 4
let y = pow(2, 8)       // 256
let greeting = upper("hello")  // "HELLO"

print(greeting)
```

**What happens internally**:
1. Parser sees `import` statements
2. `ModuleLoader::load_module("math")` is called
3. DependencyResolver checks for circular imports
4. Cache hit: returns cached module
5. Functions are called:
   - `sqrt(16)` → invokes handler from `registrar`
   - Handler: `|args| -> args[0].as_f64().sqrt()`
6. Type conversion automatic: `RuntimeValue::Int(16)` → `16.0` → sqrt → `RuntimeValue::Float(4.0)`
7. No manual boilerplate, clean type safety

---

## 12. BENEFITS SUMMARY

✅ **Minimal Boilerplate**: 87% reduction per function
✅ **Single Source of Truth**: One Registrar for everything
✅ **Type Safe**: Compile-time checked conversions
✅ **Scalable**: Add new modules in 2 minutes
✅ **Maintainable**: Clear separation of concerns
✅ **No Duplication**: Traits generated once
✅ **Circular Import Detection**: Automatic
✅ **Unified Module System**: No duplicate caches
✅ **Better Error Messages**: Structured error handling
✅ **Future-Proof**: Easy to add plugins later

