# Quick Reference: Decoradores Definibles por Usuario

Resumen ejecutivo para desarrolladores.

## Objetivo

Implementar decoradores como feature first-class, completamente definibles en Raccoon.

**De:**
```rust
// hardcoded en Rust
impl DecoratorRegistry {
    fn register_all_decorators(&mut self) {
        self.register_decorator(DecoratorSpec {
            name: "@cache".to_string(),
            // ...
        });
    }
}
```

**A:**
```raccoon
// definible en Raccoon
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
```

## Sintaxis Final

### Definir un Decorador

```raccoon
decorator <name>(<params>) {
    // Acceso implícito a:
    // - this: la entidad siendo decorada
    // - context: { type, name, target }
    return <entidad_decorada>
}
```

### Usar un Decorador

```raccoon
@<name>(<args>)
fn/class/method/property ...
```

### Ejemplos

```raccoon
// Simple
@cache(5000)
fn expensive() { }

// Con match (patrón matching)
decorator log {
    return match context.type {
        "function" => fn(...args) { print(context.name); return this(...args) },
        "class" => this,
        _ => this
    }
}

// Retorna función anónima
decorator timing {
    return fn(...args) {
        let start = now()
        let result = this(...args)
        print(context.name + " took " + (now() - start) + "ms")
        return result
    }
}

// Retorna clase anónima
decorator observable {
    return class extends this {
        method notifyObservers() { /* ... */ }
    }
}
```

## Prerrequisitos: 3 Features

| # | Feature | Status | Tiempo | Notas |
|---|---------|--------|--------|-------|
| 1 | Funciones Anónimas | ✅ DONE | 2-3h | `fn(params) { stmts }` con return explícito |
| 2 | Pattern Matching | TODO | 4-6h | `match expr { pattern => expr }` |
| 3 | Clases Anónimas | TODO | 2-3h | `class { ... }` sin nombre |

## Plan de Implementación

```
1. Funciones Anónimas (2-3h) ✅ COMPLETADO
   ├─ Parser: fn(params) { stmts }
   ├─ Paréntesis requeridos incluso sin argumentos
   └─ Return explícito (no arrow syntax)

2. Pattern Matching (4-6h)
   ├─ Parser: match expr { arms }
   ├─ AST: MatchExpr, Pattern
   └─ Interpreter: evaluate_match_expr(), matches_pattern()

3. Clases Anónimas (2-3h)
   ├─ Parser: class sin nombre
   ├─ AST: ClassLiteral
   └─ Interpreter: evaluate_class_literal()

4. Decoradores (3-4h)
   ├─ AST: DecoratorDefinition
   ├─ Parser: decorator keyword
   ├─ Interpreter: execute_decorator_def(), apply_decorators()
   ├─ Runtime: refactorizar DecoratorRegistry
   └─ Stdlib: crear stdlib/decorators.rcc

TOTAL: 11-16 horas
```

## Variables Implícitas

### `this`
La entidad siendo decorada.

| Contexto | Tipo | Ejemplo |
|----------|------|---------|
| function | Function | Llamas: `this(...args)` |
| method | Function | Llamas: `this(...args)` |
| class | Class | Heredas: `class extends this` |
| property | cualquier | El valor: int, str, obj, etc. |

### `context`
Metadata sobre la entidad.

```raccoon
{
    type: "function" | "method" | "class" | "property" | ...,
    name: "nombreEntidad",
    target: <internal>
}
```

## Cambios en Código

### src/parser/mod.rs
```rust
+ parse_anonymous_fn()
+ parse_match_expr()
+ parse_decorator_def()
+ parse_pattern()
+ Actualizar parse_primary_expr()
```

### src/ast/nodes.rs
```rust
+ Stmt::DecoratorDef(DecoratorDefinition)
+ Expr::Match(MatchExpr)
+ Expr::AnonymousFn(AnonymousFnExpr)
+ Expr::ClassLiteral(ClassLiteral)
+ enum Pattern
```

### src/interpreter/mod.rs
```rust
+ evaluate_match_expr()
+ evaluate_anonymous_fn()
+ evaluate_class_literal()
+ matches_pattern()
+ create_context_object()
+ apply_decorators()
- validate_and_process_decorators()
```

### src/interpreter/declarations.rs
```rust
+ execute_decorator_def()
~ execute_fn_decl(): aplicar decoradores
~ execute_class_decl(): aplicar decoradores
- lógica hardcodeada de decoradores
```

### src/runtime/decorator_registry.rs
```rust
✎ REFACTORIZAR
- DecoratorSpec
- DecoratorVisibility
- allowed_on
+ DecoratorFunction { name, parameters, body }
```

### stdlib/decorators.rcc (NUEVO)
```raccoon
decorator deprecated(msg: str = "") { ... }
decorator cache(ms: int = 5000) { ... }
decorator log(prefix: str = "[LOG]") { ... }
decorator retry(times: int = 3, delay: int = 1000) { ... }
decorator timing { ... }
decorator memoize(ttl: int = 300000) { ... }
// Etc.
```

## Flujo de Ejecución Resumido

```
Código:
  @cache(3000)
  fn fibonacci(n: int): int { ... }

Parse Time:
  FnDecl { decorators: [@cache(3000)], body: [...] }

Define Time (Interpreter):
  1. Buscar "cache" en DecoratorRegistry
  2. Crear environment: { this: fn_value, context: {...}, ms: 3000 }
  3. Ejecutar cuerpo decorador → retorna función decorada
  4. Registrar función decorada

Call Time:
  fibonacci(5) → ejecuta versión decorada (con cache)
```

## Ejemplo: @cache

### Definición

```raccoon
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
```

### Uso

```raccoon
@cache(3000)
fn expensive(n: int): int {
    print("Computing...")
    return n * n
}

expensive(5)      // Computing... → 25 (calcula)
expensive(5)      // → 25 (del cache)
expensive(10)     // Computing... → 100 (diferente arg)
```

## Ejemplo: @log Multi-Contexto

```raccoon
decorator log(prefix: str = "[LOG]") {
    return match context.type {
        "function" => fn(...args) {
            print(prefix + " Function " + context.name + " called")
            return this(...args)
        },
        "method" => fn(...args) {
            print(prefix + " Method " + context.name + " called")
            return this(...args)
        },
        "class" => this,  // No loguear clases
        _ => this
    }
}

@log("[TRACE]")
fn process(x: int): int { ... }

class Service {
    @log("[API]")
    method getData(): str { ... }
}
```

## Tests a Implementar

### Funciones Anónimas (IMPLEMENTADO ✅)
```raccoon
// Sintaxis: fn(params) { stmts } con return explícito
let add = fn(a: int, b: int) {
    return a + b
}

let greet = fn() {
    return "Hello"
}

// Nota: Arrow functions siguen siendo válidas: fn(x) => x * 2
// Pero fn { ... } REQUIERE paréntesis y return explícito
```

### Pattern Matching
```raccoon
let msg = match 5 {
    0 => "zero",
    1..10 => "small",
    _ => "big"
}
assert(msg == "small")
```

### Clases Anónimas
```raccoon
let Point = class {
    property x: int
    property y: int
    method sum() { return this.x + this.y }
}
let p = new Point()
p.x = 3
p.y = 4
assert(p.sum() == 7)
```

### Decoradores
```raccoon
@cache(1000)
fn fib(n: int): int {
    if (n <= 1) return n
    return fib(n-1) + fib(n-2)
}
assert(fib(10) == 55)
```

## Documentos Relacionados

- **VISION_DECORADORES.md** - Visión completa y casos de uso
- **DECORATOR_DEPENDENCIES.md** - Detalles técnicos de cada feature
- **IMPLICIT_CONTEXT_DETAILS.md** - Cómo funcionan `this` y `context`
- **RESUMEN_PLAN.txt** - Timeline y checklist

## Notas Importantes

1. **Orden crítico**: Implementar en orden 1 → 2 → 3 → 4
2. **Sin boilerplate**: Decoradores son funciones normales
3. **Contexto implícito**: `this` y `context` disponibles automáticamente
4. **Multi-contexto**: Mismo decorador funciona en function/method/class/property
5. **Extensible**: Los usuarios pueden crear sus propios decoradores
6. **Sin hardcoding**: Cero implementaciones en Rust, todo en Raccoon

## FAQ

**¿Por qué necesitamos funciones anónimas?**
R: Los decoradores retornan funciones, necesitamos poder crear funciones sin nombre.

**¿Por qué pattern matching?**
R: Los decoradores necesitan tener lógica diferente para diferentes contextos.

**¿Por qué clases anónimas?**
R: Algunos decoradores extienden clases dinámicamente.

**¿El orden de decoradores importa?**
R: Sí. Se aplican secuencialmente: primero el más cercano al nombre.

```raccoon
@decorator1
@decorator2
@decorator3
fn myFunc() { }

// Se aplica: decorator3 → decorator2 → decorator1
```

**¿Puedo acceder el AST original en el decorador?**
R: El contexto tiene información, pero la idea es trabajar con `this` (el valor runtime).

**¿Los decoradores pueden ser async?**
R: Sí, el decorador puede retornar una función async.

```raccoon
decorator timeout(ms: int = 5000) {
    return async fn(...args) {
        // Logic
        return await this(...args)
    }
}
```

## Checklist de Implementación

### Fase 1: Funciones Anónimas
- [ ] Actualizar parser para distinguir `fn => expr` de `fn { stmts }`
- [ ] Agregar `AnonymousFnExpr` a AST
- [ ] Implementar `evaluate_anonymous_fn()` en interpreter
- [ ] Tests básicos

### Fase 2: Pattern Matching
- [ ] Agregar keyword `match` al lexer/parser
- [ ] Implementar parser para patterns
- [ ] Agregar `MatchExpr` y `Pattern` a AST
- [ ] Implementar `evaluate_match_expr()` y `matches_pattern()`
- [ ] Tests para cada tipo de pattern

### Fase 3: Clases Anónimas
- [ ] Actualizar parser para hacer nombre opcional
- [ ] Agregar `ClassLiteral` a AST
- [ ] Implementar `evaluate_class_literal()`
- [ ] Tests para herencia

### Fase 4: Decoradores
- [ ] Agregar keyword `decorator` al lexer
- [ ] Implementar parser para decoradores
- [ ] Agregar `DecoratorDefinition` a AST
- [ ] Refactorizar `DecoratorRegistry`
- [ ] Implementar `apply_decorators()` en interpreter
- [ ] Crear `stdlib/decorators.rcc`
- [ ] Tests para cada contexto
- [ ] Tests multi-contexto

---

**Última actualización**: 2025-11-05
**Estado**: Plan documentado, listo para implementación
