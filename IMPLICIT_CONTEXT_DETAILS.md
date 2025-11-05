# Contexto Implícito en Decoradores - Detalles Técnicos

## Variables Implícitas Dentro de un Decorador

Cuando un decorador se ejecuta, tiene acceso automático a 2 variables especiales:

### 1. `this`
**La entidad siendo decorada**, en su forma runtime.

```rust
// En pseudocódigo Rust que ocurre cuando ejecutas decorador
let this = /* RuntimeValue de la entidad decorada */;
let context = create_context_object(target_type, name);

// Luego ejecutar el cuerpo del decorador con estas variables
```

#### Tipo de `this` por Contexto

| Contexto | Tipo de `this` | Ejemplo |
|----------|---|---|
| **Function** | `RuntimeValue::Function` | La función original |
| **Async Function** | `RuntimeValue::Function` | La función async original |
| **Method** | `RuntimeValue::Function` | El método (es función internamente) |
| **Class** | `RuntimeValue::Class` | La clase original |
| **Property** | El tipo del valor | `int`, `str`, `obj`, etc. |
| **Class Property** | El tipo del valor | El valor inicial |

#### Ejemplos de Uso

```raccoon
// Decorador para funciones - this es función
decorator log {
    return fn(...args) {
        print("Calling " + context.name)
        return this(...args)  // 'this' es una función, se invoca
    }
}

// Decorador para propiedades - this es el valor
decorator readonly {
    let value = this  // 'this' es el valor (int, str, etc.)
    return {
        get: fn() { return value },
        set: fn(v) { throw "Cannot set readonly property" }
    }
}

// Decorador para clases - this es la clase
decorator singleton {
    return class extends this {  // 'this' es una clase, se hereda
        static property instance = null
    }
}
```

---

### 2. `context`
**Metadata sobre qué estoy decorando**

Es un objeto con propiedades:

```rust
pub struct DecoratorContext {
    pub context_type: String,     // "function", "method", "class", etc.
    pub name: String,              // Nombre de la entidad
    pub target: DecoratorTarget,   // Enum con el tipo
    pub node: ASTNode,             // El nodo AST original (si lo necesitas)
}

// En Raccoon se ve como:
{
    type: "function",
    name: "myFunc",
    target: <internal>  // Detalles si lo necesitas
}
```

#### Propiedades de `context`

**`context.type` (string)**: El tipo de entidad siendo decorada.

```raccoon
match context.type {
    "function" => { /* función regular */ }
    "async_function" => { /* función async */ }
    "method" => { /* método de clase */ }
    "class" => { /* clase */ }
    "property" => { /* propiedad de clase */ }
    "accessor" => { /* getter/setter */ }
    _ => { /* unknown */ }
}
```

**`context.name` (string)**: El nombre de la entidad.

```raccoon
decorator log {
    return fn(...args) {
        print("Function " + context.name + " was called")
        return this(...args)
    }
}

// Cuando decorador ejecuta:
// context.name = "fibonacci" (si decoró @log fn fibonacci() { } )
// context.name = "getData" (si decoró @log method getData() { } )
```

**`context.target`**: Información de bajo nivel (quizás no lo necesites).

En el futuro podría haber más propiedades como:
- `context.isAsync`: boolean
- `context.isStatic`: boolean (para métodos)
- `context.accessModifier`: "public" | "private" | "protected"
- Etc.

---

## Flujo de Ejecución Completo

### Paso 1: Cargar Definición

```
Archivo: stdlib/decorators.rcc

  decorator cache(ms: int = 5000) {
      let store = {}
      return fn(...args) { ... }
  }

Parser → DecoratorDefinition { name: "cache", parameters: [...], body: [...] }
         ↓
DecoratorRegistry.register_decorator(def)
         ↓
Se almacena en registry para usar después
```

### Paso 2: Usar Decorador

```
Archivo: main.rcc

  @cache(3000)
  fn fibonacci(n: int): int { ... }

Parser → FnDecl {
    name: "fibonacci",
    decorators: [
        DecoratorDecl { name: "cache", args: [IntLiteral(3000)] }
    ],
    body: [...]
}
```

### Paso 3: Ejecutar (Interpreter)

```
execute_fn_decl(fibonacci_decl):
  1. Crear RuntimeValue::Function para fibonacci
     │
  2. Para cada decorador en decorators:
     │
     a. Buscar definición de "cache" en registry
     │
     b. Crear environment nuevo ("frame"):
     │  ├─ this = RuntimeValue::Function (fibonacci original)
     │  ├─ context = {
     │  │    type: "function",
     │  │    name: "fibonacci",
     │  │    target: Function,
     │  │    ...
     │  │  }
     │  └─ ms = 3000  (evaluado de IntLiteral(3000))
     │
     c. Ejecutar cuerpo del decorador en ese environment:
     │  return fn(...args) {
     │      let key = stringify(args)
     │      if (store.has(key)) return store.get(key)
     │      let result = this(...args)        // Llama fibonacci original
     │      store.set(key, result)
     │      setTimeout(() => store.delete(key), 3000)
     │      return result
     │  }
     │
     d. El resultado es: RuntimeValue::Function (nueva función decorada)
     │
     e. Reemplazar: fibonacci = función decorada
     │
  3. Registrar fibonacci (ahora decorada) en environment
```

### Paso 4: Ejecutar (Runtime)

```
let result = fibonacci(5)

Internamente:
  1. Lookup fibonacci en environment → RuntimeValue::Function (decorada)
  2. Ejecutar función decorada:
     a. Evalúa key = stringify([5])
     b. Busca en store → no existe
     c. Llama this(5) → llama fibonacci original
     d. Guarda resultado en store
     e. Retorna resultado
  3. Próxima llamada fibonacci(5):
     a. Evalúa key = stringify([5])
     b. Busca en store → EXISTE! Retorna directamente
     c. Nunca ejecuta fibonacci original
```

---

## Manejo de Parámetros del Decorador

### Evaluación de Argumentos

Los argumentos del decorador se evalúan **antes** de crear el environment:

```raccoon
@cache(5 + 5, is_enabled())
fn myFunc() { }

// Paso 1: Evaluar argumentos
5 + 5 = 10
is_enabled() = true  // Se ejecuta AHORA

// Paso 2: Crear environment con valores evaluados
// ms = 10
// is_enabled = true
```

### Parámetros por Defecto

```raccoon
// Definición
decorator cache(ms: int = 5000) { ... }

// Uso sin argumentos
@cache
fn a() { }
// ms = 5000 (default)

// Uso con argumentos
@cache(3000)
fn b() { }
// ms = 3000 (provided)

// Nota: Los parámetros son como en funciones normales
```

### Parámetros Tipados

```raccoon
// Tipo específico
decorator validate(schema: object) {
    return fn(...args) {
        if (!check_schema(args[0], schema)) {
            throw "Validation failed"
        }
        return this(...args)
    }
}

@validate({ type: "int", min: 0, max: 100 })
fn processPercentage(val: int) { }

// En decorador:
// schema = { type: "int", min: 0, max: 100 }
```

---

## Contextos Especiales

### Contexto: Función

```raccoon
decorator myDecorator {
    // context.type = "function"
    // this = RuntimeValue::Function

    return fn(...args) {
        // Puedo llamar this(...args)
        // Puedo acceder context.name
        // Puedo acceder context.type para condicionales
        return this(...args)
    }
}

@myDecorator
fn example() { }
```

### Contexto: Método

```raccoon
decorator myDecorator {
    // context.type = "method"
    // this = RuntimeValue::Function (el método)

    // El método ejecuta en el contexto de una instancia
    // Para acceder a la instancia, necesitas capturar 'this' de la clase

    return fn(...args) {
        // Aquí 'this' es el método original
        // Para acceder a la instancia usarías argumentos
        // Ejemplo: si el método es method getData(x) { }
        // Puedes hacer: this(x) para llamarlo
        return this(...args)
    }
}

class MyClass {
    @myDecorator
    method getData(x: int) { }
}
```

### Contexto: Clase

```raccoon
decorator myDecorator {
    // context.type = "class"
    // this = RuntimeValue::Class

    // Puedo extender o envolver la clase

    return class extends this {
        // 'this' es la clase original
        // Puedo sobrescribir métodos, agregar propiedades, etc.

        constructor(...args) {
            super(...args)
            print("Instance created from decorated class")
        }
    }
}

@myDecorator
class MyClass { }
```

### Contexto: Propiedad

```raccoon
decorator myDecorator {
    // context.type = "property"
    // this = el valor de la propiedad (int, str, obj, etc.)

    let originalValue = this

    return {
        get: fn() {
            return originalValue
        },
        set: fn(newValue) {
            originalValue = newValue
        }
    }
}

class MyClass {
    @myDecorator
    property x: int = 10
}
```

---

## Scoping y Variables del Decorador

### Variables del Decorador

Las variables declaradas en el decorador son locales al decorador:

```raccoon
decorator cache(ms: int = 5000) {
    let store = {}        // Variable DEL DECORADOR
    let calls = 0         // Variable DEL DECORADOR

    return fn(...args) {
        // Aquí puedo acceder store y calls (closure)
        calls = calls + 1
        let key = stringify(args)  // Variable DE LA FUNCIÓN
        if (store.has(key)) {
            return store.get(key)
        }
        let result = this(...args)
        store.set(key, result)
        return result
    }
}

// store y calls son PRIVADOS al decorador
// Cada función decorada con @cache tiene su propia store y calls
```

### Closure Automático

```raccoon
decorator flexible {
    let counter = 0

    return fn(...args) {
        counter = counter + 1
        print("Call #" + counter)
        return this(...args)
    }
}

@flexible
fn a() { }

@flexible
fn b() { }

a()  // Prints "Call #1"
b()  // Prints "Call #1" (su propio counter)
a()  // Prints "Call #2" (same a as before)
```

Cada decorador crea su propio closure con sus propias variables.

---

## Manejo de Errores

### Errores en Decorador

Si el decorador lanza un error:

```raccoon
decorator bad {
    throw "Decorador error"
}

@bad
fn myFunc() { }
```

El programa falla al definir `myFunc`, no al llamarlo.

### Validación en Decorador

```raccoon
decorator validate(schema: object) {
    // Validar que schema sea válido
    if (!schema || schema.type == null) {
        throw "Invalid schema for @validate"
    }

    return fn(...args) {
        if (!check_schema(args[0], schema)) {
            throw "Validation failed"
        }
        return this(...args)
    }
}
```

El primer error (validación de schema) ocurre en tiempo de definición.
El segundo error (validación de argumentos) ocurre en tiempo de ejecución.

---

## Decoradores Que No Retornan Función

### Para Clases

```raccoon
decorator observable {
    return class extends this {
        // Retorna una clase, no una función
    }
}

@observable
class MyClass { }
```

### Para Propiedades (getter/setter)

```raccoon
decorator readonly {
    let value = this

    return {
        get: fn() { return value },
        set: fn(v) { throw "readonly" }
    }
}

@readonly
property x: int = 10
```

### Caso Especial: Decorador que No Cambia Nada

```raccoon
decorator noop {
    // Simplemente retorna this sin cambios
    return this
}

@noop
fn whatever() { }

// whatever = whatever (sin cambios)
```

---

## Ejemplo Complejo: Decorador Multi-Contexto

```raccoon
decorator smart(mode: str = "auto") {
    // Usa match como expresión
    return match context.type {
        "function" => {
            // Retorna función
            fn(...args) {
                print("[fn] " + context.name)
                return this(...args)
            }
        },
        "class" => {
            // Retorna clase
            class extends this {
                constructor(...args) {
                    super(...args)
                    print("[class] " + context.name)
                }
            }
        },
        "method" => {
            // Retorna función
            fn(...args) {
                print("[method] " + context.name)
                return this(...args)
            }
        },
        "property" => {
            // Retorna la propiedad sin cambios
            this
        },
        _ => {
            // Default
            this
        }
    }
}

@smart("verbose")
fn funcA() { }

@smart("verbose")
class ClassB { }

class Holder {
    @smart("verbose")
    method methodC() { }

    @smart("verbose")
    property x: int = 10
}
```

---

## Performance y Consideraciones

### Variables en Closure
```raccoon
decorator cache(ms: int) {
    let store = {}  // Se crea UNA VEZ cuando se define el decorador

    return fn(...args) {
        // store se reutiliza en cada llamada
    }
}
```

El `store` se crea una vez, no en cada llamada. Es eficiente.

### Múltiples Decoradores
```raccoon
@decorator1
@decorator2
@decorator3
fn myFunc() { }

// Se aplican en orden:
// 1. myFunc original
// 2. Envuelta por @decorator3 → fn3
// 3. Envuelta por @decorator2(fn3) → fn2
// 4. Envuelta por @decorator1(fn2) → fn1
// 5. Se registra fn1

// Al llamar myFunc() → ejecuta fn1 → fn2 → fn3 → original
```

El orden importa.

---

## Integración con Otras Features

### Con Pattern Matching
```raccoon
decorator smart {
    return match context.type {
        "function" => wrapper_fn(),
        "class" => wrapper_class(),
        _ => this
    }
}
```

### Con Funciones Anónimas
```raccoon
decorator cache(ms: int) {
    return fn(...args) {  // Función anónima
        // ...
    }
}
```

### Con Clases Anónimas
```raccoon
decorator proxy {
    return class extends this {  // Clase anónima
        // ...
    }
}
```

Todas las features trabajan juntas naturalmente.

---

## Checklist para Implementar Contexto Implícito

- [ ] Crear helper `create_context_object(target_type, name)` en interpreter
- [ ] En `apply_decorators()`, crear environment con `this` y `context`
- [ ] Asegurar que los parámetros del decorador estén disponibles
- [ ] Ejecutar el cuerpo del decorador en ese environment
- [ ] Capturar el valor de retorno
- [ ] Reemplazar la entidad original con la decorada
- [ ] Tests para cada tipo de contexto (function, method, class, property)
- [ ] Tests para multi-contexto (mismo decorador en diferentes tipos)
- [ ] Tests para closures (variables del decorador)
