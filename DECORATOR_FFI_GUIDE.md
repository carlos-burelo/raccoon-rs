# Guía de Decoradores y FFI en Raccoon

## 📋 Índice
1. [Introducción](#introducción)
2. [Decoradores Disponibles](#decoradores-disponibles)
3. [Sistema FFI](#sistema-ffi)
4. [Ejemplos](#ejemplos)
5. [Migración de Código](#migración-de-código)

---

## Introducción

El sistema de decoradores de Raccoon permite:

✅ **Registrar funciones dinámicamente** en el FFI Registry
✅ **Agregar metadatos** (cache, validación, deprecation)
✅ **Separar lógica** entre stdlib (interna) y código usuario (público)
✅ **Hacer el runtime completamente extensible**

Los decoradores se dividen en:
- **Decoradores Internos** (prefijo `_`): Solo stdlib puede usarlos
- **Decoradores Públicos**: Usuarios pueden usarlos libremente

---

## Decoradores Disponibles

### Decoradores Internos (Stdlib Only)

#### `@_ffi()`
Registra una función en el FFI Registry para invocación dinámica.

```raccoon
@_ffi()
export fn readFile(path: str): str {
    return internal_read_file(path);
}
```

**Uso interno:**
- Solo en stdlib
- Permite que otras funciones llamen esta vía FFI
- Metadatos automáticos: parámetros, tipos, return type

---

#### `@_register(namespace: str)`
Registra una función en un namespace específico.

```raccoon
// stdlib/math.rcc
@_register("Math")
@_ffi()
export fn sqrt(x: float): float {
    return native_sqrt(x);
}

// Ahora accesible como:
// Math.sqrt(16.0) → 4.0
// También como sqrt(16.0) sin namespace
```

---

#### `@_validate()`
Habilita validación automática de tipos en parámetros y return.

```raccoon
@_validate()
@_ffi()
export fn divide(a: float, b: float): float {
    if (b == 0.0) {
        throw new Error("Division by zero");
    }
    return a / b;
}
```

---

### Decoradores Públicos (Users & Stdlib)

#### `@cache(ttl_ms: int)`
Cachea resultados de función por N milisegundos.

```raccoon
@cache(300000)  // 5 minutos
export fn getSystemInfo(): any {
    return native_system_info();
}

// Segunda llamada en menos de 5min devuelve cached
// Después de 5min, se ejecuta de nuevo
```

**Casos de uso:**
- Operaciones I/O costosas
- Cálculos complejos
- Llamadas a APIs externas

---

#### `@deprecated(message: str)`
Marca función como deprecated con mensaje opcional.

```raccoon
@deprecated("Use fetch() instead of oldRequest()")
export fn oldRequest(url: str): any {
    return fetch(url);
}

// Runtime emitirá warning si se usa
```

---

#### `@pure()`
Marca función como pura (sin side effects).

```raccoon
@pure()
export fn add(a: int, b: int): int {
    return a + b;
}

// Hints para optimizaciones:
// - Memoization
// - Constant folding
// - Dead code elimination
```

---

#### `@inline()`
Sugiere al compilador hacer inline de esta función.

```raccoon
@inline()
export fn min(a: int, b: int): int {
    return a < b ? a : b;
}

// Compilador puede reemplazar llamadas con:
// let x = a < b ? a : b;
```

---

#### `@readonly()`
Marca propiedad de clase como solo lectura.

```raccoon
class User {
    @readonly()
    id: str;

    name: str;

    constructor(id: str, name: str) {
        this.id = id;    // OK en constructor
        this.name = name;
    }
}

let user = new User("123", "John");
user.name = "Jane";  // OK
user.id = "456";     // ERROR: readonly property
```

---

#### `@override()`
Marca método como override de clase base.

```raccoon
class Animal {
    speak(): str {
        return "Some sound";
    }
}

class Dog extends Animal {
    @override()
    speak(): str {
        return "Woof!";
    }
}
```

---

## Sistema FFI

### Registrando Funciones (Interno)

Durante la ejecución de stdlib, funciones con `@_ffi()` se registran automáticamente:

```raccoon
// stdlib/io.rcc
@_ffi()
export fn readFile(path: str): str {
    return internal_read_file(path);
}

@_ffi()
@cache(60000)
export fn readDir(path: str): str[] {
    return internal_read_dir(path);
}

// Al ejecutar stdlib/io.rcc:
// 1. readFile se registra en FFIRegistry("readFile")
// 2. readDir se registra con metadatos de cache
// 3. Ambas accesibles vía FFI.call() si se necesita
```

### Accediendo FFI Registry

```rust
// Desde Rust (interno)
let ffi = interpreter.get_ffi_registry();
let result = ffi.call_function("readFile", args)?;
```

---

## Ejemplos

### Ejemplo 1: Función Simple con Cache

```raccoon
// stdlib/system.rcc
@_ffi()
@cache(300000)
export fn getOsInfo(): any {
    return {
        os: getOS(),
        timestamp: now()
    };
}
```

### Ejemplo 2: Operaciones Matemáticas Puras

```raccoon
// stdlib/math.rcc
@_register("Math")
@_ffi()
@pure()
export fn abs(x: float): float {
    return x < 0.0 ? -x : x;
}

@_register("Math")
@_ffi()
@pure()
@inline()
export fn min(a: int, b: int): int {
    return a < b ? a : b;
}
```

### Ejemplo 3: Validación Automática

```raccoon
// stdlib/validation.rcc
@_ffi()
@_validate()
export fn parseInt(str: str): int {
    // Convertir string a int
    let result = 0;
    for (let i = 0; i < str.length(); i = i + 1) {
        let char = str[i];
        if (char >= "0" && char <= "9") {
            result = result * 10 + (int)(char[0] - "0"[0]);
        }
    }
    return result;
}
```

### Ejemplo 4: Usuario usando Decoradores Públicos

```raccoon
// user_code.rcc
@cache(10000)
fn fibonacci(n: int): int {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

@deprecated("Use fibonacci() instead")
fn fib(n: int): int {
    return fibonacci(n);
}

@pure()
fn multiply(a: float, b: float): float {
    return a * b;
}
```

---

## Migración de Código

### Antes (Con @native hack)

```raccoon
// stdlib/io.rcc - ANTERIOR
@native("native_io_read_file")
declare fn _read_file_native(path: str): str;

export fn readFile(path: str): str {
    return _read_file_native(path);
}
```

**Problemas:**
- ❌ Hardcodeado en Rust
- ❌ Require recompilar
- ❌ Sin metadatos
- ❌ No extensible

---

### Después (Con @_ffi())

```raccoon
// stdlib/io.rcc - NUEVO
@_ffi()
export fn readFile(path: str): str {
    // Implementación puede estar aquí o ser delegada a native
    return internal_read_file(path);
}

@_ffi()
@cache(60000)
export fn readDir(path: str): str[] {
    return internal_read_dir(path);
}
```

**Ventajas:**
- ✅ Definido en Raccoon
- ✅ Sin recompilar
- ✅ Metadatos (cache)
- ✅ Totalmente extensible
- ✅ Separación clara: interno vs público

---

## Guía de Migración Paso a Paso

### Paso 1: Reemplazar @native con @_ffi()

```raccoon
// Antes
@native("native_io_read_file")
declare fn _read_file_native(path: str): str;

// Después
@_ffi()
export fn readFile(path: str): str {
    return _read_file_native(path);  // Aún existe mientras se migra
}
```

### Paso 2: Agregar Metadatos

```raccoon
// Agregar cache donde tenga sentido
@_ffi()
@cache(60000)
export fn readDir(path: str): str[] {
    return _read_dir_native(path);
}
```

### Paso 3: Eliminar @native gradualmente

Una vez todo usa @_ffi(), eliminar @native y las funciones `_*_native`:

```raccoon
// Final
@_ffi()
export fn readFile(path: str): str {
    return internal_read_file(path);  // Implementación real
}
```

---

## Reglas de Oro

✅ **Usa `@_ffi()`** en stdlib para registrar funciones dinámicamente
✅ **Usa `@cache()`** para operaciones costosas
✅ **Usa `@deprecated()`** para funciones viejas que aún soportas
✅ **Usa `@pure()`** en funciones matemáticas simples
✅ **Usa `@readonly()`** en propiedades que no deben cambiar

❌ **No uses `@_ffi()`** en código de usuario (solo stdlib)
❌ **No confundas** decoradores internos con públicos
❌ **No agregues** decoradores sin razón (mantenibilidad)

---

## Próximos Pasos

1. ✅ Sistema base implementado
2. ⏳ Migrar stdlib completo a usar @_ffi()
3. ⏳ Eliminar NativeBridge (800+ líneas)
4. ⏳ Permitir usuarios registrar funciones FFI (avanzado)
5. ⏳ Soporte para async en FFI

---
