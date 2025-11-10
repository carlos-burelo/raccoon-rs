# Sistema de Primitivas Raccoon

## Filosofía de Diseño

El sistema de primitivas de Raccoon sigue el principio **"Primitivas en Rust, Lógica en Raccoon"**:

- **Rust**: Solo operaciones de bajo nivel que DEBEN estar en Rust (matemáticas nativas, I/O del sistema, HTTP, etc.)
- **Raccoon**: Toda la lógica de alto nivel, algoritmos complejos, y funcionalidades compuestas

## Arquitectura

```
src/runtime/natives/
├── macros.rs           # Sistema de macros para definir primitivas
├── primitives.rs       # SOLO funciones primitivas atómicas
└── mod.rs              # Exportaciones

stdlib/
├── math.rcc            # Lógica matemática en Raccoon
├── string.rcc          # Manipulación de strings
├── array.rcc           # Operaciones de arrays
├── io.rcc              # Sistema de archivos
├── http.rcc            # Cliente HTTP
└── ...                 # Más módulos
```

## Usando Primitivas desde Raccoon

### 1. Importar desde `std:runtime`

Las primitivas están disponibles en el módulo especial `std:runtime`:

```raccoon
import { sqrt, sin, cos, pow } from "std:runtime";

let x = sqrt(16.0);  // 4.0
let y = sin(3.14159 / 2);  // 1.0
```

### 2. Ejemplo: stdlib/math.rcc

```raccoon
// Importar primitivas
import {
    sqrt as core_sqrt,
    sin as core_sin,
    pow as core_pow
} from "std:runtime";

class Math {
    static PI = 3.141592653589793;

    // Delegar a primitiva
    static sqrt(x: float): float {
        return core_sqrt(x);
    }

    // Lógica en Raccoon usando primitivas
    static pow(base: float, exp: float): float {
        if (exp == 0.0) return 1.0;
        if (base == 0.0) return 0.0;

        // Usar primitivas: exp(exp * ln(base))
        return core_exp(exp * core_ln(base));
    }

    // Lógica pura en Raccoon (sin primitivas)
    static factorial(n: int): int {
        if (n <= 1) return 1;
        let result = 1;
        for (let i = 2; i <= n; i++) {
            result *= i;
        }
        return result;
    }
}
```

## Primitivas Disponibles

### Matemáticas (`core_*`)
- `sqrt`, `cbrt`, `pow`
- `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
- `sinh`, `cosh`, `tanh`
- `exp`, `ln`, `log10`, `log`
- `floor`, `ceil`, `round`, `trunc`
- `abs`, `sign`

### I/O de Archivos (`core_file_*`, `core_dir_*`)
- `file_read(path: string): string`
- `file_write(path: string, content: string): bool`
- `file_append(path: string, content: string): bool`
- `file_exists(path: string): bool`
- `file_delete(path: string): bool`
- `dir_create(path: string): bool`
- `dir_list(path: string): string` (JSON array)

### HTTP (`core_http_*`)
- `http_get(url: string): string`
- `http_post(url: string, body: string): string`
- `http_request(method: string, url: string, body: string, headers: string): string`

### Tiempo (`core_time_*`)
- `time_now(): int` (milisegundos desde epoch)
- `time_now_micros(): int`
- `sleep(ms: int): void`

### Strings (`core_string_*`)
- `string_len(s: string): int`
- `string_char_at(s: string, index: int): string`
- `string_substring(s: string, start: int, end: int): string`
- `string_to_upper(s: string): string`
- `string_to_lower(s: string): string`
- `string_trim(s: string): string`
- `string_split(s: string, delimiter: string): string` (JSON)
- `string_replace(s: string, from: string, to: string): string`
- `string_starts_with(s: string, prefix: string): bool`
- `string_ends_with(s: string, suffix: string): bool`
- `string_contains(s: string, substring: string): bool`
- `string_index_of(s: string, substring: string): int`

### Arrays (`core_array_*`)
- `array_join(array: string, separator: string): string`
- `array_sort(array: string): string` (JSON)
- `array_reverse(array: string): string` (JSON)

### JSON (`core_json_*`)
- `json_parse(json: string): string`
- `json_stringify(value: string): string`

### Sistema (`core_*`)
- `print(message: string): void`
- `println(message: string): void`
- `env_get(name: string): string`
- `env_set(name: string, value: string): bool`
- `exit(code: int): void`
- `random(): float` (0.0 a 1.0)

## Agregando Nuevas Primitivas

### 1. Definir la función en `primitives.rs`

```rust
/// Documentación de la primitiva
pub fn core_nueva_funcion(args: Vec<RuntimeValue>) -> RuntimeValue {
    let x = f64::from_raccoon(&args[0]).unwrap_or(0.0);
    let resultado = x.alguna_operacion();
    resultado.to_raccoon()
}
```

### 2. Registrarla en `register_core_primitives()`

```rust
registrar.register_fn("core_nueva_funcion", None, core_nueva_funcion, 1, Some(1));
```

### 3. Usarla desde Raccoon

```raccoon
import { nueva_funcion } from "std:runtime";

let resultado = nueva_funcion(42.0);
```

## Mejores Prácticas

1. **Primitivas Atómicas**: Las funciones en Rust deben ser operaciones atómicas simples
2. **Sin Lógica de Negocio**: La lógica compleja va en archivos .rcc
3. **Documentación**: Cada primitiva debe estar documentada
4. **Nomenclatura**: Usar prefijo `core_` para todas las primitivas
5. **Tipos Claros**: Usar FromRaccoon/ToRaccoon para conversión de tipos

## Ventajas de Esta Arquitectura

✅ **Separación clara**: Rust para performance, Raccoon para lógica
✅ **Mantenibilidad**: Fácil modificar lógica sin recompilar Rust
✅ **Testeable**: Código Raccoon es más fácil de testear
✅ **Flexible**: Agregar features sin tocar el runtime
✅ **Performance**: Operaciones críticas en Rust nativo

## Ejemplo Completo: Módulo de I/O

```raccoon
// stdlib/io.rcc
import {
    file_read,
    file_write,
    file_exists,
    dir_create,
    dir_list
} from "std:runtime";

class IO {
    // Wrapper simple
    static readFile(path: string): string {
        return file_read(path);
    }

    // Lógica adicional en Raccoon
    static readLines(path: string): string[] {
        let content = file_read(path);
        return content.split("\n");
    }

    // Operación compleja usando primitivas
    static copyFile(src: string, dest: string): bool {
        if (!file_exists(src)) {
            return false;
        }

        let content = file_read(src);
        return file_write(dest, content);
    }

    // Lógica recursiva en Raccoon
    static ensureDir(path: string): bool {
        if (file_exists(path)) {
            return true;
        }

        // Crear directorios padres si es necesario
        // ... lógica compleja aquí ...

        return dir_create(path);
    }
}

export default IO;
```

## Conclusión

El sistema de primitivas de Raccoon proporciona una base sólida para construir funcionalidades complejas manteniendo el rendimiento nativo donde importa, mientras que mantiene la flexibilidad y facilidad de desarrollo en el lenguaje Raccoon.
