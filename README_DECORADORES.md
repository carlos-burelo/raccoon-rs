# Decoradores Definibles por Usuario - Documentación Completa

## 📑 Índice de Documentación

Esta carpeta contiene el plan completo para refactorizar decoradores en Raccoon, transformándolos de una capa de abstracción hardcodeada en Rust a una feature completamente definible y extensible en el lenguaje.

### Documentos Principales

#### 1. **QUICK_REFERENCE.md** ⭐ COMIENZA AQUÍ
**Lectura: 5-10 minutos**

Resumen ejecutivo con lo esencial:
- Objetivo y transformación
- Sintaxis final
- 3 prerrequisitos y timeline
- Cambios en cada módulo
- Checklist de implementación

👉 **Para**: Desarrolladores que quieren entender rápidamente qué hay que hacer.

---

#### 2. **VISION_DECORADORES.md**
**Lectura: 10-15 minutos**

Visión estratégica y ejemplos:
- Problema actual vs solución
- Arquitectura de alto nivel
- Flujo completo (parse → apply → runtime)
- Casos de uso reales (cache, logging, retry, etc.)
- Antes vs después

👉 **Para**: Entender POR QUÉ hacemos esto y cómo se vería el resultado.

---

#### 3. **DECORATOR_DEPENDENCIES.md**
**Lectura: 30-45 minutos**

Detalles técnicos de las 3 features prerequisito:

**1. Funciones Anónimas (2-3h)**
- Sintaxis: `fn { stmts }` vs `fn => expr`
- Cambios en parser, AST, interpreter
- Ejemplos de uso

**2. Pattern Matching (4-6h)**
- Expresión que retorna valor
- Tipos de patterns (wildcard, literal, type, range, list, object)
- Integración con guards
- Cambios técnicos

**3. Clases Anónimas (2-3h)**
- Expresión que retorna clase
- Herencia desde anónimas
- Cambios técnicos

👉 **Para**: Entender en detalle qué hay que implementar en cada feature.

---

#### 4. **IMPLICIT_CONTEXT_DETAILS.md**
**Lectura: 20-30 minutos**

Detalles técnicos de contexto implícito:
- Variable `this` por contexto (function, method, class, property)
- Variable `context` y sus propiedades
- Flujo de ejecución paso a paso
- Evaluación de argumentos
- Manejo de scoping y closures
- Ejemplos complejos
- Checklist de implementación

👉 **Para**: Entender exactamente cómo funciona `this` y `context` dentro de un decorador.

---

#### 5. **RESUMEN_PLAN.txt**
**Lectura: 15-20 minutos**

Visión ASCII con estructura del plan:
- 8 tareas ordenadas
- Cambios en cada módulo (src/parser, src/ast, src/interpreter, etc.)
- Ejemplo completo paso a paso
- Timeline visual
- Beneficios

👉 **Para**: Una vista rápida y visual del plan completo.

---

## 🎯 Cómo Usar esta Documentación

### Si Tienes 10 Minutos
1. Lee **QUICK_REFERENCE.md** (secciones: Objetivo, Sintaxis Final, Prerrequisitos, Plan)

### Si Tienes 30 Minutos
1. Lee **VISION_DECORADORES.md** completo
2. Echa un vistazo a **RESUMEN_PLAN.txt**

### Si Tienes 1 Hora
1. Lee **QUICK_REFERENCE.md** completo
2. Lee **VISION_DECORADORES.md** completo
3. Estudia los ejemplos en **IMPLICIT_CONTEXT_DETAILS.md**

### Si Vas a Implementar
1. Lee **QUICK_REFERENCE.md** para contexto general
2. Lee **DECORATOR_DEPENDENCIES.md** para cada feature que implementes
3. Consulta **IMPLICIT_CONTEXT_DETAILS.md** cuando hagas el contexto implícito
4. Usa el checklist en **QUICK_REFERENCE.md** para tracking

---

## 📊 Resumen Ejecutivo

### Situación Actual
- ❌ Decoradores hardcodeados en Rust
- ❌ Cada decorador nuevo = cambio en código Rust
- ❌ Sistema desacoplado de FFI
- ❌ No extensible por usuarios

### Situación Deseada
- ✅ Decoradores definibles en Raccoon
- ✅ Nuevo decorador = escribir función en Raccoon
- ✅ FFI integrado como decorador estándar
- ✅ Completamente extensible

### Solución
**3 features nuevas** + **4 cambios a la arquitectura** = **Decoradores user-friendly**

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

@cache(3000)
fn expensive(n: int): int { ... }
```

### Timeline
| Feature | Tiempo |
|---------|--------|
| Funciones Anónimas | 2-3h |
| Pattern Matching | 4-6h |
| Clases Anónimas | 2-3h |
| Decoradores | 3-4h |
| **TOTAL** | **11-16h** |

---

## 🔗 Relaciones entre Documentos

```
QUICK_REFERENCE.md (Inicio aquí)
├─ → VISION_DECORADORES.md (¿Por qué y cómo?)
├─ → DECORATOR_DEPENDENCIES.md (Detalles de cada feature)
├─ → IMPLICIT_CONTEXT_DETAILS.md (Cómo funciona this/context)
└─ → RESUMEN_PLAN.txt (Vista visual)
```

---

## 📝 Palabras Clave

Para búsqueda rápida:

- **Funciones Anónimas**: `AnonymousFnExpr`, `fn { stmts }`
- **Pattern Matching**: `MatchExpr`, `Pattern`, `match { arms }`
- **Clases Anónimas**: `ClassLiteral`, `class { }`
- **Decoradores**: `DecoratorDefinition`, `decorator`, `@name`
- **Contexto**: `this`, `context`, `DecoratorTarget`
- **Registry**: `DecoratorRegistry`, `DecoratorFunction`
- **Aplicación**: `apply_decorators()`, `execute_decorator_def()`

---

## 🛠️ Cambios Principales en Código

### Parser (`src/parser/mod.rs`)
```rust
+ parse_anonymous_fn()
+ parse_match_expr()
+ parse_decorator_def()
+ parse_pattern()
```

### AST (`src/ast/nodes.rs`)
```rust
+ DecoratorDefinition
+ MatchExpr, Pattern
+ AnonymousFnExpr
+ ClassLiteral
```

### Interpreter (`src/interpreter/`)
```rust
+ evaluate_match_expr()
+ evaluate_anonymous_fn()
+ evaluate_class_literal()
+ matches_pattern()
+ apply_decorators()
- validate_and_process_decorators()
```

### Runtime (`src/runtime/decorator_registry.rs`)
```rust
✎ REFACTORIZAR completamente
+ DecoratorFunction
```

### Stdlib (`stdlib/decorators.rcc`) - NUEVO
```raccoon
decorator deprecated(msg: str) { ... }
decorator cache(ms: int) { ... }
decorator log(prefix: str) { ... }
// ... más
```

---

## ✅ Checklist de Lectura

- [ ] QUICK_REFERENCE.md - Objetivo (2 min)
- [ ] QUICK_REFERENCE.md - Sintaxis Final (3 min)
- [ ] QUICK_REFERENCE.md - Prerrequisitos (2 min)
- [ ] VISION_DECORADORES.md - Completo (15 min)
- [ ] DECORATOR_DEPENDENCIES.md - Funciones Anónimas (5 min)
- [ ] DECORATOR_DEPENDENCIES.md - Pattern Matching (10 min)
- [ ] DECORATOR_DEPENDENCIES.md - Clases Anónimas (5 min)
- [ ] IMPLICIT_CONTEXT_DETAILS.md - Variables Implícitas (10 min)
- [ ] IMPLICIT_CONTEXT_DETAILS.md - Contextos Especiales (10 min)
- [ ] QUICK_REFERENCE.md - Checklist de Implementación

**Total**: ~60 minutos para lectura completa

---

## 🚀 Próximos Pasos

1. **Entender**: Lee los documentos en orden QUICK_REFERENCE → VISION → DEPENDENCIES
2. **Planificar**: Revisa el checklist en QUICK_REFERENCE.md
3. **Implementar**: Sigue el orden: Func. Anónimas → Pattern Matching → Clases Anónimas → Decoradores
4. **Validar**: Tests para cada feature según especificaciones
5. **Integrar**: Crear stdlib/decorators.rcc y refactorizar código existente

---

## 📚 Documentos Relacionados Existentes

En el repositorio puedes encontrar:
- Tests en `tests/test_decorators.rcc`
- Ejemplos en `tests/test_decorators_comprehensive.rcc`
- FFI actual en `stdlib/ffi.rcc`
- Registry actual en `src/runtime/decorator_registry.rs`

---

## 🤔 FAQ

**¿Por dónde empiezo a implementar?**
R: Comienza por funciones anónimas. Es lo más simple y todo lo demás depende de ello.

**¿Cuál es el documento más importante?**
R: QUICK_REFERENCE.md. Tiene todo lo que necesitas en forma concisa.

**¿Necesito leer TODA la documentación?**
R: No. QUICK_REFERENCE.md + VISION_DECORADORES.md son suficientes para empezar. Consulta los demás según necesites.

**¿Hay ejemplos de decoradores reales?**
R: Sí, hay muchos en VISION_DECORADORES.md y IMPLICIT_CONTEXT_DETAILS.md.

**¿Cuánto tiempo tardará implementar todo?**
R: 11-16 horas aproximadamente, dependiendo de tu experiencia con el codebase.

---

## 📞 Contacto / Preguntas

Si tienes dudas:
1. Revisa los documentos (muy detallados)
2. Consulta los ejemplos
3. Revisa el checklist para ver si algo falta

---

**Última actualización**: 2025-11-05
**Estado**: ✅ Documentación completa, lista para implementación
**Próximo paso**: Implementar Funciones Anónimas

---

## 📋 Indice Completo

1. **QUICK_REFERENCE.md** - Inicio rápido (5-10 min)
2. **VISION_DECORADORES.md** - Visión estratégica (10-15 min)
3. **DECORATOR_DEPENDENCIES.md** - Detalles técnicos (30-45 min)
4. **IMPLICIT_CONTEXT_DETAILS.md** - Contexto implícito (20-30 min)
5. **RESUMEN_PLAN.txt** - Vista visual (15-20 min)
6. **README_DECORADORES.md** - Este archivo (índice)

