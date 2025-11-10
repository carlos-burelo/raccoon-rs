# Implementación del IR (Intermediate Representation)

## Resumen

Este documento describe la implementación del IR (Intermediate Representation) para el intérprete Raccoon. El IR es una capa intermedia entre el AST y la ejecución que permite optimizar el código antes de ejecutarlo.

## Arquitectura

El sistema IR consta de cuatro componentes principales:

### 1. Instrucciones IR (`src/ir/instruction.rs`)

Define un conjunto de instrucciones de bajo nivel basadas en registros:

- **Operaciones básicas**: LoadConst, Move, Store, Load
- **Operaciones aritméticas**: BinaryOp, UnaryOp
- **Control de flujo**: Jump, JumpIfFalse, JumpIfTrue, Label
- **Funciones**: Call, Return, CreateFunction
- **Arrays y Objetos**: CreateArray, CreateObject, LoadIndex, StoreIndex, LoadProperty, StoreProperty
- **Operaciones especiales**: Await, TypeOf, InstanceOf, Match, etc.

El IR utiliza registros temporales (r0, r1, r2...) y registros de variables (local:x, global:x) para almacenar valores intermedios.

### 2. Compilador AST → IR (`src/ir/compiler.rs`)

El compilador `IRCompiler` transforma el AST en una secuencia de instrucciones IR:

```rust
let compiler = IRCompiler::new();
let ir_program = compiler.compile(&ast_program)?;
```

**Características**:
- Generación automática de registros temporales
- Generación automática de etiquetas para control de flujo
- Manejo de scopes
- Soporte para todas las construcciones del lenguaje

### 3. Optimizador IR (`src/ir/optimizer.rs`)

El optimizador `IROptimizer` aplica varias pasadas de optimización:

```rust
let optimizer = IROptimizer::new(ir_program);
let optimized = optimizer.optimize();
```

**Optimizaciones implementadas**:
- **Constant Folding**: Evalúa expresiones constantes en tiempo de compilación
  - Ejemplo: `2 + 3` → `5`
- **Dead Code Elimination**: Elimina código inalcanzable
- **Jump Threading**: Simplifica cadenas de saltos
- **Removal de Nops**: Elimina instrucciones no-op

### 4. Máquina Virtual (`src/ir/vm.rs`)

La VM ejecuta las instrucciones IR optimizadas:

```rust
let mut vm = VM::new(environment);
let result = vm.execute(optimized_program).await?;
```

**Características**:
- Ejecución stack-based con registros
- Soporte para operaciones asíncronas
- Manejo de scopes y variables
- Integración con el runtime existente

## Uso

### Habilitar el modo IR

```rust
let mut interpreter = Interpreter::new(Some("test.rcc".to_string()));
interpreter.enable_ir_mode(); // Activa el IR
let result = interpreter.interpret(&program).await?;
```

### Compilación y ejecución manual

```rust
use raccoon::{IRCompiler, IROptimizer, VM, Environment};

// 1. Compilar AST a IR
let compiler = IRCompiler::new();
let ir_program = compiler.compile(&ast_program)?;

// 2. Optimizar IR
let optimizer = IROptimizer::new(ir_program);
let optimized = optimizer.optimize();

// 3. Ejecutar con VM
let env = Environment::new(None);
let mut vm = VM::new(env);
let result = vm.execute(optimized).await?;
```

## Ejemplo de Compilación

### Código fuente:
```typescript
const x = 10;
const y = 20;
const z = x + y;
```

### IR generado (simplificado):
```
LoadConst r0, 10
Store "x", r0
LoadConst r1, 20
Store "y", r1
Load r2, "x"
Load r3, "y"
BinaryOp r4, r2, r3, Add
Store "z", r4
```

### IR optimizado:
```
LoadConst r0, 10
Store "x", r0
LoadConst r1, 20
Store "y", r1
LoadConst r4, 30      // Constant folding: 10 + 20 = 30
Store "z", r4
```

## Ventajas del IR

1. **Rendimiento mejorado**: Las optimizaciones reducen el trabajo en tiempo de ejecución
2. **Separación de preocupaciones**: La lógica de compilación está separada de la ejecución
3. **Flexibilidad**: Facilita agregar nuevas optimizaciones
4. **Análisis estático**: Permite análisis más profundo del código antes de ejecutar
5. **Portabilidad**: El IR es independiente del AST y puede ser generado por diferentes front-ends

## Limitaciones actuales

1. **Funciones**: La creación de funciones en el IR está simplificada
2. **Clases**: El soporte para clases está parcialmente implementado
3. **Excepciones**: Try-catch está simplificado
4. **Módulos**: Import/export no están completamente integrados

## Futuras mejoras

- [ ] Optimizaciones adicionales (inlining, loop unrolling, etc.)
- [ ] Soporte completo para closures y funciones de primera clase
- [ ] JIT compilation para hot paths
- [ ] Análisis de flujo de datos más avanzado
- [ ] Register allocation más sofisticada
- [ ] Serialización del IR para caché entre ejecuciones

## Notas de implementación

- El IR usa una representación de tres direcciones (3AC)
- Los registros temporales se generan automáticamente durante la compilación
- Las etiquetas se resuelven en la VM mediante un HashMap de posiciones
- El IR es compatible con operaciones asíncronas del runtime

## Rendimiento

El modo IR está diseñado para mejorar el rendimiento en:
- Programas con cálculos intensivos
- Loops con operaciones repetitivas
- Código con muchas expresiones constantes

Para scripts simples o ejecución única, el overhead de compilación puede no valer la pena.
