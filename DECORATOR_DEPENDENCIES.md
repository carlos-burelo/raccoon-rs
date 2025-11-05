# Decoradores Definibles por Usuario - Dependencias

Para implementar decoradores como se diseñó en `DECORATOR_USER_DEFINED.md` (Opción 3 con contexto implícito), necesitamos primero tres features:

1. **Pattern Matching** (como expresión)
2. **Funciones Anónimas/Lambda**
3. **Clases Anónimas**

---

## 1. Pattern Matching (como Expresión)

### Por Qué Lo Necesitamos

El decorador `@log` necesita distinguir entre contextos y **retornar valores diferentes**:

```raccoon
decorator log(prefix: str = "[LOG]") {
    // match es una EXPRESIÓN que retorna un valor
    return match context.type {
        "function" => fn(...args) {
            print(prefix, context.name, "called")
            return this(...args)
        },
        "method" => fn(...args) {
            print(prefix, "Method", context.name)
            return this(...args)
        },
        "class" => this,
        _ => this
    }
}
```

### Sintaxis Propuesta para Raccoon

```raccoon
// Match como expresión: retorna un valor
let greeting = match hour {
    0..12 => "Buenos días",
    12..18 => "Buenas tardes",
    18..24 => "Buenas noches",
    _ => "Hola"
}
print(greeting)

// Match con tipos
let description = match value {
    is int => "Número: " + value,
    is str => "Texto: " + value,
    is null => "Es nulo",
    _ => "Tipo desconocido"
}

// Match con desestructuración
let info = match point {
    { x: 0, y: 0 } => "origen",
    { x: 0, y } => "eje Y: " + y,
    { x, y: 0 } => "eje X: " + x,
    { x, y } => "(" + x + ", " + y + ")"
}

// Match con arrays
let status = match arr {
    [] => "vacío",
    [x] => "un elemento: " + x,
    [a, b] => "dos: " + a + ", " + b,
    [first, ...rest] => "múltiples"
}

// En decoradores: match retorna función
decorator flexible {
    return match context.type {
        "function" => wrap_function(this),
        "method" => wrap_method(this),
        "class" => wrap_class(this),
        _ => this
    }
}
```

### Sintaxis Formal

```
match_expr ::= "match" expression "{" match_arms "}"
match_arms ::= match_arm ("," match_arm)* ","?
match_arm  ::= pattern ("if" expression)? "=>" expression
pattern    ::= wildcard
             | literal
             | range
             | type_pattern
             | list_pattern
             | object_pattern
             | variable
             | or_pattern
```

### Cambios en AST

```rust
// Ya existe Expr enum, agregar:
pub enum Expr {
    // ... existentes
    Match(MatchExpr),  // NUEVO - Expresión match
}

pub struct MatchExpr {
    pub scrutinee: Box<Expr>,        // La expresión a analizar
    pub arms: Vec<MatchArm>,          // Las ramas del match
    pub position: Position,
}

pub struct MatchArm {
    pub pattern: Pattern,             // El patrón a comparar
    pub guard: Option<Box<Expr>>,    // if condition (opcional)
    pub body: Box<Expr>,              // Lo que retorna (es expresión!)
}

pub enum Pattern {
    Wildcard,                         // _
    Literal(Expr),                    // 42, "hello"
    Range(Box<Expr>, Box<Expr>),      // 0..10
    Type(Type),                       // is int
    List(Vec<Pattern>),               // [x, y, ...rest]
    Object(Vec<(String, Pattern)>),   // { x, y: n }
    Variable(String),                 // x (vincula variable)
    Or(Vec<Pattern>),                 // | alternativas
}
```

### Cambios en Parser

```rust
fn parse_primary_expr(&mut self) -> Result<Expr, RaccoonError> {
    // ... casos existentes
    if self.check_keyword("match") {
        return self.parse_match_expr();
    }
    // ...
}

fn parse_match_expr(&mut self) -> Result<Expr, RaccoonError> {
    self.expect_keyword("match")?;
    let scrutinee = Box::new(self.parse_expression()?);
    self.expect(TokenType::LeftBrace)?;

    let mut arms = Vec::new();

    while !self.check(TokenType::RightBrace) && !self.is_at_end() {
        let pattern = self.parse_pattern()?;

        let guard = if self.check_keyword("if") {
            self.consume_keyword("if")?;
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.expect(TokenType::FatArrow)?;
        let body = Box::new(self.parse_assignment_expr()?);

        arms.push(MatchArm { pattern, guard, body });

        if !self.check(TokenType::RightBrace) {
            self.expect(TokenType::Comma)?;
        }
    }

    self.expect(TokenType::RightBrace)?;

    Ok(Expr::Match(MatchExpr {
        scrutinee,
        arms,
        position: /* ... */,
    }))
}

fn parse_pattern(&mut self) -> Result<Pattern, RaccoonError> {
    // Parsear patrón
    if self.check(TokenType::Underscore) {
        self.consume(TokenType::Underscore)?;
        return Ok(Pattern::Wildcard);
    }

    if self.check_keyword("is") {
        self.consume_keyword("is")?;
        let typ = self.parse_type()?;
        return Ok(Pattern::Type(typ));
    }

    if self.check(TokenType::LeftBracket) {
        // Lista pattern
        return self.parse_list_pattern();
    }

    if self.check(TokenType::LeftBrace) {
        // Objeto pattern
        return self.parse_object_pattern();
    }

    // Literal o variable
    if self.check(TokenType::Identifier) {
        let name = self.expect_identifier()?;
        return Ok(Pattern::Variable(name));
    }

    // Literal (número, string, etc)
    let expr = self.parse_primary_expr()?;
    Ok(Pattern::Literal(expr))
}
```

### Cambios en Interpreter

```rust
pub async fn evaluate_match_expr(
    interpreter: &mut Interpreter,
    expr: &MatchExpr,
) -> Result<RuntimeValue, RaccoonError> {
    let scrutinee_value = interpreter.evaluate_expr(&expr.scrutinee).await?;

    for arm in &expr.arms {
        if interpreter.matches_pattern(&arm.pattern, &scrutinee_value)? {
            // Opcionalmente validar guard
            if let Some(guard) = &arm.guard {
                let guard_result = interpreter.evaluate_expr(guard).await?;
                if !interpreter.is_truthy(&guard_result) {
                    continue;  // Guard no pasó, intentar siguiente
                }
            }

            // Ejecutar el cuerpo del arm
            return interpreter.evaluate_expr(&arm.body).await;
        }
    }

    Err(RaccoonError::new(
        "No match found in match expression".to_string(),
        expr.position,
        interpreter.file.clone(),
    ))
}

fn matches_pattern(
    &mut self,
    pattern: &Pattern,
    value: &RuntimeValue,
) -> Result<bool, RaccoonError> {
    match pattern {
        Pattern::Wildcard => Ok(true),

        Pattern::Variable(name) => {
            // Siempre match, pero vincula la variable
            self.environment.declare(name.clone(), value.clone())?;
            Ok(true)
        }

        Pattern::Literal(expr) => {
            let literal_val = self.evaluate_expr(expr)?;
            Ok(value.equals(&literal_val))
        }

        Pattern::Type(typ) => {
            Ok(&value.get_type() == typ)
        }

        Pattern::Range(start, end) => {
            // Range solo para números
            match value {
                RuntimeValue::Int(IntValue { value: v }) => {
                    let s = self.evaluate_expr(start)?;
                    let e = self.evaluate_expr(end)?;
                    if let (RuntimeValue::Int(IntValue { value: sv }),
                             RuntimeValue::Int(IntValue { value: ev })) = (s, e) {
                        Ok(*v >= sv && *v < ev)  // [start, end)
                    } else {
                        Err(RaccoonError::new("Range pattern needs int bounds".into(), ...))
                    }
                }
                _ => Ok(false),
            }
        }

        Pattern::List(patterns) => {
            match value {
                RuntimeValue::List(ListValue { elements, .. }) => {
                    if patterns.len() != elements.len() {
                        return Ok(false);
                    }
                    for (pat, elem) in patterns.iter().zip(elements.iter()) {
                        if !self.matches_pattern(pat, elem)? {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                _ => Ok(false),
            }
        }

        Pattern::Object(fields) => {
            // Similar a lista pero para objetos
            match value {
                RuntimeValue::Object(ObjectValue { properties, .. }) => {
                    for (key, pattern) in fields {
                        if let Some(val) = properties.get(key) {
                            if !self.matches_pattern(pattern, val)? {
                                return Ok(false);
                            }
                        } else {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                _ => Ok(false),
            }
        }

        Pattern::Or(patterns) => {
            for pattern in patterns {
                if self.matches_pattern(pattern, value)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}
```

### Ejemplo Completo

```raccoon
decorator log(prefix: str = "[LOG]") {
    // match retorna una función o el mismo this
    return match context.type {
        "function" => fn(...args) {
            print(prefix, context.name, "called with", args)
            let result = this(...args)
            print(prefix, "returned:", result)
            return result
        },
        "method" => fn(...args) {
            print(prefix, "Method", context.name, "on", this.constructor.name)
            return this(...args)
        },
        "class" => this,
        _ => this
    }
}

@log("[TRACE]")
fn myFunc(x: int): int {
    return x * 2
}
```

---

## 2. Funciones Anónimas/Lambda

### Por Qué Lo Necesitamos

En decoradores necesitamos retornar funciones sin nombre:

```raccoon
decorator cache(ms: int = 5000) {
    return fn(...args) {  // ← Función anónima
        // Cuerpo
    }
}
```

### Sintaxis Propuesta

```raccoon
// Arrow function (ya existe - expresión)
let add = fn(x: int, y: int) => x + y

// Función anónima con cuerpo (NUEVO - también expresión)
let add = fn(x: int, y: int) {
    let result = x + y
    return result
}

// Sin parámetros
let getValue = fn {
    return 42
}

// Async
let getData = async fn {
    return await fetchData()
}

// Con spread
let printAll = fn(...args) {
    for (let arg of args) {
        print(arg)
    }
}

// En decorador (importante: es expresión)
decorator cache(ms: int) {
    let store = {}
    return fn(...args) {  // ← Retorna función anónima
        let key = stringify(args)
        if (store.has(key)) return store.get(key)
        let result = this(...args)
        store.set(key, result)
        return result
    }
}
```

### Cambios en AST

Ya tienes `ArrowFnExpr`, agregar:

```rust
pub enum Expr {
    // ... existentes
    ArrowFn(ArrowFnExpr),           // Existe: fn(x) => expr
    AnonymousFn(AnonymousFnExpr),   // NUEVO: fn { stmts }
}

pub struct AnonymousFnExpr {
    pub parameters: Vec<FnParam>,
    pub body: Vec<Stmt>,            // Lista de statements
    pub is_async: bool,
    pub position: Position,
}
```

### Cambios en Parser

Ya parseáis `fn`, solo hay que distinguir:

```rust
fn parse_primary_expr(&mut self) -> Result<Expr, RaccoonError> {
    if self.check_keyword("fn") || self.check_keyword("async fn") {
        return self.parse_function_expr();
    }
    // ...
}

fn parse_function_expr(&mut self) -> Result<Expr, RaccoonError> {
    let is_async = if self.check_keyword("async") {
        self.consume_keyword("async")?;
        true
    } else {
        false
    };

    self.expect_keyword("fn")?;

    let parameters = if self.check(TokenType::LeftParen) {
        self.parse_parameters()?
    } else {
        Vec::new()
    };

    if self.check(TokenType::FatArrow) {
        // Arrow: fn(x) => expr
        self.consume(TokenType::FatArrow)?;
        let body_expr = self.parse_assignment_expr()?;
        Ok(Expr::ArrowFn(ArrowFnExpr {
            parameters,
            body: body_expr,
            is_async,
            position: /* ... */,
        }))
    } else if self.check(TokenType::LeftBrace) {
        // Block: fn { stmts }
        let body = self.parse_block()?;
        Ok(Expr::AnonymousFn(AnonymousFnExpr {
            parameters,
            body,
            is_async,
            position: /* ... */,
        }))
    } else {
        Err(RaccoonError::new("Expected '=>' or '{'".into(), ...))
    }
}
```

### Cambios en Interpreter

```rust
pub async fn evaluate_anonymous_fn(
    interpreter: &mut Interpreter,
    expr: &AnonymousFnExpr,
) -> Result<RuntimeValue, RaccoonError> {
    let fn_type = Type::Function(Box::new(FunctionType {
        params: expr.parameters.iter().map(|p| p.param_type.clone()).collect(),
        return_type: PrimitiveType::unknown(),
        is_variadic: expr.parameters.iter().any(|p| p.is_rest),
    }));

    Ok(RuntimeValue::Function(
        FunctionValue::new(
            expr.parameters.clone(),
            expr.body.clone(),
            expr.is_async,
            fn_type,
        )
    ))
}
```

### Ejemplo

```raccoon
let add = fn(a: int, b: int) {
    return a + b
}
print(add(5, 3))  // 8

decorator timing {
    return fn(...args) {
        let start = now()
        let result = this(...args)
        let elapsed = now() - start
        print(context.name, "took", elapsed, "ms")
        return result
    }
}
```

---

## 3. Clases Anónimas

### Por Qué Lo Necesitamos

Algunos decoradores pueden necesitar crear clases dinámicamente:

```raccoon
decorator proxy {
    return class extends this {  // ← Clase anónima
        property name: str = "Proxied"
    }
}
```

### Sintaxis Propuesta

```raccoon
// Clase anónima - expresión
let MyClass = class {
    property x: int = 10

    method getValue() {
        return this.x
    }
}

let instance = new MyClass()

// Clase anónima con herencia
let Enhanced = class extends BaseClass {
    method override getValue() {
        return super.getValue() * 2
    }
}

// En decorador
decorator observable {
    return class extends this {
        property _observers: fn[] = []

        method notifyObservers() {
            for (let obs of this._observers) {
                obs(this)
            }
        }
    }
}
```

### Cambios en Parser

```rust
fn parse_primary_expr(&mut self) -> Result<Expr, RaccoonError> {
    if self.check_keyword("class") {
        // Puede ser statement O expresión
        return self.parse_class_expr_or_stmt();
    }
    // ...
}

fn parse_class_expr_or_stmt(&mut self) -> Result<Expr | Stmt, RaccoonError> {
    self.expect_keyword("class")?;

    // El nombre es OPCIONAL (diferencia clave)
    let name = if self.check(TokenType::Identifier)
        && !self.peek_ahead(1).is_some_and(|t| matches!(t, TokenType::LeftBrace | TokenType::Extends)) {
        Some(self.expect_identifier()?)
    } else {
        None
    };

    let superclass = if self.check_keyword("extends") {
        Some(self.expect_identifier()?)
    } else {
        None
    };

    // ... resto del parsing de clase

    if let Some(n) = name {
        // Es statement
        Ok(Stmt::ClassDecl(ClassDecl {
            name: n,
            superclass,
            // ...
        }))
    } else {
        // Es expresión (clase anónima)
        Ok(Expr::ClassLiteral(ClassLiteral {
            superclass,
            // ...
        }))
    }
}
```

### Cambios en AST

```rust
pub enum Expr {
    // ... existentes
    ClassLiteral(ClassLiteral),  // NUEVO
}

pub struct ClassLiteral {
    pub superclass: Option<String>,
    pub properties: Vec<ClassProperty>,
    pub constructor: Option<ConstructorDecl>,
    pub methods: Vec<ClassMethod>,
    pub position: Position,
}
```

### Cambios en Interpreter

```rust
pub async fn evaluate_class_literal(
    interpreter: &mut Interpreter,
    literal: &ClassLiteral,
) -> Result<RuntimeValue, RaccoonError> {
    // Similar a execute_class_decl pero:
    // - NO registra en environment
    // - Retorna el ClassValue directamente

    let mut static_methods = HashMap::new();
    let mut static_properties = HashMap::new();

    // ... procesamiento de métodos/propiedades

    Ok(RuntimeValue::Class(ClassValue::with_properties(
        "AnonymousClass".to_string(),  // Nombre genérico
        static_methods,
        static_properties,
        class_type,
        /* ... */,
    )))
}
```

### Ejemplo

```raccoon
let Point = class {
    property x: int
    property y: int

    constructor(px: int, py: int) {
        this.x = px
        this.y = py
    }

    method distance() {
        return Math.sqrt(this.x * this.x + this.y * this.y)
    }
}

let p = new Point(3, 4)
print(p.distance())  // 5

decorator cached {
    return class extends this {
        property _cache = {}

        method override getValue(key: str) {
            if (this._cache.has(key)) {
                return this._cache.get(key)
            }
            let result = super.getValue(key)
            this._cache.set(key, result)
            return result
        }
    }
}
```

---

## Plan de Implementación

### Orden Recomendado

1. **Funciones Anónimas** (PRIMERO)
   - Cambios mínimos (ya casi funciona)
   - Necesario para match y decoradores
   - **2-3 horas**

2. **Pattern Matching** (SEGUNDO)
   - Depende de funciones anónimas
   - Muy importante para decoradores contextuales
   - **4-6 horas**

3. **Clases Anónimas** (TERCERO)
   - Depende de funciones anónimas
   - Menos crítico que match
   - **2-3 horas**

4. **Decoradores Definibles** (FINAL)
   - Depende de los 3 anteriores
   - **3-4 horas**

**Total: 11-16 horas**

---

## Decorador Ejemplo: Cómo Se Vería Todo Junto

```raccoon
decorator smart(mode: str = "auto", ttl: int = 5000) {
    // Pattern matching (expresión): Detectar y retornar función/clase
    return match context.type {
        "function" => {  // cada rama retorna una expresión
            // Función anónima con lógica smart
            fn(...args) {
                let result = this(...args)
                return result
            }
        },
        "class" => {
            // Clase anónima extendida
            class extends this {
                constructor(...args) {
                    super(...args)
                    print("[smart] Instance of " + context.name)
                }

                method override toString() {
                    return "Smart[" + super.toString() + "]"
                }
            }
        },
        "method" => fn(...args) {
            print("[smart] Calling " + context.name)
            return this(...args)
        },
        _ => this
    }
}

@smart("cache", 3000)
fn fibonacci(n: int): int {
    if (n <= 1) return n
    return fibonacci(n-1) + fibonacci(n-2)
}

@smart("observable")
class DataModel {
    property value: int = 0

    method setValue(v: int) {
        this.value = v
    }
}
```

---

## Resumen

| Feature | Tipo | Para Decoradores | Complejidad | Dependencias |
|---------|------|------------------|-------------|--------------|
| Funciones Anónimas | Expresión | **Crítico** | Baja | Nada |
| Pattern Matching | Expresión | **Muy Importante** | Media | Func. Anónimas |
| Clases Anónimas | Expresión | Secundario | Media | Func. Anónimas |
| Decoradores | Sistema | **Final** | Media | Las 3 anteriores |

