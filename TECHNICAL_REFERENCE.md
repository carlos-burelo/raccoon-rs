# 📖 Referencia Técnica Completa - IR y VM de Raccoon

## Tabla de Contenidos
1. [Instrucciones IR](#instrucciones-ir)
2. [Compilador IR](#compilador-ir)
3. [Máquina Virtual](#máquina-virtual)
4. [Casos de Uso](#casos-de-uso)
5. [Especificaciones Técnicas](#especificaciones-técnicas)

---

## Instrucciones IR

### Categoría: Literales y Constantes

#### `LoadConst { dest, value }`
- **Descripción**: Carga una constante en un registro
- **Uso**: `let x = 42;`
- **IR**:
```
LoadConst { dest: r0, value: Int(42) }
```

---

### Categoría: Variables

#### `Declare { name, is_const }`
- **Descripción**: Declara una variable en el ambiente
- **Uso**: `let x = 10;`

#### `Load { dest, name }`
- **Descripción**: Carga el valor de una variable a un registro
- **Uso**: `println(x);`

#### `Store { name, src }`
- **Descripción**: Guarda el valor de un registro a una variable
- **Uso**: `x = y + 5;`

---

### Categoría: Operaciones Binarias

#### `BinaryOp { dest, left, right, op }`
- **Descripción**: Ejecuta una operación binaria
- **Operadores**: `+`, `-`, `*`, `/`, `%`, `**`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`
- **Uso**: `let z = x + y;`
- **IR**:
```
BinaryOp {
    dest: r0,
    left: r1,
    right: r2,
    op: Add
}
```

#### `UnaryOp { dest, operand, op }`
- **Descripción**: Ejecuta una operación unaria
- **Operadores**: `!`, `-`, `~`
- **Uso**: `let neg = -x;`

---

### Categoría: Control de Flujo

#### `Jump { label }`
- **Descripción**: Salto incondicional
- **Uso**: Bucles y bifurcaciones

#### `JumpIfTrue { condition, label }`
- **Descripción**: Salta si la condición es verdadera
- **Uso**: `if (x > 0)`

#### `JumpIfFalse { condition, label }`
- **Descripción**: Salta si la condición es falsa
- **Uso**: `if (x <= 0)`

#### `Label { name }`
- **Descripción**: Marca una posición para saltos
- **Uso**: Destino de jumps

#### `Break`
- **Descripción**: Sale del bucle actual
- **Uso**: `break;`

#### `Continue`
- **Descripción**: Continúa con la siguiente iteración
- **Uso**: `continue;`

---

### Categoría: Llamadas de Función

#### `Call { dest, callee, args }`
- **Descripción**: Llama una función con argumentos
- **Uso**: `let result = add(5, 3);`
- **IR**:
```
Call {
    dest: r0,
    callee: r1,  // registro con función
    args: [r2, r3]
}
```

#### `Return { value }`
- **Descripción**: Retorna de una función
- **Uso**: `return x + y;`

#### `CreateFunction { dest, name, params, body, is_async }`
- **Descripción**: Crea una función/closure
- **Uso**: `let fn = (x) => x * 2;`

---

### Categoría: Arrays

#### `CreateArray { dest, elements }`
- **Descripción**: Crea un array con elementos
- **Uso**: `let arr = [1, 2, 3];`
- **IR**:
```
CreateArray {
    dest: r0,
    elements: [r1, r2, r3]
}
```

#### `LoadIndex { dest, array, index }`
- **Descripción**: Accede a elemento de array
- **Uso**: `let val = arr[0];`

#### `StoreIndex { array, index, value }`
- **Descripción**: Modifica elemento de array
- **Uso**: `arr[0] = 10;`

---

### Categoría: Objetos

#### `CreateObject { dest, properties }`
- **Descripción**: Crea un objeto literal
- **Uso**: `let obj = { x: 1, y: 2 };`

#### `LoadProperty { dest, object, property }`
- **Descripción**: Accede a propiedad de objeto
- **Uso**: `let x = obj.x;`

#### `StoreProperty { object, property, value }`
- **Descripción**: Modifica propiedad de objeto
- **Uso**: `obj.x = 10;`

#### `MethodCall { dest, object, method, args }`
- **Descripción**: Llama un método de un objeto
- **Uso**: `let result = array.push(5);`

#### `DeleteProperty { dest, object, property }`
- **Descripción**: Elimina propiedad de objeto
- **Uso**: `delete obj.prop;`

#### `In { dest, property, object }`
- **Descripción**: Verifica existencia de propiedad
- **Uso**: `if ("prop" in obj)`

---

### Categoría: Clases

#### `CreateClass { name, constructor, methods, properties }`
- **Descripción**: Define una clase
- **Componentes**:
  - `name`: Nombre de la clase
  - `constructor`: (params, body) para inicialización
  - `methods`: [(name, params, body, is_async)]
  - `properties`: [(name, value)]
- **Uso**: `class Point { constructor(x, y) { ... } }`

#### `NewInstance { dest, class_name, args }`
- **Descripción**: Crea instancia de clase
- **Uso**: `let p = new Point(3, 4);`

#### `LoadThis { dest }`
- **Descripción**: Carga referencia 'this'
- **Uso**: Dentro de método: `this.x`

#### `CallSuper { dest, method, args }`
- **Descripción**: Llama método de clase padre
- **Uso**: `super.method(args);`

---

### Categoría: Spread Operator

#### `SpreadArray { dest, operand }`
- **Descripción**: Expande array en contexto de array
- **Uso**: `let arr2 = [...arr1, 4, 5];`

#### `SpreadObject { dest, operand }`
- **Descripción**: Expande objeto en contexto de objeto
- **Uso**: `let obj2 = {...obj1, c: 3};`

#### `SpreadCall { dest, operand }`
- **Descripción**: Expande argumentos en llamada
- **Uso**: `fn(...args);`

---

### Categoría: Módulos

#### `Import { dest, path, items }`
- **Descripción**: Importa módulo
- **Parámetros**:
  - `path`: Ruta del módulo
  - `items`: Símbolos específicos (vacío = default)
- **Uso**: `import { fn } from "module";`

#### `Export { name, value }`
- **Descripción**: Exporta símbolo
- **Uso**: `export const PI = 3.14;`

---

### Categoría: Bucles Especializados

#### `ForIn { variable, object, body }`
- **Descripción**: Itera sobre propiedades de objeto
- **Semantics**:
  - Objetos: itera sobre claves
  - Arrays: itera sobre índices
- **IR**:
```
ForIn {
    variable: "key",
    object: r0,
    body: [... instrucciones ...]
}
```

#### `ForOf { variable, iterable, body }`
- **Descripción**: Itera sobre elementos de iterable
- **Semantics**: Solo arrays actualmente
- **IR**:
```
ForOf {
    variable: "elem",
    iterable: r0,
    body: [... instrucciones ...]
}
```

---

### Categoría: Try-Catch

#### `TryCatch { try_body, catch_handler, finally_body }`
- **Descripción**: Bloque try-catch-finally
- **Estructura**:
  - `try_body`: Código a ejecutar
  - `catch_handler`: Some((var_name, handler_body))
  - `finally_body`: Option<body>
- **Semántica**: Try → (si error) Catch → Finally

---

### Categoría: Destructuring

#### `DestructureArray { dests, src, has_rest, rest_dest }`
- **Descripción**: Destructura array en variables
- **Uso**: `let [a, b, ...rest] = array;`

#### `DestructureObject { mappings, src, rest_dest }`
- **Descripción**: Destructura objeto en variables
- **Uso**: `let {x, y} = obj;`

---

### Categoría: Operadores Avanzados

#### `Increment { dest, operand, is_prefix }`
- **Descripción**: Incrementa valor (++/--)
- **Prefijo**: `++x` devuelve nuevo valor
- **Postfijo**: `x++` devuelve valor anterior

#### `Decrement { dest, operand, is_prefix }`
- **Descripción**: Decrementa valor

#### `CreateTemplate { dest, parts }`
- **Descripción**: Template string con interpolación
- **Partes**: String e Expr alternos

#### `Conditional { dest, condition, then_val, else_val }`
- **Descripción**: Operador ternario
- **Uso**: `let val = x > 0 ? x : -x;`

#### `NullCoalesce { dest, left, right }`
- **Descripción**: Nullish coalescing (??)
- **Uso**: `let val = null ?? "default";`

#### `OptionalChain { dest, object, property }`
- **Descripción**: Optional chaining (?.)
- **Uso**: `let val = obj?.prop?.nested;`

#### `NullAssert { dest, value }`
- **Descripción**: Null assertion (!)
- **Uso**: `let val = maybe_value!;`

---

### Categoría: Pattern Matching

#### `Match { dest, scrutinee, arms }`
- **Descripción**: Expresión match
- **Arms**: Patrón + Guard + Cuerpo
- **Uso**:
```
let result = match x {
    1 => "one",
    2 => "two",
    _ => "other"
};
```

---

### Categoría: Iteradores

#### `GetIterator { dest, iterable }`
- **Descripción**: Obtiene iterador de iterable

#### `IteratorNext { dest, iterator }`
- **Descripción**: Llama next() en iterador
- **Retorna**: {value, done}

---

### Categoría: Generadores

#### `CreateGenerator { dest, name, params, body }`
- **Descripción**: Crea función generadora

#### `Yield { value }`
- **Descripción**: Genera valor en generador
- **Uso**: `yield 42;`

---

### Categoría: Promesas

#### `Catch { dest, promise, handler }`
- **Descripción**: Captura error de promesa
- **Uso**: `promise.catch(err => ...)`

#### `Finally { block }`
- **Descripción**: Bloque finally de promesa

---

### Categoría: Operadores Especiales

#### `TypeOf { dest, operand }`
- **Descripción**: Obtiene tipo de valor
- **Valores**: "int", "float", "str", "bool", "array", "object", etc.

#### `InstanceOf { dest, operand, type_name }`
- **Descripción**: Verifica tipo de instancia
- **Uso**: `p instanceof Point`

#### `Await { dest, future }`
- **Descripción**: Espera resolución de future
- **Uso**: `let result = await promise;`

#### `Throw { value }`
- **Descripción**: Lanza excepción
- **Uso**: `throw "error message";`

---

### Categoría: Scope Management

#### `PushScope`
- **Descripción**: Crea nuevo scope
- **Uso**: Inicio de bloque

#### `PopScope`
- **Descripción**: Destruye scope actual
- **Uso**: Fin de bloque

---

### Categoría: Misc

#### `Nop`
- **Descripción**: No-op (sin operación)
- **Uso**: Eliminado en optimización

#### `Comment { text }`
- **Descripción**: Comentario en IR
- **Uso**: Debug/optimización

---

## Compilador IR

### Estructura del Compilador

```rust
pub struct IRCompiler {
    program: IRProgram,      // Programa siendo compilado
    temp_counter: usize,     // Contador de registros temporales
    label_counter: usize,    // Contador de etiquetas
    scope_depth: usize,      // Profundidad de scope
}
```

### Métodos Principales

#### `compile(program: &Program) -> Result<IRProgram, Error>`
- Compila un programa AST a IR
- Punto de entrada principal

#### `compile_stmt(stmt: &Stmt) -> Result<(), Error>`
- Compila una declaración
- Maneja: variables, funciones, clases, bucles, condicionales, etc.

#### `compile_expr(expr: &Expr) -> Result<Register, Error>`
- Compila una expresión
- Retorna el registro con el resultado

#### `next_temp() -> Register`
- Genera nuevo registro temporal (r0, r1, r2...)

#### `next_label(prefix: &str) -> String`
- Genera nueva etiqueta única

### Ejemplo: Compilar `let x = 10;`

```
AST:
VarDecl {
    pattern: Identifier("x"),
    initializer: Some(IntLiteral(10))
}

IR Generado:
Declare { name: "x", is_const: false }
LoadConst { dest: r0, value: Int(10) }
Store { name: "x", src: r0 }
```

### Ejemplo: Compilar `for (let i = 0; i < 5; i++) { ... }`

```
IR Generado:
Declare { name: "i", is_const: false }
LoadConst { dest: r0, value: Int(0) }
Store { name: "i", src: r0 }

Label { name: "for_start_0" }

Load { dest: r1, name: "i" }
LoadConst { dest: r2, value: Int(5) }
BinaryOp { dest: r3, left: r1, right: r2, op: Less }
JumpIfFalse { condition: r3, label: "for_end_0" }

[... cuerpo del bucle ...]

Label { name: "for_continue_0" }
Load { dest: r4, name: "i" }
LoadConst { dest: r5, value: Int(1) }
BinaryOp { dest: r6, left: r4, right: r5, op: Add }
Store { name: "i", src: r6 }

Jump { label: "for_start_0" }

Label { name: "for_end_0" }
PopScope
```

---

## Máquina Virtual

### Estructura de la VM

```rust
pub struct VM {
    registers: HashMap<String, RuntimeValue>,  // Archivo de registros
    environment: Environment,                  // Variables y scopes
    pc: usize,                                 // Program Counter
    call_stack: Vec<CallFrame>,               // Stack de llamadas
    program: Option<IRProgram>,               // Programa actual
}
```

### Ciclo de Ejecución

```
┌─────────────────────────────┐
│ Cargar instrucción en PC    │
└────────────┬────────────────┘
             │
             ▼
┌─────────────────────────────┐
│ Ejecutar instrucción        │
│ (match sobre tipo)          │
└────────────┬────────────────┘
             │
             ▼
┌─────────────────────────────┐
│ Obtener resultado           │
│ (ExecutionResult)           │
└────────────┬────────────────┘
             │
        ┌────┴─────────────────┐
        │                      │
        ▼                      ▼
    Continue            Jump/Return
        │                      │
        │                  ┌───┴───┐
        │                  │       │
        ▼                  ▼       ▼
    PC += 1           Jump Ret  Done
        │                  │
        └──────┬───────────┘
               │
               ▼
        Siguiente iteración
```

### Ejecución de Instrucción

```rust
async fn execute_instruction(
    &mut self,
    instruction: &Instruction,
) -> Result<ExecutionResult, Error>
```

**Retorna**:
- `ExecutionResult::Continue` - Continuar con siguiente instrucción
- `ExecutionResult::Jump(label)` - Saltar a etiqueta
- `ExecutionResult::Return(value)` - Retornar valor

### Ejemplo: Ejecutar `BinaryOp`

```
Instrucción:
BinaryOp {
    dest: r0,
    left: r1,
    right: r2,
    op: Add
}

Ejecución:
1. left_val = get_register(r1)      // Obtener valor de r1
2. right_val = get_register(r2)     // Obtener valor de r2
3. result = apply_binary_op(+)      // Ejecutar operación
4. set_register(r0, result)         // Guardar resultado en r0
5. Return Continue
```

### Manejo de For-In Loop

```rust
Instruction::ForIn { variable, object, body } => {
    let obj_val = get_register(object)?;

    // Iterar sobre propiedades
    for key in obj_val.keys() {
        // Declarar variable de loop
        environment.declare(variable, key)?;

        // Ejecutar cuerpo
        let mut loop_vm = VM::new(environment.clone());
        loop_vm.execute(body_program).await?;
    }
}
```

### Manejo de Try-Catch

```rust
Instruction::TryCatch { try_body, catch_handler, finally_body } => {
    // Ejecutar try
    match execute_try(try_body).await {
        Ok(val) => {
            // Éxito
        }
        Err(error) => {
            // Ejecutar catch si existe
            if let Some((var, handler)) = catch_handler {
                environment.declare(var, error_str)?;
                execute_catch(handler).await?;
            }
        }
    }

    // Siempre ejecutar finally
    if let Some(finally) = finally_body {
        execute_finally(finally).await?;
    }
}
```

---

## Casos de Uso

### Caso 1: Función Simple

```raccoon
function add(a, b) {
    return a + b;
}
let result = add(5, 3);
```

**IR**:
```
CreateFunction {
    dest: global:add,
    name: "add",
    params: ["a", "b"],
    body: [
        Load { dest: r0, name: "a" },
        Load { dest: r1, name: "b" },
        BinaryOp { dest: r2, left: r0, right: r1, op: Add },
        Return { value: Some(r2) }
    ]
}

Load { dest: r3, name: "add" }
LoadConst { dest: r4, value: Int(5) }
LoadConst { dest: r5, value: Int(3) }
Call { dest: r6, callee: r3, args: [r4, r5] }
Store { name: "result", src: r6 }
```

### Caso 2: Clase Simple

```raccoon
class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    distance() {
        return (this.x * this.x + this.y * this.y) ** 0.5;
    }
}

let p = new Point(3, 4);
```

**IR**:
```
CreateClass {
    name: "Point",
    constructor: Some((
        ["x", "y"],
        [
            LoadThis { dest: r0 },
            Load { dest: r1, name: "x" },
            StoreProperty { object: r0, property: "x", value: r1 },
            LoadThis { dest: r2 },
            Load { dest: r3, name: "y" },
            StoreProperty { object: r2, property: "y", value: r3 }
        ]
    )),
    methods: [
        ("distance", [], [...body...], false)
    ]
}

LoadConst { dest: r0, value: Int(3) }
LoadConst { dest: r1, value: Int(4) }
NewInstance { dest: r2, class_name: "Point", args: [r0, r1] }
Store { name: "p", src: r2 }
```

### Caso 3: For-Of Loop

```raccoon
let arr = [1, 2, 3];
for elem of arr {
    println(elem);
}
```

**IR**:
```
CreateArray { dest: r0, elements: [r1, r2, r3] }
Store { name: "arr", src: r0 }

ForOf {
    variable: "elem",
    iterable: r0,
    body: [
        Load { dest: r4, name: "elem" },
        ... (println implementation)
    ]
}
```

---

## Especificaciones Técnicas

### Tipos de Registros

```
r0-r999     = Temporales (generados por compilador)
local:x     = Variable local
global:x    = Variable global
```

### Ambiente de Ejecución

- **Scopes**: Stack de diccionarios (variable → valor)
- **Push/Pop**: Cuando entrar/salir de bloque
- **Lookup**: Búsqueda desde scope actual hacia arriba

### Manejo de Errores

Todos los `Result<T, RaccoonError>` propagan errores:
- Errores de tipo
- Variables no encontradas
- Argumentos incorrectos
- Excepciones lanzadas (throw)

### Características Asincrónicas

- `async fn` en VM para await/async
- `async_recursion` para recursión
- Futures como valores RuntimeValue
- `.await?` para manejo de errores

---

## Conclusión

La arquitectura del IR y VM proporciona:
✅ Separación clara entre compilación y ejecución
✅ Posibilidad de optimizaciones
✅ Soporte para todas las características del lenguaje
✅ Base sólida para extensiones futuras

---

*Referencia Técnica Completa - v1.0*
