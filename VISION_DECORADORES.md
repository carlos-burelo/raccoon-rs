# Visión: Decoradores Definibles por Usuario en Raccoon

## Objetivo Final

**Decoradores como feature first-class del lenguaje, completamente definibles y extensibles por usuarios.**

Sin implementaciones hardcodeadas en Rust. Sin capas de abstracción raras. Solo Raccoon.

---

## ¿Por Qué?

Actualmente:
- ❌ Decoradores están hardcodeados en Rust (`DecoratorRegistry`, `@deprecated`, `@cache`, etc.)
- ❌ No puedes definir tus propios decoradores
- ❌ Cada decorador nuevo requiere cambios en código Rust
- ❌ FFI es un sistema paralelo desacoplado de decoradores

Con esta visión:
- ✅ Decoradores se definen en Raccoon como funciones especiales
- ✅ Usuarios pueden crear cualquier decorador que necesiten
- ✅ El lenguaje simplemente aplica transformaciones
- ✅ FFI integrado como decorador estándar
- ✅ Extensible sin tocar código Rust

---

## Sintaxis Final

```raccoon
// Definir un decorador
decorator cache(ms: int = 5000) {
    let store = {}
    return fn(...args) {
        let key = stringify(args)
        if (store.has(key)) return store.get(key)
        let result = this(...args)
        store.set(key, result)
        setTimeout(() => store.delete(key), ms)
        return result
    }
}

// Definir otro decorador con lógica por contexto
decorator log(prefix: str = "[LOG]") {
    return match context.type {
        "function" => fn(...args) {
            print(prefix, context.name, "called")
            return this(...args)
        },
        "method" => fn(...args) {
            print(prefix, "Method", context.name)
            return this(...args)
        },
        _ => this
    }
}

// Usarlos
@cache(3000)
fn fibonacci(n: int): int {
    return n <= 1 ? n : fibonacci(n-1) + fibonacci(n-2)
}

@log("[TRACE]")
fn process(x: int) {
    return x * 2
}

class Service {
    @cache(10000)
    method getData(): str {
        return api.fetch()
    }

    @log("[API]")
    method sendData(data: obj) {
        api.post(data)
    }
}

// FFI integrado
@native("math", "sqrt")
fn sqrt(x: float): float

@native("strings", "reverse")
fn reverse(s: str): str
```

---

## Cómo Funciona

### 1. Definición (Parse Time)

```
Código Raccoon con "decorator" keyword
        ↓
Parser reconoce y crea DecoratorDefinition (AST)
        ↓
Se almacena en DecoratorRegistry
```

### 2. Aplicación (Runtime)

```
Función decorada es encontrada: @cache(5000) fn foo() { }
        ↓
Interpreter busca definición de @cache en registry
        ↓
Crea environment con:
  - "this" = la función siendo decorada
  - "context" = { type: "function", name: "foo", target: FnDecl }
  - Parámetros del decorador: ms = 5000
        ↓
Ejecuta el cuerpo del decorador
        ↓
El decorador retorna una función decorada
        ↓
La función decorada es registrada en environment
```

### 3. Llamada

```
Cuando llamas foo(), ejecutas la versión decorada (con cache)
```

---

## Contexto Implícito

Dos variables siempre disponibles dentro de un decorador:

### `this`
La entidad siendo decorada. Tipo depende del contexto:
- **Function**: `RuntimeValue::Function` - la función original
- **Method**: `RuntimeValue::Function` - el método original
- **Class**: `RuntimeValue::Class` - la clase original
- **Property**: El valor de la propiedad (puede ser int, str, obj, etc.)

### `context`
Objeto con metadatos sobre qué estoy decorando:
```rust
{
    type: "function" | "method" | "class" | "property" | "async_function",
    name: "nombreDeEntidad",
    target: DecoratorTarget  // Para acceso a AST si necesitas
}
```

---

## Ejemplo: Decorador Completo

### Definición

```raccoon
decorator observable(eventName: str = "changed") {
    // Acceso implícito a 'this' (la propiedad/método/función)
    // Acceso implícito a 'context' (tipo, nombre, etc.)

    // Match es expresión - retorna lo que va a ejecutarse
    return match context.type {
        // Para propiedades
        "property" => {
            let originalValue = this
            return {
                get: fn() { return originalValue },
                set: fn(newValue) {
                    originalValue = newValue
                    // Disparar evento
                    emit(eventName, { newValue })
                }
            }
        },

        // Para métodos
        "method" => fn(...args) {
            let result = this(...args)
            emit(eventName, { method: context.name, args, result })
            return result
        },

        // Para funciones
        "function" => fn(...args) {
            let result = this(...args)
            emit(eventName + ":" + context.name, { args, result })
            return result
        },

        // Default: no hacer nada
        _ => this
    }
}
```

### Uso

```raccoon
class User {
    @observable("userCreated")
    property name: str = "John"

    @observable("dataFetched")
    method loadProfile() {
        return api.getProfile()
    }
}

class DataService {
    @observable()
    fn fetchData(id: int) {
        return api.fetch(id)
    }
}
```

---

## Antes vs Después

### ANTES (Hardcodeado en Rust)

```rust
// En src/runtime/decorator_registry.rs
impl DecoratorRegistry {
    fn register_all_decorators(&mut self) {
        self.register_decorator(DecoratorSpec {
            name: "@cache".to_string(),
            description: "Caches function results...",
            allowed_on: vec![DecoratorTarget::Function],
        });
        // ... más decoradores hardcodeados
    }
}

// En src/interpreter/declarations.rs
for decorator_info in &decorators {
    match decorator_info.spec.name.as_str() {
        "@deprecated" => { /* lógica especial */ }
        "@cache" => { /* lógica especial */ }
        _ => {} // Se ignoran
    }
}
```

**Problemas**:
- Cada decorador nuevo = cambio en Rust
- FFI es sistema separado
- No hay extensibilidad
- Decoradores se ignoran en mayoría de contextos

### DESPUÉS (Definibles en Raccoon)

```raccoon
// En stdlib/decorators.rcc
decorator deprecated(msg: str = "") {
    return match context.type {
        "function" => fn(...args) {
            print("⚠️  Deprecated function: " + context.name)
            if (msg) print("   " + msg)
            return this(...args)
        },
        "class" => this,  // Las clases no se pueden realmente deprecar, solo warn
        _ => this
    }
}

decorator cache(ms: int = 5000) {
    let store = {}
    return fn(...args) {
        let key = stringify(args)
        if (store.has(key)) return store.get(key)
        let result = this(...args)
        store.set(key, result)
        setTimeout(() => store.delete(key), ms)
        return result
    }
}

// Usuario puede hacer sus propios decoradores
decorator timing {
    return fn(...args) {
        let start = now()
        let result = this(...args)
        let elapsed = now() - start
        print(context.name + " took " + elapsed + "ms")
        return result
    }
}
```

**Ventajas**:
- Todo en Raccoon, fácil de entender
- Usuarios pueden extender
- Un solo sistema de decoradores
- FFI integrado sin boilerplate

---

## Dependencias de Implementación

Para que esto funcione necesitamos primero 3 features:

### 1. Funciones Anónimas ✅
```raccoon
let fn_value = fn(x: int) {
    return x * 2
}

let arrow = fn(x) => x * 2
```

**Por qué**: Los decoradores retornan funciones anónimas.

### 2. Pattern Matching ✅
```raccoon
let msg = match value {
    0 => "cero",
    1..10 => "pocos",
    is int => "entero",
    _ => "otro"
}
```

**Por qué**: Decoradores necesitan lógica diferente por contexto.

### 3. Clases Anónimas ✅
```raccoon
let MyClass = class {
    property x: int = 10
}

let Enhanced = class extends Original {
    method override getValue() {
        return super.getValue() * 2
    }
}
```

**Por qué**: Algunos decoradores extenderán clases.

---

## Timeline

| Paso | Feature | Tiempo | Bloqueador |
|------|---------|--------|-----------|
| 1 | Funciones Anónimas | 2-3h | Ninguno |
| 2 | Pattern Matching | 4-6h | Paso 1 |
| 3 | Clases Anónimas | 2-3h | Paso 1 |
| 4 | Decoradores | 3-4h | Pasos 1-3 |
| **Total** | | **11-16h** | |

---

## Cambios Arquitecturales

### Parser
- Agregar keyword `decorator`
- Distinguir `fn { stmts }` de `fn => expr`
- Agregar `match expr { arms }` como expresión
- Hacer nombre de clase opcional

### AST
- `DecoratorDefinition` (nueva declaración)
- `AnonymousFnExpr` (nueva expresión)
- `MatchExpr` (nueva expresión)
- `ClassLiteral` (nueva expresión)
- `Pattern` enum (para matching)

### Interpreter
- `execute_decorator_def()` - Registrar decorador en registry
- `apply_decorators()` - Aplicar decoradores a una entidad
- `evaluate_match_expr()` - Evaluar match como expresión
- `evaluate_anonymous_fn()` - Crear función anónima
- `evaluate_class_literal()` - Crear clase anónima

### Runtime
- `DecoratorRegistry` - Almacenar funciones, no specs
- Agregar `this` y `context` al ambiente cuando ejecuta decorador
- `create_context_object()` - Helper para crear contexto

### Remover
- `DecoratorSpec` (ya no necesario)
- `allowed_on` (validación implícita)
- Lógica hardcodeada de decoradores
- `NativeDecoratorProcessor`

---

## Casos de Uso Reales

### 1. Cache Automático
```raccoon
@cache(5000)
async fn fetchUserData(id: int): obj {
    return await api.getUser(id)
}
```

### 2. Logging Automático
```raccoon
@log("[API]")
async fn sendRequest(endpoint: str, data: obj): obj {
    return await http.post(endpoint, data)
}
```

### 3. Retry Automático
```raccoon
@retry(3, 1000)
async fn unstableOperation(): result {
    return await riskyCall()
}
```

### 4. Timing de Rendimiento
```raccoon
@timing
fn complexCalculation(n: int): int {
    // Automáticamente loguea cuánto tardó
    return fibonacci(n)
}
```

### 5. Validación
```raccoon
decorator validate(schema: obj) {
    return fn(...args) {
        // Validar argumentos con schema
        for (let arg of args) {
            if (!matches(arg, schema)) {
                throw "Validation failed"
            }
        }
        return this(...args)
    }
}

@validate({ type: "int", min: 0, max: 100 })
fn processPercentage(value: int): str {
    return value + "%"
}
```

### 6. Autorización
```raccoon
@require("admin")
method deleteUser(id: int) {
    // Solo ejecuta si usuario es admin
    return db.delete("users", id)
}
```

### 7. Transacciones
```raccoon
@transaction
method transferMoney(from: str, to: str, amount: float) {
    // Automáticamente envuelto en transacción
    db.update("accounts", from, { balance: -amount })
    db.update("accounts", to, { balance: +amount })
}
```

---

## Conclusión

**Los decoradores dejan de ser una capa de abstracción rara en Rust para convertirse en una feature normal del lenguaje.**

El usuario define qué es un decorador y cómo funciona. El sistema simplemente lo aplica.

**Elegante. Simple. Extensible. Raccoon.**
