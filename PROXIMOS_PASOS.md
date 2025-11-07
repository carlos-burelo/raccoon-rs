# PRÓXIMOS PASOS: Refactor de Módulos (Con Prioridades)

## TL;DR - La Decisión

**DESTRUIR EL SISTEMA ACTUAL Y REHACERLO CON LA ARQUITECTURA NUEVA**

Razones:
- ✅ 65% menos código
- ✅ 87% menos boilerplate por función
- ✅ Un único punto de registro (Registrar)
- ✅ Cero duplicación
- ✅ Escalable sin esfuerzo
- ✅ Compilación más rápida (menos archivos)

Costo: ~10-12 horas de refactor, pero una vez hecho, agregar nuevas funciones toma 30 segundos.

---

## PLAN DE EJECUCIÓN (Orden de Prioridades)

### FASE 1: Foundation (2-3 horas) ⚙️

**Objetivo**: Crear la infraestructura base que el resto depende.

#### 1.1 Crear `src/runtime/conversion.rs` (Nueva)
**Archivo**: Centraliza TODAS las conversiones FromRaccoon/ToRaccoon

```rust
// Implementar:
- impl FromRaccoon for f64, i32, i64, bool, String, Vec<T>, HashMap<K,V>
- impl ToRaccoon para todos los anteriores
- Elimina duplicación de native.rs y rust_ffi.rs

Tiempo: 30 minutos
Riesgo: BAJO (no rompe nada, solo expone lo que ya existe)
```

**Pasos**:
1. Crear archivo vacío `src/runtime/conversion.rs`
2. Copiar de `native.rs` líneas 6-126 (FromRaccoon/ToRaccoon implementations)
3. Eliminar duplicación de `rust_ffi.rs`
4. Agregar `pub mod conversion;` en `src/runtime/mod.rs`
5. Update imports en `native.rs` y `rust_ffi.rs`

---

#### 1.2 Crear `src/runtime/registrar.rs` (Nueva)
**Archivo**: El corazón del nuevo sistema

```rust
// Implementar:
pub struct Registrar {
    functions: HashMap<String, FunctionSignature>,
    constants: HashMap<String, RuntimeValue>,
}

Métodos:
- register_fn(name, namespace, handler, min_args, max_args)
- register_async_fn(...)
- register_const(...)
- apply_to_environment(&self, env: &mut Environment)

Tiempo: 45 minutos
Riesgo: BAJO (standalone, no interactúa con nada aún)
```

**Pasos**:
1. Crear `src/runtime/registrar.rs`
2. Copiar estructura base del documento EJEMPLOS_CODIGO_NUEVO.md (sección 1)
3. Implementar `apply_to_environment()`
4. Agregar `pub mod registrar;` en `src/runtime/mod.rs`

---

#### 1.3 Crear macros `src/runtime/register_macros.rs` (Nueva)
**Archivo**: Las macros que eliminan boilerplate

```rust
// Implementar:
#[register_native] macro
#[register_native_async] macro
#[register_const] macro
#[auto_convert] macro (para tipos)

Tiempo: 1 hora
Riesgo: MEDIO (macros son complejas, necesita testing)
```

**Pasos**:
1. Crear `src/runtime/register_macros.rs`
2. Implementar `register_native!` macro (la más importante)
3. Implementar `register_native_async!` macro
4. Agregar `#[macro_export]` y exports en `src/runtime/mod.rs`
5. Testing: Probar macro con un módulo simple primero

---

### FASE 2: Module System Refactor (2 horas) 🔄

**Objetivo**: Unificar ModuleSystem + StdLibLoader en ModuleLoader único

#### 2.1 Crear `src/runtime/dependency_resolver.rs` (Nueva)
**Archivo**: Detecta imports circulares automáticamente

```rust
// Implementar:
pub struct DependencyResolver;
- fn resolve(module_path: &str, source: &str) -> Result<Vec<String>>
- fn extract_imports(source: &str) -> Result<Vec<String>>
- fn detect_cycles() -> Result<()>

Tiempo: 45 minutos
Riesgo: BAJO (standalone, no rompe sistema existente)
```

---

#### 2.2 Crear `src/runtime/module_loader.rs` (Nueva/Refactored)
**Archivo**: Unifica todo lo de ModuleSystem + StdLibLoader

```rust
// Fusionar:
- ModuleSystem::load_module() → ModuleLoader::load_module()
- StdLibLoader::load_stdlib() → ModuleLoader::load_stdlib_module()
- DependencyResolver integration
- Single cache (reemplaza ambos caches)

Tiempo: 1 hora
Riesgo: MEDIUM (combina 2 sistemas, pero ambos son simples)
```

**Pasos**:
1. Crear `src/runtime/module_loader.rs`
2. Copiar estructura base del documento EJEMPLOS_CODIGO_NUEVO.md (sección 7)
3. Integrar DependencyResolver
4. Reemplazar llamadas a ModuleSystem/StdLibLoader en interpreter/mod.rs

---

### FASE 3: Refactor Native Modules (3-4 horas) 🎯

**Objetivo**: Reescribir cada módulo nativo con las macros nuevas

#### 3.1 Refactor `src/runtime/natives/math.rs`
**Antes**: 150 líneas
**Después**: 40 líneas

```rust
use crate::register_native;

#[register_native]
pub mod math {
    pub fn sqrt(x: f64) -> f64 { x.sqrt() }
    pub fn pow(base: f64, exp: f64) -> f64 { base.powf(exp) }
    // ... 13 more functions
}

Tiempo: 30 minutos (primera vez es más lenta, aprendes el pattern)
Riesgo: LOW (macro maneja todas las conversiones)
```

**Pasos**:
1. Abrir `src/runtime/natives/math.rs`
2. Reemplazar TODA la función `register()` con el bloque `#[register_native]`
3. Copiar implementaciones de funciones del viejo archivo
4. Eliminar todas las conversiones manuales RuntimeValue
5. Test: Ejecutar `cargo build` para verificar macro expansion

---

#### 3.2-3.8 Refactor remaining modules (30 min cada uno)
```
string.rs    (200 líneas → 50 líneas)  - 30 min
array.rs     (150 líneas → 45 líneas)  - 30 min
json.rs      (138 líneas → 30 líneas)  - 20 min
time.rs      (150+ líneas → 35 líneas) - 30 min
random.rs    (100+ líneas → 30 líneas) - 20 min
io.rs        (226 líneas → 80 líneas)  - 45 min
http.rs      (330 líneas → 100 líneas) - 60 min
```

**Total FASE 3**: 3.5 horas

---

### FASE 4: Builtins & Initialization (1 hora) 🚀

**Objetivo**: Simplificar builtins.rs y crear init_all() único

#### 4.1 Simplificar `src/runtime/builtins.rs`
**Antes**: 502 líneas
**Después**: 30 líneas

```rust
pub fn register_builtins(registrar: &mut Registrar) {
    registrar.register_fn(
        "print",
        None,
        |args| { /* closure */ },
        0,
        None,  // variadic
    );
    registrar.register_fn(
        "println",
        None,
        |args| { /* closure */ },
        0,
        Some(1),
    );
    // input, len
}

Tiempo: 20 minutos
Riesgo: LOW
```

---

#### 4.2 Reescribir `src/interpreter/mod.rs` initialization
**Antes**: setup_builtins() solamente
**Después**: init_all() que carga todo

```rust
impl Interpreter {
    pub async fn new() -> Self {
        let mut env = Environment::new();
        Self::init_all(&mut env).await.unwrap();
        Self { environment: env, ... }
    }

    async fn init_all(env: &mut Environment) -> Result<()> {
        let mut registrar = Registrar::new();

        // 1. Builtins
        register_builtins(&mut registrar);

        // 2. Native modules (auto-generated)
        register_math_module(&mut registrar);
        register_string_module(&mut registrar);
        // ... all others

        // 3. Apply
        registrar.apply_to_environment(env);

        Ok(())
    }
}

Tiempo: 30 minutos
Riesgo: LOW
```

---

### FASE 5: FFI Cleanup (1 hora) 🧹

**Objetivo**: Limpiar y refactor FFI system

#### 5.1 Remove duplication from `src/runtime/rust_ffi.rs`
```rust
// BEFORE: Lines 6-126 are duplicated from native.rs
// AFTER: Import from conversion.rs

Tiempo: 15 minutos
Riesgo: LOW
```

#### 5.2 Register FFI modules using new system
```rust
// Create register_rust_ffi_modules() function
// Use registrar.register_fn() for each FFI function

Tiempo: 30 minutos
Riesgo: MEDIUM
```

#### 5.3 Delete dead code
```rust
// DELETE:
- builtin_plugins.rs (94 líneas) - UNUSED
- plugin_system.rs (177 líneas) - UNUSED
- Old native.rs (si todas conversiones están en conversion.rs)
- stdlib_loader.rs (si moduleloader unificado funciona)

Tiempo: 15 minutos
Riesgo: LOW (after confirming everything works)
```

---

## DETAILED TIMELINE

```
Day 1 - Morning (3 hours)
├─ FASE 1.1: conversion.rs                [✓ 30 min]
├─ FASE 1.2: registrar.rs                 [✓ 45 min]
├─ FASE 1.3: register_macros.rs           [✓ 1 hour]
└─ Test: cargo build works                [✓ 15 min]

Day 1 - Afternoon (2 hours)
├─ FASE 2.1: dependency_resolver.rs       [✓ 45 min]
├─ FASE 2.2: module_loader.rs             [✓ 1 hour]
└─ Test: Module loading works             [✓ 15 min]

Day 2 - Morning (3 hours)
├─ FASE 3.1: Refactor math.rs             [✓ 30 min]
├─ FASE 3.2-3.4: string, array, json      [✓ 1.5 hours]
└─ Test: Each module builds and compiles  [✓ 1 hour]

Day 2 - Afternoon (2 hours)
├─ FASE 3.5-3.8: time, random, io, http   [✓ 2 hours]
└─ Test: cargo build clean                [✓ 15 min]

Day 3 - Morning (1.5 hours)
├─ FASE 4.1: Simplify builtins.rs         [✓ 20 min]
├─ FASE 4.2: Update interpreter init      [✓ 30 min]
└─ Integration test: All 150+ functions available [✓ 20 min]

Day 3 - Afternoon (1 hour)
├─ FASE 5.1-5.3: FFI Cleanup              [✓ 45 min]
└─ Final test: cargo build + cargo test   [✓ 15 min]

TOTAL: 12-13 hours spread over 3 days
```

---

## TESTING STRATEGY

After each phase, verify:

```bash
# After FASE 1 (Foundation)
cargo check                    # Macros compile?
cargo build                    # No errors?

# After FASE 2 (Module System)
# Create test_module.rcc and load it
# Verify no duplicate caching

# After FASE 3 (Native Modules)
cargo build
# Check that register_math_module(), etc. are generated

# After FASE 4 (Initialization)
cargo run -- repl
> print("hello")              # Works?
> let x = 2.sqrt()            # math.sqrt works?
> "hello".upper()             # string.upper works?

# After FASE 5 (Cleanup)
cargo test
# All existing tests pass?
```

---

## RISK MITIGATION

| Risk | Mitigation |
|------|-----------|
| **Macros don't expand** | Start with simple functions, test early |
| **Type conversions break** | Keep old conversion.rs alongside, then replace |
| **Module loading fails** | Test module_loader with single module first |
| **Functions not initialized** | Print debug info in init_all() |
| **Performance regression** | Profile with large number of functions |

---

## ROLLBACK PLAN

If something breaks critically:

```bash
# Commit before each phase
git commit -m "Before FASE X"

# If FASE N fails:
git reset --hard HEAD~1
# Re-assess and try different approach
```

---

## DECISION MATRIX

| Option | Code Reduction | Maintainability | Time | Risk | Boilerplate |
|--------|---|---|---|---|---|
| **Do nothing** | 0% | 17% | 0 | LOW | 100% |
| **Patch current** | 20% | 40% | 2h | LOW | 80% |
| **Full refactor** ⭐ | 65% | 95% | 12h | MED | 13% |

**RECOMMENDATION**: Full refactor now. Pain today, smooth sailing forever.

---

## SUCCESS CRITERIA

After refactor is complete:

- [ ] ✅ `cargo build` compiles cleanly (no warnings)
- [ ] ✅ All 150+ functions available in REPL
- [ ] ✅ `math.sqrt(16)` returns 4.0
- [ ] ✅ `string.upper("hello")` returns "HELLO"
- [ ] ✅ Adding new function takes <1 minute
- [ ] ✅ No code duplication in registration
- [ ] ✅ Single Registrar is source of truth
- [ ] ✅ Circular imports detected automatically
- [ ] ✅ Module caching unified (one cache only)
- [ ] ✅ `src/runtime/` folder has <1,500 lines (was 3,500)

---

## QUESTIONS TO ANSWER BEFORE STARTING

1. **Do you want to keep FFI compatibility?**
   - YES → Keep rust_ffi.rs, just clean it up
   - NO → Delete it entirely

2. **Do you want async function support in the new system?**
   - YES → Include register_native_async! macro
   - NO → Only sync functions

3. **Timeline preference?**
   - AGGRESSIVE: 2-3 days straight
   - NORMAL: 1-2 weeks, 2-3 hours per day
   - SLOW: Whenever you have time

4. **Testing approach?**
   - Unit tests for each module: Add `#[test]` functions
   - Integration tests: Load and call functions in REPL
   - Regression tests: Run against existing test suite

---

## NEXT STEP

When ready, start with FASE 1.1:

```bash
# 1. Create the file
touch src/runtime/conversion.rs

# 2. Start with imports
cat > src/runtime/conversion.rs << 'EOF'
use crate::runtime::RuntimeValue;
use std::collections::HashMap;

pub trait FromRaccoon: Sized {
    fn from_raccoon(val: &RuntimeValue) -> Result<Self>;
}

pub trait ToRaccoon {
    fn to_raccoon(self) -> RuntimeValue;
}
EOF

# 3. Copy implementations from native.rs
# 4. Update mod.rs
# 5. cargo check

echo "✓ FASE 1.1 complete!"
```

Let me know when you want to start and which phase you want to tackle first! 🚀

