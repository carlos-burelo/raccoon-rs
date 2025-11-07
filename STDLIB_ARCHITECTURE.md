# Arquitectura del Sistema de Tipos y Stdlib

## Principio Fundamental

**Regla de Oro**: Solo implementa en Rust lo que **no se puede** hacer en Raccoon. Todo lo demás debe estar en el stdlib (.rcc).

## Capas del Sistema

### Capa 1: Primitivas en Rust (src/runtime/natives/*.rs)

Operaciones que REQUIEREN implementación en Rust:
- Acceso directo a memoria/estructuras de datos
- Operaciones matemáticas complejas (trigonometría, raíces, etc.)
- Operaciones que requieren librerías del sistema
- Operaciones de performance crítica

**Ejemplos:**
```rust
// ✅ CORRECTO: Operación imposible en Raccoon
registrar.register_fn("upper", Some("string"), |args| {
    let s = String::from_raccoon(&args[0]).unwrap_or_default();
    s.to_uppercase().to_raccoon()
}, 1, Some(1));

// ✅ CORRECTO: Requiere libm
registrar.register_fn("sqrt", Some("math"), |args| {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    x.sqrt().to_raccoon()
}, 1, Some(1));
```

### Capa 2: Stdlib en Raccoon (stdlib/*.rcc)

Funciones de alto nivel construidas usando las primitivas:

#### ❌ INCORRECTO - Wrappers Inútiles
```typescript
// NO HAGAS ESTO
export fn upper(s: str): str {
    return native_str_upper(s);  // ← Sin valor agregado
}
```

#### ✅ CORRECTO - Funciones Útiles
```typescript
// HAZ ESTO
export fn capitalize(s: str): str {
    if (string.length(s) == 0) return s;
    return string.upper(string.char_at(s, 0)) +
           string.lower(string.substring(s, 1, string.length(s)));
}

export fn title_case(s: str): str {
    let words = string.split(s, " ");
    let result: str[] = [];
    for (let word in words) {
        array.push(result, capitalize(word));
    }
    return array.join(result, " ");
}
```

## Primitivas Mínimas por Módulo

### String (Rust)
- `length(s)` - acceso a longitud
- `char_at(s, index)` - acceso a caracter
- `substring(s, start, end)` - slice de memoria
- `upper(s)` / `lower(s)` - transformación Unicode
- `split(s, delim)` - crear array
- `replace(s, from, to)` - búsqueda/reemplazo
- `trim(s)` - manipulación de whitespace
- `index_of(s, needle)` - búsqueda
- `starts_with(s, prefix)` / `ends_with(s, suffix)` - comparación

### Array (Rust)
- `length(arr)` - acceso a longitud
- `push(arr, item)` / `pop(arr)` - manipulación de memoria
- `shift(arr)` / `unshift(arr, item)` - manipulación de memoria
- `slice(arr, start, end)` - slice de memoria
- `reverse(arr)` / `sort(arr)` - operaciones in-place

### Math (Rust)
- `sqrt(x)`, `pow(base, exp)` - requieren libm
- `sin(x)`, `cos(x)`, `tan(x)` - requieren libm
- `random()` - requiere RNG del sistema

## Ejemplos de Funciones por Capa

| Función | Implementación | Razón |
|---------|---------------|-------|
| `string.upper()` | ✅ Rust | Unicode, transformación compleja |
| `string.capitalize()` | ✅ Raccoon | Combina upper + lower + substring |
| `string.is_empty()` | ✅ Raccoon | Solo checa length == 0 |
| `string.truncate()` | ✅ Raccoon | Usa substring + length |
| `array.map()` | ✅ Raccoon | Loop + push + callbacks |
| `array.filter()` | ✅ Raccoon | Loop + push + predicado |
| `array.push()` | ✅ Rust | Manipulación de memoria |
| `math.abs()` | ✅ Raccoon | Solo checa signo |
| `math.clamp()` | ✅ Raccoon | Comparaciones simples |
| `math.sqrt()` | ✅ Rust | Requiere libm |
| `math.factorial()` | ✅ Raccoon | Recursión simple |
| `math.isPrime()` | ✅ Raccoon | Algoritmo en loop |

## Ventajas de Esta Arquitectura

1. **Menos código Rust**: Más fácil de mantener
2. **Más flexible**: Usuarios pueden extender stdlib en Raccoon
3. **Mejor testing**: Código Raccoon es más fácil de probar
4. **Documentación implícita**: El código Raccoon es autoexplicativo
5. **Menos recompilaciones**: Cambios en stdlib no requieren recompilar Rust

## Antipatrones a Evitar

### ❌ Duplicación de Sistemas
```rust
// NO tengas múltiples sistemas de registro
NativeRegistry   // legacy
Registrar       // moderno ← usa solo este
PluginRegistry  // deprecated
```

### ❌ Triple Implementación
```
string.upper() en:
├─ src/runtime/natives/string.rs     ← OK
├─ src/runtime/stdlib_natives.rs     ← ELIMINAR
└─ stdlib/string.rcc                 ← ELIMINAR wrapper
```

### ❌ Wrappers Sin Valor
```typescript
// NO HAGAS ESTO
export class String {
    static upper(s: str): str {
        return native_str_upper(s);  // Inútil
    }
}
```

## Uso desde Código Raccoon

### Llamando Primitivas Directamente
```typescript
// Las primitivas se exponen en su namespace
let s = "hello";
let upper = string.upper(s);        // ← Llama directamente a Rust
let len = string.length(s);         // ← Llama directamente a Rust
```

### Usando Funciones de Stdlib
```typescript
// Las funciones de stdlib agregan funcionalidad
import { capitalize, title_case } from "string";

let name = "john doe";
let proper = capitalize(name);      // "John doe"
let title = title_case(name);       // "John Doe"
```

## Migración del Sistema Legacy

### Paso 1: Eliminar stdlib_natives.rs
- Todas las funciones `native_*` deberían estar en `natives/*.rs`
- Usar sistema de `Registrar` uniformemente

### Paso 2: Limpiar Stdlib
- Eliminar wrappers inútiles de .rcc
- Mantener solo funciones que agregan valor

### Paso 3: Actualizar Tests
- Probar que primitivas funcionan directamente
- Probar que funciones stdlib usan primitivas correctamente

## Referencias

- Sistema Registrar: `src/runtime/registrar.rs`
- Module Registry: `src/runtime/module_registry.rs`
- Natives: `src/runtime/natives/*.rs`
- Stdlib: `stdlib/*.rcc`
