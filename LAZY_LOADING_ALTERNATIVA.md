# ALTERNATIVA: Lazy Loading (Load-on-Demand)

## El Problema con Inicialización Eagre

```rust
// ACTUAL (malo)
impl Interpreter::new() {
    Self::init_all()  // Carga TODO inmediatamente
        ├─ register_math_module()      ⏱️ 5ms
        ├─ register_string_module()    ⏱️ 3ms
        ├─ register_array_module()     ⏱️ 4ms
        ├─ register_json_module()      ⏱️ 2ms
        ├─ register_time_module()      ⏱️ 3ms
        ├─ register_random_module()    ⏱️ 2ms
        ├─ register_io_module()        ⏱️ 4ms
        └─ register_http_module()      ⏱️ 5ms
                                        ────────
                                        Total: 28ms

Problema: El usuario paga 28ms de startup aunque solo use math.sqrt()
```

---

## La Solución: Lazy Loading

```rust
// MEJOR (lazy loading)
impl Interpreter::new() {
    // NO cargar nada, solo preparar el sistema
    Self {
        environment: Environment::new(),
        module_loader: ModuleLoader::new(),
        // ...
    }
}
// Startup: <1ms ✨

// Cuando el usuario llama: math.sqrt(16)
// 1. Runtime: "math" module not found
// 2. ModuleLoader::load_module("math")
// 3. register_math_module() ejecuta
// 4. Caches en environment
// 5. math.sqrt(16) → 4.0

// Llamadas posteriores a math.* → Hit del cache, instant
```

---

## Architecture: Module On-Demand Registry

### 1. ModuleRegistry: El Centro de Control

```rust
// src/runtime/module_registry.rs (NEW)

pub struct ModuleRegistry {
    /// Maps module name to its registration function
    /// Example: "math" → register_math_module
    registrations: HashMap<String, Arc<dyn Fn(&mut Registrar)>>,

    /// Cache of loaded modules
    loaded: Arc<RwLock<HashSet<String>>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            registrations: HashMap::new(),
            loaded: Arc::new(RwLock::new(HashSet::new())),
        };

        // Register ALL module registration functions
        // But DON'T execute them yet
        registry.register("math", Arc::new(register_math_module));
        registry.register("string", Arc::new(register_string_module));
        registry.register("array", Arc::new(register_array_module));
        registry.register("json", Arc::new(register_json_module));
        registry.register("time", Arc::new(register_time_module));
        registry.register("random", Arc::new(register_random_module));
        registry.register("io", Arc::new(register_io_module));
        registry.register("http", Arc::new(register_http_module));

        registry
    }

    /// Register a module's loader function (metadata only, no execution)
    pub fn register(
        &mut self,
        name: &str,
        loader: Arc<dyn Fn(&mut Registrar)>,
    ) {
        self.registrations.insert(name.to_string(), loader);
    }

    /// Load a module (lazy) - only execute if not already loaded
    pub fn load_module(
        &self,
        name: &str,
        registrar: &mut Registrar,
    ) -> Result<()> {
        // Check if already loaded
        if self.loaded.blocking_read().contains(name) {
            return Ok(());
        }

        // Get the registration function
        let loader = self.registrations
            .get(name)
            .ok_or(format!("Module '{}' not found", name))?;

        // Execute it (first time only)
        (loader)(registrar);

        // Mark as loaded
        self.loaded.blocking_write().insert(name.to_string());

        Ok(())
    }

    /// Check if module is available (without loading)
    pub fn has_module(&self, name: &str) -> bool {
        self.registrations.contains_key(name)
    }

    /// Get list of available modules
    pub fn list_modules(&self) -> Vec<String> {
        self.registrations.keys().cloned().collect()
    }
}
```

---

## 2. Lazy Resolution in Interpreter

```rust
// src/interpreter/mod.rs

pub struct Interpreter {
    environment: Environment,
    registrar: Arc<Mutex<Registrar>>,
    module_registry: Arc<ModuleRegistry>,
    // ...
}

impl Interpreter {
    /// Fast initialization - no module loading
    pub async fn new() -> Self {
        let registrar = Registrar::new();
        let mut module_registry = ModuleRegistry::new();
        let mut env = Environment::new();

        // 1. Only register builtins (print, println, len, input)
        Self::register_builtins(&mut registrar);
        registrar.apply_to_environment(&mut env);

        // 2. Store registry for lazy loading later
        Self {
            environment: env,
            registrar: Arc::new(Mutex::new(registrar)),
            module_registry: Arc::new(module_registry),
        }
    }

    /// When a function call happens: identifier.method()
    /// Called from interpreter during evaluation
    pub fn resolve_function(
        &mut self,
        namespace: &str,
        function_name: &str,
    ) -> Result<RuntimeValue> {
        // 1. Check if already in environment (fast path)
        let full_name = format!("{}.{}", namespace, function_name);
        if let Some(func) = self.environment.get(&full_name) {
            return Ok(func);
        }

        // 2. Check if module exists in registry
        if !self.module_registry.has_module(namespace) {
            return Err(format!("Module '{}' not found", namespace));
        }

        // 3. Load module (first call only)
        {
            let mut registrar = self.registrar.lock().unwrap();
            self.module_registry.load_module(namespace, &mut registrar)?;
            registrar.apply_to_environment(&mut self.environment);
        }

        // 4. Try again (should be in environment now)
        self.environment
            .get(&full_name)
            .ok_or(format!("Function '{}' not found", full_name))
    }
}
```

---

## 3. Integration Point: Function Resolution

```rust
// In src/interpreter/expressions.rs, when evaluating MemberAccess
// Example: math.sqrt(16)

fn eval_member_access(&mut self, object: &Expression, property: &str) -> Result<RuntimeValue> {
    match object {
        // Case: namespace.function (like math.sqrt)
        Expression::Identifier(module_name) => {
            // Lazy load the module
            self.resolve_function(module_name, property)
        }

        // Case: object.method (like "hello".upper())
        _ => {
            // ... existing object method resolution
        }
    }
}
```

---

## 4. Comparison: Eager vs Lazy

```
┌──────────────────────────────────────────────────┐
│         EAGER LOADING (Original Proposal)        │
├──────────────────────────────────────────────────┤
│ Startup: 28ms (ALL modules loaded)               │
│ math.sqrt() call: <1ms (already in memory)       │
│ Memory: ~2MB (all modules registered)            │
│                                                  │
│ Use case: Applications using MANY modules       │
│ Example: Data processing app using math,        │
│          string, array, json, io all together   │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│         LAZY LOADING (Better for REPL/Scripts)   │
├──────────────────────────────────────────────────┤
│ Startup: <1ms (nothing loaded)                   │
│ First math.sqrt() call: 5ms (math loaded)        │
│ Subsequent math.* calls: <1ms (cached)           │
│ Memory: ~100KB (only loaded modules in memory)   │
│                                                  │
│ Use case: REPL, scripts, one-off programs       │
│ Example: User runs script that only uses math.  │
│          Why load string, array, http?          │
└──────────────────────────────────────────────────┘
```

---

## 5. Real-World Scenarios

### Scenario A: REPL User (Lazy is Better ✅)

```
$ raccoon repl

> let x = math.sqrt(16)
  [LAZY LOAD math module] (5ms first time)
  math.sqrt loaded and cached
  = 4.0

> let y = math.pow(2, 10)
  (cache hit, instant)
  = 1024.0

> let words = "hello world".split(" ")
  [LAZY LOAD string module] (3ms first time)
  string.split loaded and cached
  = ["hello", "world"]

Total overhead: 8ms spread across 3 calls
With eager: Would have been 28ms at startup, even if user never uses http or io
```

### Scenario B: Data Processing App (Eager Might Be Better)

```rust
// app.rcc - uses many modules
use math.sqrt, math.pow
use string.upper, string.split
use array.map, array.filter
use json.parse, json.stringify

fn process_data(file: string) {
    let data = json.parse(io.readFile(file))
    // ... uses 6+ modules extensively
}

// With eager: 28ms setup, then instant all calls
// With lazy: 20ms spread across first calls to each module
// Either way is OK, but eager slightly better here
```

---

## 6. Hybrid Approach (Best of Both Worlds)

```rust
// Optional: Pre-load common modules
impl Interpreter {
    pub async fn new() -> Self {
        let mut registrar = Registrar::new();
        let mut module_registry = ModuleRegistry::new();
        let mut env = Environment::new();

        // 1. Always load builtins (used 100% of the time)
        Self::register_builtins(&mut registrar);
        registrar.apply_to_environment(&mut env);

        // 2. Optionally pre-load "common" modules
        // These are loaded eagerly because they're used often
        if cfg!(feature = "preload_common") {
            // Load math, string - they're small and common
            module_registry.load_module("math", &mut registrar)?;
            module_registry.load_module("string", &mut registrar)?;
            registrar.apply_to_environment(&mut env);
        }

        // 3. Everything else lazy-loaded on demand
        Self {
            environment: env,
            registrar: Arc::new(Mutex::new(registrar)),
            module_registry: Arc::new(module_registry),
        }
    }
}

// In Cargo.toml:
// [features]
// preload_common = []  # Optional preloading for performance
```

---

## 7. Implementation Order

### Phase A: Foundation (Lazy Loading Ready)
```
src/runtime/
├── conversion.rs          ✓ (already planned)
├── registrar.rs           ✓ (already planned)
├── module_registry.rs     ✨ NEW (1 hour)
└── mod.rs                 (update exports)
```

### Phase B: Interpreter Integration
```
src/interpreter/
├── mod.rs                 (update resolve_function)
├── expressions.rs         (call resolve_function on member access)
```

### Phase C: Native Modules
```
src/runtime/natives/
├── math.rs                → generates register_math_module()
├── string.rs              → generates register_string_module()
└── ...                    (same as before)
```

**Key difference**: register_*_module() functions are NEVER called automatically.
They're only called when ModuleRegistry::load_module() is invoked.

---

## 8. Code Example: The Flow

```rust
// User code
let x = math.sqrt(16)

// Interpreter evaluation
eval_call_expression(
    Identifier("math"),      // namespace
    "sqrt",                  // function
    [Literal(16)]           // args
)

// Step 1: Try to find "math.sqrt" in environment
let func = env.get("math.sqrt")?;  // NOT FOUND (first time)

// Step 2: Call resolve_function
let func = interp.resolve_function("math", "sqrt")?;
    // Inside resolve_function:
    // 1. Check cache: not loaded
    // 2. Check registry: "math" exists
    // 3. Load: module_registry.load_module("math", &mut registrar)
    //    → This calls register_math_module() for the first time
    //    → All math.* functions added to registrar
    // 4. Apply: registrar.apply_to_environment()
    //    → All functions now in environment
    // 5. Get from environment again: SUCCESS
    // 6. Return function

// Step 3: Call the function
func.call([16])  // math.sqrt(16) → 4.0

// Step 4: Second call to math.pow
let y = math.pow(2, 10)
// resolve_function("math", "pow")
// 1. Check cache: "math" IS LOADED
// 2. Get from environment: SUCCESS (instant)
// 3. Call function: instant
```

---

## 9. Comparison Table

| Aspect | Eager | Lazy | Hybrid |
|--------|-------|------|--------|
| **Startup time** | 28ms | <1ms | 1-5ms |
| **REPL responsiveness** | Good | Excellent | Excellent |
| **First module call** | N/A | 3-5ms | <1ms (if preloaded) |
| **Subsequent calls** | <1ms | <1ms | <1ms |
| **Memory (idle)** | 2MB | 100KB | 500KB |
| **Complexity** | Low | Medium | Medium |
| **Best for** | Heavy apps | REPL/Scripts | Both |

---

## 10. Recommendation

**For Raccoon, I recommend: LAZY LOADING**

### Why:
1. ✅ Raccoon is primarily a REPL + script runner
2. ✅ Users rarely use ALL modules in one session
3. ✅ <1ms startup is critical for interactive experience
4. ✅ Users don't care if `math.sqrt()` takes 5ms on first call
5. ✅ Subsequent calls are instant anyway
6. ✅ Option to preload common modules if needed later

### Exception:
If you add a `--preload` flag for server mode or web app mode, you can enable eager loading there.

---

## 11. Implementation Code

### Create `src/runtime/module_registry.rs`

```rust
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct ModuleRegistry {
    registrations: HashMap<String, Arc<dyn Fn(&mut crate::runtime::Registrar) + Send + Sync>>,
    loaded: Arc<Mutex<HashSet<String>>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            registrations: HashMap::new(),
            loaded: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn register<F>(
        &mut self,
        name: &str,
        loader: Arc<F>,
    )
    where
        F: Fn(&mut crate::runtime::Registrar) + Send + Sync + 'static,
    {
        self.registrations.insert(name.to_string(), Arc::new(loader) as Arc<_>);
    }

    pub fn load_module(
        &self,
        name: &str,
        registrar: &mut crate::runtime::Registrar,
    ) -> Result<(), String> {
        let mut loaded = self.loaded.lock().unwrap();

        // Already loaded?
        if loaded.contains(name) {
            return Ok(());
        }

        // Get loader
        let loader = self.registrations
            .get(name)
            .ok_or(format!("Module '{}' not found", name))?;

        // Execute loader (first time)
        (loader)(registrar);

        // Mark loaded
        loaded.insert(name.to_string());

        Ok(())
    }

    pub fn has_module(&self, name: &str) -> bool {
        self.registrations.contains_key(name)
    }

    pub fn list_modules(&self) -> Vec<String> {
        self.registrations.keys().cloned().collect()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 12. Next Steps

Choose your approach:

- [ ] **Eager Loading** - Load all modules at startup (28ms penalty but simpler code)
- [ ] **Lazy Loading** ⭐ - Load on demand (<1ms startup, better UX for REPL)
- [ ] **Hybrid** - Preload common modules, lazy-load others

Once you decide, I'll adapt the implementation plan!

