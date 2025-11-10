# Sistema de Primitivas con Carga Perezosa

## Resumen

Este documento describe el nuevo sistema de primitivas organizadas por contexto de operación con carga perezosa en Raccoon.

## Conceptos Clave

### 1. Contextos de Operación

Las primitivas están organizadas por contextos de operación en lugar de módulos std:

- **Math**: Operaciones matemáticas (sqrt, sin, cos, pow, abs, floor, ceil, etc.)
- **String**: Operaciones de cadenas (len, charAt, substring, toUpper, toLower, trim, etc.)
- **Array**: Operaciones de arreglos (join, sort, reverse, etc.)
- **IO**: Operaciones de entrada/salida de archivos (fileRead, fileWrite, fileAppend, etc.)
- **HTTP**: Operaciones HTTP (httpGet, httpPost, httpRequest, etc.)
- **Time**: Operaciones de tiempo (timeNow, timeNowMicros, sleep, etc.)
- **JSON**: Operaciones JSON (jsonParse, jsonStringify, etc.)
- **System**: Operaciones del sistema (print, println, envGet, envSet, exit, random, etc.)
- **Builtins**: Funciones globales integradas (print, println, eprint, input, len, etc.)

### 2. Carga Perezosa

Las primitivas solo se cargan cuando se solicita su contexto. Esto mejora el rendimiento al inicio y reduce el uso de memoria.

## Uso del Sistema

### Cargar un Contexto Específico

```rust
use raccoon::runtime::{LazyPrimitiveRegistry, PrimitiveContext};

// Crear el registro
let registry = LazyPrimitiveRegistry::new(registrar);

// Cargar solo el contexto Math
registry.load_context(PrimitiveContext::Math);

// Ahora todas las primitivas matemáticas están disponibles
```

### Cargar Todos los Contextos

```rust
// Cargar todos los contextos a la vez
registry.load_all();
```

### Verificar si un Contexto está Cargado

```rust
if registry.is_loaded(PrimitiveContext::Math) {
    println!("El contexto Math ya está cargado");
}
```

### Obtener Lista de Contextos Cargados

```rust
let loaded = registry.loaded_contexts_list();
for context in loaded {
    println!("Contexto cargado: {}", context);
}
```

## Agregar Nuevas Primitivas

### 1. Definir la Primitiva usando Macros

```rust
// En src/runtime/primitives/math.rs
use crate::primitive;

primitive! {
    math::core_nueva_funcion(x: f64) -> f64 {
        // Implementación
        x * 2.0
    }
}
```

### 2. Registrar la Primitiva

```rust
// En la función register_math_primitives
register_context_primitives!(registrar, math, {
    core_sqrt: 1..=1,
    core_nueva_funcion: 1..=1,  // Agregar aquí
    // ... otras primitivas
});
```

### Macros Disponibles para Primitivas

El macro `primitive!` soporta varios patrones:

```rust
// Math - un argumento f64
primitive! {
    math::func(x: f64) -> f64 { x.sqrt() }
}

// Math - dos argumentos f64
primitive! {
    math::pow(base: f64, exp: f64) -> f64 { base.powf(exp) }
}

// String - argumento String -> String
primitive! {
    string::to_upper(s: String) -> String { s.to_uppercase() }
}

// String - dos argumentos String -> String
primitive! {
    string::concat(s1: String, s2: String) -> String {
        format!("{}{}", s1, s2)
    }
}

// String - argumento String -> bool
primitive! {
    string::is_empty(s: String) -> bool { s.is_empty() }
}

// String - dos argumentos String -> bool
primitive! {
    string::starts_with(s: String, prefix: String) -> bool {
        s.starts_with(&prefix)
    }
}

// String - argumento String -> i64
primitive! {
    string::len(s: String) -> i64 { s.len() as i64 }
}

// String - dos argumentos String -> i64
primitive! {
    string::index_of(s: String, sub: String) -> i64 {
        s.find(&sub).map(|i| i as i64).unwrap_or(-1)
    }
}

// System - argumento String -> void
primitive! {
    system::print(msg: String) -> () { print!("{}", msg); }
}

// System - argumento String -> String
primitive! {
    system::env_get(name: String) -> String {
        std::env::var(&name).unwrap_or_default()
    }
}

// System - sin argumentos -> f64
primitive! {
    system::random() -> f64 { /* implementación */ }
}

// Time - sin argumentos -> i64
primitive! {
    time::now() -> i64 { /* implementación */ }
}

// IO, HTTP, Array, JSON - patrones similares
```

## Agregar Nuevos Builtins

### 1. Usar Macros para Definir Builtins

```rust
// En src/runtime/builtins/global.rs
use crate::builtin_fn;
use crate::runtime::RuntimeValue;

builtin_fn! {
    mi_nuevo_builtin(args) -> RuntimeValue {
        // Implementación
        RuntimeValue::Null(NullValue::new())
    }
}
```

### 2. Registrar en el Environment

```rust
pub fn register(env: &mut Environment) {
    register_builtin!(env, "print", print_fn());
    register_builtin!(env, "mi_nuevo_builtin", mi_nuevo_builtin_fn());
}
```

### Macros Disponibles para Builtins

```rust
// Definir una función builtin
builtin_fn! {
    my_function(args) -> RuntimeValue {
        // implementación
    }
}

// Registrar un builtin
register_builtin!(env, "nombre", valor);

// Registrar múltiples builtins
builtins! {
    env => {
        "print" => print_fn(),
        "println" => println_fn(),
    }
}
```

## Estructura de Archivos

```
src/runtime/
├── primitives/
│   ├── mod.rs              # Módulo principal
│   ├── macros.rs           # Macros para primitivas
│   ├── contexts.rs         # Definición de contextos
│   ├── registry.rs         # Sistema de carga perezosa
│   ├── math.rs             # Primitivas matemáticas
│   ├── string.rs           # Primitivas de cadenas
│   ├── array.rs            # Primitivas de arreglos
│   ├── io.rs               # Primitivas de I/O
│   ├── http.rs             # Primitivas HTTP
│   ├── time.rs             # Primitivas de tiempo
│   ├── json.rs             # Primitivas JSON
│   └── system.rs           # Primitivas del sistema
└── builtins/
    ├── mod.rs              # Módulo principal
    ├── builtin_macros.rs   # Macros para builtins
    ├── global.rs           # Builtins globales
    └── ...
```

## Beneficios

1. **Rendimiento**: Solo se cargan las primitivas que se necesitan
2. **Organización**: Las primitivas están agrupadas por su propósito operacional
3. **Facilidad de uso**: Macros simplifican la adición de nuevas primitivas
4. **Mantenibilidad**: Código más limpio y fácil de mantener
5. **Flexibilidad**: Se pueden cargar contextos según las necesidades del programa

## Ejemplo Completo

```rust
use raccoon::runtime::{LazyPrimitiveRegistry, PrimitiveContext, Registrar};
use std::sync::{Arc, Mutex};

// Crear registrar y registry
let registrar = Arc::new(Mutex::new(Registrar::new()));
let registry = LazyPrimitiveRegistry::new(registrar.clone());

// Cargar solo contextos necesarios
registry.load_context(PrimitiveContext::Math);
registry.load_context(PrimitiveContext::String);

// Verificar carga
println!("Contextos cargados: {:?}", registry.loaded_contexts_list());

// Las primitivas matemáticas y de string ahora están disponibles
// en el registrar para su uso
```

## Migración desde el Sistema Anterior

El sistema anterior de `register_core_primitives()` aún funciona pero ya no es necesario. Para migrar:

1. En lugar de llamar `register_core_primitives(&mut registrar)`, usa:
   ```rust
   let registry = LazyPrimitiveRegistry::new(registrar);
   registry.load_all(); // o load_context para contextos específicos
   ```

2. Los builtins siguen funcionando igual con `setup_builtins(env)`, pero ahora puedes usar las nuevas macros para agregar más fácilmente.

## Notas

- Los builtins siempre están disponibles y no requieren carga perezosa
- El contexto se carga solo una vez, llamadas subsecuentes no hacen nada
- Es seguro usar desde múltiples hilos (thread-safe)
