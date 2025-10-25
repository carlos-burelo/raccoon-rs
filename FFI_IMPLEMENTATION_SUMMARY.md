# FFI Implementation Summary - Raccoon Runtime Independence

## 🎯 Objetivo Alcanzado

Crear un sistema de decoradores potente que permite:
- ✅ Registrar funciones dinámicamente en FFI Registry
- ✅ Separar decoradores internos (stdlib) de públicos (users)
- ✅ Validar decoradores según contexto (file, target)
- ✅ Preparar runtime para independencia de Rust hardcode

---

## 📦 Componentes Creados

### 1. **src/runtime/decorator_registry.rs** (267 líneas)

**Responsabilidad:** Registro y validación de decoradores

```rust
pub struct DecoratorRegistry {
    decorators: HashMap<String, DecoratorSpec>,
}

pub enum DecoratorVisibility {
    Internal,  // @_ffi, @_register, @_validate
    Public,    // @cache, @deprecated, @pure, @inline, @readonly, @override
}
```

**Funciones:**
- `new()` - Crea registry con todos los decoradores registrados
- `register_decorator()` - Registra un nuevo decorador
- `validate()` - Valida decoradores para un contexto (file, target, stdlib)
- `exists()` - Verifica si decorador existe

**Decoradores Registrados:**

| Decorador | Visibilidad | Target | Propósito |
|-----------|-------------|--------|-----------|
| `@_ffi()` | Internal | Function, AsyncFn | Registra en FFI Registry |
| `@_register(ns)` | Internal | Function, AsyncFn | Registra en namespace |
| `@_validate()` | Internal | Function, AsyncFn | Validación automática |
| `@cache(ttl)` | Public | Function, AsyncFn | Cache de resultados |
| `@deprecated(msg)` | Public | Fn, AsyncFn, Class | Marca como deprecated |
| `@pure()` | Public | Function, AsyncFn | Sin side effects |
| `@inline()` | Public | Function, AsyncFn | Sugerir inline |
| `@readonly()` | Public | ClassProperty | Propiedad readonly |
| `@override()` | Public | ClassMethod | Override de base |

---

### 2. **src/runtime/decorators.rs** (115 líneas)

**Responsabilidad:** Metadatos y aplicación de decoradores

```rust
pub struct DecoratorMetadata {
    pub is_ffi: bool,
    pub namespace: Option<String>,
    pub validate: bool,
    pub cache_ttl_ms: Option<i64>,
    pub is_deprecated: Option<String>,
    pub is_pure: bool,
    pub should_inline: bool,
}

pub struct FunctionCache { ... }

pub struct DecoratorApplier { ... }
```

**Funciones:**
- `FunctionCache::get()` - Obtiene valor cacheado
- `FunctionCache::set()` - Guarda valor en cache
- `DecoratorApplier::apply_cache()` - Aplica lógica de caché
- `DecoratorApplier::apply_deprecated()` - Emite warning

---

### 3. **src/runtime/ffi_registry.rs** (336 líneas)

**Responsabilidad:** Registro dinámico de funciones invocables

```rust
pub struct FFIRegistry {
    functions: Arc<RwLock<HashMap<String, FFIFunctionInfo>>>,
    async_functions: Arc<RwLock<HashMap<String, FFIFunctionInfo>>>,
    implementations: Arc<RwLock<HashMap<String, FFIFunction>>>,
    async_implementations: Arc<RwLock<HashMap<String, FFIAsyncFunction>>>,
    namespaces: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

pub type FFIFunction = Arc<dyn Fn(Vec<RuntimeValue>) -> RuntimeValue + Send + Sync>;
```

**Funciones:**
- `register_function()` - Registra función síncrona
- `register_async_function()` - Registra función async
- `call_function()` - Llama función registrada
- `call_async_function()` - Llama función async registrada
- `get_function_info()` - Obtiene metadatos
- `list_functions()` - Lista todas las funciones
- `list_namespace()` - Lista funciones en namespace
- `exists()` - Verifica si existe función

---

### 4. **Modificaciones a Interpreter** (src/interpreter/mod.rs)

**Cambios:**
- Importa `DecoratorRegistry` y `FFIRegistry`
- Agrega campos al struct `Interpreter`
- Inicializa registries en `new()`
- Valida decoradores en `execute_fn_decl()`
- Métodos helpers:
  - `get_ffi_registry()` - Acceso público a FFI
  - `get_decorator_registry()` - Acceso público a decoradores
  - `is_in_stdlib()` - Detecta si archivo es stdlib

---

## 🔄 Flujo de Ejecución

### Cuando se Declara una Función en Stdlib

```
stdlib/io.rcc contiene:
  @_ffi()
  @cache(60000)
  export fn readFile(path: str): str { ... }

1. Parser: Parsea decoradores → DecoratorDecl[]
2. Interpreter.execute_fn_decl():
   a. Detecta decoradores
   b. Llama decorator_registry.validate()
      - ✓ @_ffi es valid
      - ✓ @cache es valid
      - ✓ archivo es stdlib → permite @_ffi
   c. Crea FunctionValue
   d. Declara en environment
   e. TODO: Procesar @_ffi() para registrar en FFIRegistry
   f. TODO: Procesar @cache() para cachear resultados
```

---

## 📊 Arquitectura Visual

```
┌─────────────────────────────────────────────────────┐
│        USUARIO ESCRIBE CÓDIGO RACCOON               │
├─────────────────────────────────────────────────────┤
│                                                     │
│  @_ffi()                  (solo stdlib)             │
│  @cache(60000)            (todos)                   │
│  @deprecated("msg")       (todos)                   │
│  @pure()                  (todos)                   │
│                                                     │
└─────────────────────────────────────────────────────┘
                    ↓ Parser
┌─────────────────────────────────────────────────────┐
│        AST CON DECORADORES (DecoratorDecl[])       │
└─────────────────────────────────────────────────────┘
                    ↓ Interpreter
┌─────────────────────────────────────────────────────┐
│   1. DecoratorRegistry.validate()                   │
│      - ¿Decorador existe?                          │
│      - ¿Es permitido en este contexto?             │
│      - ¿Archivo es stdlib?                         │
│                                                     │
│   2. Procesar efectos del decorador                │
│      - @_ffi() → FFIRegistry.register_function()  │
│      - @cache() → DecoratorApplier.apply_cache()  │
│      - @deprecated() → DecoratorApplier.apply_...  │
│                                                     │
└─────────────────────────────────────────────────────┘
                    ↓ Runtime
┌─────────────────────────────────────────────────────┐
│   FFIRegistry (funciones registradas)               │
│   FunctionCache (caché de resultados)               │
│   Environment (variables y funciones)               │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## 🛡️ Validación de Seguridad

### Regla 1: Decoradores Internos Solo en Stdlib

```raccoon
// stdlib/io.rcc - ✅ PERMITIDO
@_ffi()
export fn readFile(path: str): str { ... }

// user_code.rcc - ❌ ERROR
@_ffi()
export fn myFunc(): int { ... }
// Error: "Decorator @_ffi is internal and can only be used in standard library"
```

**Implementación:**
```rust
if spec.visibility == DecoratorVisibility::Internal && !is_in_stdlib() {
    return Err(RaccoonError::new(...));
}
```

---

### Regla 2: Decoradores Solo en Targets Permitidos

```raccoon
// ✅ PERMITIDO
@cache(5000)
fn myFunc(): int { ... }

// ❌ ERROR
class MyClass {
    @cache(5000)  // No permitido en clase
}
```

**Implementación:**
```rust
if !spec.allowed_on.contains(&target) {
    return Err(RaccoonError::new(...));
}
```

---

## 📈 Próximos Pasos

### Fase 1: Completado ✅
- [x] Crear DecoratorRegistry con validación
- [x] Crear DecoratorMetadata y FunctionCache
- [x] Crear FFIRegistry para funciones dinámicas
- [x] Integrar en Interpreter
- [x] Documentación completa

### Fase 2: En Progreso ⏳
- [ ] Implementar procesamiento de decoradores en execute_fn_decl()
- [ ] Registrar funciones con @_ffi() en FFIRegistry
- [ ] Aplicar caché con @cache()
- [ ] Emitir warnings con @deprecated()

### Fase 3: Próximo
- [ ] Migrar stdlib/io.rcc a usar @_ffi()
- [ ] Migrar stdlib/*.rcc completo
- [ ] Eliminar NativeBridge (800+ líneas de hardcode)
- [ ] Runtime completamente independiente de Rust

### Fase 4: Avanzado
- [ ] Permitir usuarios registrar FFI functions
- [ ] Sistema de módulos FFI externos
- [ ] Plugins de terceros
- [ ] Hot reload de módulos

---

## 💡 Ejemplo de Uso Completo

### Antes (Hardcoded en Rust)
```rust
// native_bridge.rs - 800+ líneas
self.functions.insert(
    "native_io_read_file".to_string(),
    NativeFunctionValue::new(|args| { ... }, ...),
);
```

### Después (Declarativo en Raccoon)
```raccoon
// stdlib/io.rcc - 10 líneas
@_ffi()
@cache(60000)
export fn readFile(path: str): str {
    return internal_read_file(path);
}
```

---

## 🎓 Conclusión

El sistema implementado permite:

✅ **Separación clara** entre lógica stdlib y runtime
✅ **Seguridad** mediante validación de decoradores
✅ **Extensibilidad** sin recompilar
✅ **Metadatos** para optimizaciones futuras
✅ **Documentación** clara mediante decoradores
✅ **Preparación** para runtime totalmente independiente

**Siguiente:** Procesar decoradores en execute_fn_decl() para activar sus efectos.
