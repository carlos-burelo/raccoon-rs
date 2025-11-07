# Sistema de Tipos como Objetos - Diseño

## Objetivo

Implementar un sistema donde **todos los tipos sean objetos de primera clase** sin hardcodear lógica y evitando deuda técnica.

## Arquitectura Actual (Problema)

```
❌ Fragmentado:
- PrimitiveTypeObject (para Future, Object)
- EnumObject (para enums)
- ClassValue (para clases)
- Sin representación unificada
```

## Arquitectura Propuesta (Solución)

```
✅ Unificado:
- TypeObject (TODOS los tipos)
- TypeKind (clasificación)
- TypeHandler (comportamiento)
- TypeRegistry (gestión centralizada)
```

## 1. Definición de TypeObject

```rust
// src/runtime/values.rs

/// Representa cualquier tipo como un objeto de primera clase en runtime
#[derive(Debug, Clone)]
pub struct TypeObject {
    /// La definición del tipo (compile-time)
    pub type_def: Type,

    /// Clasificación del tipo
    pub kind: TypeKind,

    /// Métodos estáticos (Future.resolve, Object.keys, etc.)
    pub static_methods: HashMap<String, RuntimeValue>,

    /// Propiedades estáticas (String.empty, Math.PI, etc.)
    pub static_properties: HashMap<String, RuntimeValue>,

    /// Constructor del tipo (new MyClass())
    pub constructor: Option<Box<RuntimeValue>>,

    /// Metadata adicional (documentación, anotaciones, etc.)
    pub metadata: TypeMetadata,
}

/// Clasificación de tipos
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    /// Tipos primitivos (int, str, bool, etc.)
    Primitive(PrimitiveKind),

    /// Clase definida por usuario
    Class {
        name: String,
        superclass: Option<String>,
    },

    /// Interface
    Interface {
        name: String,
    },

    /// Enum
    Enum {
        name: String,
        variants: Vec<String>,
    },

    /// Tipo función
    Function,

    /// Tipo genérico no instanciado
    Generic {
        name: String,
        constraints: Vec<Type>,
    },

    /// Alias de tipo
    Alias {
        name: String,
        target: Box<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveKind {
    Int,
    BigInt,
    Float,
    Decimal,
    String,
    Bool,
    Null,
    Void,
    Any,
    Unknown,
    Never,
}

#[derive(Debug, Clone, Default)]
pub struct TypeMetadata {
    pub documentation: Option<String>,
    pub source_location: Option<SourceLocation>,
    pub decorators: Vec<String>,
}
```

## 2. Builder Pattern para Construir Tipos

```rust
// src/runtime/type_object_builder.rs

pub struct TypeObjectBuilder {
    type_def: Type,
    kind: TypeKind,
    static_methods: HashMap<String, RuntimeValue>,
    static_properties: HashMap<String, RuntimeValue>,
    constructor: Option<Box<RuntimeValue>>,
    metadata: TypeMetadata,
}

impl TypeObjectBuilder {
    pub fn new(type_def: Type, kind: TypeKind) -> Self {
        Self {
            type_def,
            kind,
            static_methods: HashMap::new(),
            static_properties: HashMap::new(),
            constructor: None,
            metadata: TypeMetadata::default(),
        }
    }

    pub fn static_method(mut self, name: &str, func: RuntimeValue) -> Self {
        self.static_methods.insert(name.to_string(), func);
        self
    }

    pub fn static_property(mut self, name: &str, value: RuntimeValue) -> Self {
        self.static_properties.insert(name.to_string(), value);
        self
    }

    pub fn constructor(mut self, func: RuntimeValue) -> Self {
        self.constructor = Some(Box::new(func));
        self
    }

    pub fn documentation(mut self, doc: &str) -> Self {
        self.metadata.documentation = Some(doc.to_string());
        self
    }

    pub fn build(self) -> TypeObject {
        TypeObject {
            type_def: self.type_def,
            kind: self.kind,
            static_methods: self.static_methods,
            static_properties: self.static_properties,
            constructor: self.constructor,
            metadata: self.metadata,
        }
    }
}
```

## 3. Integración con Builtins

```rust
// src/runtime/builtins.rs

pub fn setup_builtins(env: &mut Environment) {
    // Registrar tipos primitivos
    register_primitive_types(env);

    // Registrar tipos complejos (Future, Object, etc.)
    register_future_type(env);
    register_object_type(env);
    register_array_type(env);

    // Registrar funciones globales
    register_global_functions(env);
}

fn register_primitive_types(env: &mut Environment) {
    // int
    let int_type = TypeObjectBuilder::new(
        Type::Primitive(PrimitiveType::Int),
        TypeKind::Primitive(PrimitiveKind::Int),
    )
    .static_method("parse", create_native_fn(|args| {
        // Implementación de int.parse(str)
        todo!()
    }))
    .static_method("max", create_native_fn(|args| {
        // Implementación de int.max(a, b)
        todo!()
    }))
    .static_method("min", create_native_fn(|args| {
        // Implementación de int.min(a, b)
        todo!()
    }))
    .static_property("MAX_VALUE", RuntimeValue::Int(IntValue::new(i64::MAX)))
    .static_property("MIN_VALUE", RuntimeValue::Int(IntValue::new(i64::MIN)))
    .documentation("32/64-bit signed integer type")
    .build();

    env.declare("int", RuntimeValue::Type(int_type));

    // str
    let str_type = TypeObjectBuilder::new(
        Type::Primitive(PrimitiveType::Str),
        TypeKind::Primitive(PrimitiveKind::String),
    )
    .static_method("fromCharCode", create_native_fn(|args| {
        todo!()
    }))
    .static_property("empty", RuntimeValue::Str(StrValue::new("")))
    .documentation("Unicode string type")
    .build();

    env.declare("str", RuntimeValue::Type(str_type));
}

fn register_future_type(env: &mut Environment) {
    let future_type = TypeObjectBuilder::new(
        Type::Generic(Box::new(GenericType {
            base: Box::new(Type::Primitive(PrimitiveType::Future)),
            type_args: vec![Type::TypeParam(TypeParameter {
                name: "T".to_string(),
                constraint: None,
            })],
        })),
        TypeKind::Generic {
            name: "Future".to_string(),
            constraints: vec![],
        },
    )
    .static_method("resolve", create_native_fn(|args| {
        // Implementación actual de Future.resolve
        todo!()
    }))
    .static_method("reject", create_native_fn(|args| {
        // Implementación actual de Future.reject
        todo!()
    }))
    .static_method("all", create_native_fn(|args| {
        // Implementación actual de Future.all
        todo!()
    }))
    .static_method("race", create_native_fn(|args| {
        todo!()
    }))
    .static_method("allSettled", create_native_fn(|args| {
        todo!()
    }))
    .static_method("any", create_native_fn(|args| {
        todo!()
    }))
    .documentation("Asynchronous computation type")
    .build();

    env.declare("Future", RuntimeValue::Type(future_type));
}
```

## 4. Integración con Clases de Usuario

```rust
// src/interpreter/statements.rs

// Cuando el intérprete encuentra una declaración de clase
fn eval_class_declaration(&mut self, class: &ClassDeclaration) -> Result<RuntimeValue, RaccoonError> {
    // 1. Crear el TypeObject para la clase
    let class_type_obj = TypeObjectBuilder::new(
        Type::Class(Box::new(ClassType {
            name: class.name.clone(),
            // ... otros campos
        })),
        TypeKind::Class {
            name: class.name.clone(),
            superclass: class.superclass.as_ref().map(|s| s.name.clone()),
        },
    )
    .constructor(RuntimeValue::NativeFunction(/* constructor */))
    .documentation(extract_doc_comment(&class.decorators))
    .build();

    // 2. Registrar en el environment
    self.environment.declare(&class.name, RuntimeValue::Type(class_type_obj));

    // 3. Registrar el TypeHandler para métodos de instancia
    self.type_registry.register_handler(
        &class.name,
        Box::new(UserDefinedClassHandler::new(class.clone())),
    );

    Ok(RuntimeValue::Null(NullValue))
}
```

## 5. Integración con Enums

```rust
// src/interpreter/statements.rs

fn eval_enum_declaration(&mut self, enum_decl: &EnumDeclaration) -> Result<RuntimeValue, RaccoonError> {
    let variants: Vec<String> = enum_decl.members.iter()
        .map(|m| m.name.clone())
        .collect();

    let enum_type_obj = TypeObjectBuilder::new(
        Type::Enum(Box::new(EnumType {
            name: enum_decl.name.clone(),
            members: enum_decl.members.clone(),
        })),
        TypeKind::Enum {
            name: enum_decl.name.clone(),
            variants: variants.clone(),
        },
    )
    .documentation(extract_doc_comment(&enum_decl.decorators))
    .build();

    // Agregar cada variante como propiedad estática
    let mut enum_obj = enum_type_obj;
    for member in &enum_decl.members {
        let variant_value = match &member.value {
            EnumMemberValue::Int(i) => RuntimeValue::Int(IntValue::new(*i)),
            EnumMemberValue::Str(s) => RuntimeValue::Str(StrValue::new(s)),
        };
        enum_obj.static_properties.insert(member.name.clone(), variant_value);
    }

    self.environment.declare(&enum_decl.name, RuntimeValue::Type(enum_obj));

    Ok(RuntimeValue::Null(NullValue))
}
```

## 6. Reflection API

```rust
// Implementar typeof como builtin que retorna TypeObject

fn register_reflection_builtins(env: &mut Environment) {
    // typeof operator
    let typeof_fn = create_native_fn(|args| {
        if args.len() != 1 {
            return Err("typeof expects 1 argument".into());
        }

        let value = &args[0];
        let type_obj = value_to_type_object(value);

        Ok(RuntimeValue::Type(type_obj))
    });

    env.declare("typeof", RuntimeValue::NativeFunction(typeof_fn));

    // instanceof operator (ya debería existir)
}

fn value_to_type_object(value: &RuntimeValue) -> TypeObject {
    match value {
        RuntimeValue::Int(_) => {
            // Retornar el TypeObject de int
            get_primitive_type_object(PrimitiveKind::Int)
        }
        RuntimeValue::Str(_) => {
            get_primitive_type_object(PrimitiveKind::String)
        }
        RuntimeValue::ClassInstance(instance) => {
            // Retornar el TypeObject de la clase
            get_class_type_object(&instance.class_name)
        }
        // ... otros casos
        _ => get_primitive_type_object(PrimitiveKind::Unknown),
    }
}
```

## 7. Uso en el Lenguaje

```javascript
// 1. Tipos primitivos como valores
const IntType = int;
print(IntType.MAX_VALUE);  // 9223372036854775807

const parsed = int.parse("42");  // 42

// 2. Tipos de usuario como valores
class Person {
    constructor(name: str) {
        this.name = name;
    }

    static fromJSON(json: str): Person {
        // ...
    }
}

const PersonType = Person;  // TypeObject
const john = PersonType.fromJSON('{"name": "John"}');

// 3. Reflection
fn printTypeInfo(value: any) {
    const t = typeof value;
    print(`Type: ${t.name}`);
    print(`Kind: ${t.kind}`);

    if (t.kind == "class") {
        print(`Superclass: ${t.superclass}`);
    }
}

printTypeInfo(42);          // Type: int, Kind: primitive
printTypeInfo(john);        // Type: Person, Kind: class
printTypeInfo("hello");     // Type: str, Kind: primitive

// 4. Enums
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
}

const ColorType = Color;  // TypeObject
print(ColorType.variants);  // ["Red", "Green", "Blue"]
const red = Color.Red;      // EnumVariant
```

## 8. Ventajas de esta Arquitectura

1. **No hardcoding**: Mismo sistema para builtins y tipos de usuario
2. **Extensible**: Agregar nuevos tipos sin modificar el core
3. **Reflection completo**: Introspección de tipos en runtime
4. **Type safety**: Type checking en compile-time + runtime
5. **Builder pattern**: Construcción consistente de tipos
6. **Metadata**: Documentación y anotaciones integradas
7. **Unificado**: Una sola abstracción para todos los tipos

## 9. Migración desde la Arquitectura Actual

### Paso 1: Crear TypeObject y TypeKind
```
src/runtime/type_object.rs (nuevo)
src/runtime/type_object_builder.rs (nuevo)
```

### Paso 2: Refactorizar RuntimeValue
```rust
pub enum RuntimeValue {
    // ...

    // ❌ REMOVER:
    // PrimitiveTypeObject(PrimitiveTypeObject),
    // EnumObject(EnumObject),
    // ClassObject(ClassObject),

    // ✅ AGREGAR:
    Type(TypeObject),  // Unified type-as-value

    // ...
}
```

### Paso 3: Migrar Builtins
```
1. Future: PrimitiveTypeObject → TypeObject
2. Object: PrimitiveTypeObject → TypeObject
3. Enums: EnumObject → TypeObject
4. Agregar tipos primitivos (int, str, bool, etc.)
```

### Paso 4: Integrar con Clases
```
Al declarar clase → crear TypeObject
Registrar constructor como static method
```

### Paso 5: Agregar Reflection
```
Implementar typeof que retorna TypeObject
Extender instanceof para usar TypeObject
```

## 10. Resumen

```
┌─────────────────────────────────────────────────┐
│           UNIFIED TYPE SYSTEM                    │
├─────────────────────────────────────────────────┤
│                                                  │
│  Type (AST) ─────────┐                          │
│                      │                          │
│                      ▼                          │
│                TypeObject ◄────── TypeKind      │
│                      │                          │
│         ┌────────────┼────────────┐             │
│         │            │            │             │
│         ▼            ▼            ▼             │
│    static_methods  constructor  metadata        │
│                                                  │
│  TypeHandler ────► instance methods             │
│                                                  │
│  TypeRegistry ───► centralized management       │
│                                                  │
└─────────────────────────────────────────────────┘
```

**Regla de Oro**: Un tipo = Un TypeObject = Un TypeHandler

**No más**:
- PrimitiveTypeObject para algunos tipos
- EnumObject para enums
- ClassValue para clases
- Lógica hardcodeada para cada caso

**Ahora**:
- TypeObject para TODOS los tipos
- TypeObjectBuilder para construcción consistente
- TypeHandler para comportamiento
- TypeRegistry para gestión centralizada
