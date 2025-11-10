use crate::ast::{nodes::*, types::*};
use crate::runtime::dynamic::DynamicRuntimeValue;
use crate::runtime::type_object::TypeObject;
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tokio::sync::Notify;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Int(IntValue),
    BigInt(BigIntValue),
    Float(FloatValue),
    Decimal(DecimalValue),
    Str(StrValue),
    Bool(BoolValue),
    Null(NullValue),
    Array(ArrayValue),
    Map(MapValue),
    Object(ObjectValue),
    Class(ClassValue),
    ClassInstance(ClassInstance),
    Function(FunctionValue),
    NativeFunction(NativeFunctionValue),
    NativeAsyncFunction(NativeAsyncFunctionValue),
    Future(FutureValue),
    Enum(EnumValue),
    PrimitiveTypeObject(PrimitiveTypeObject),
    EnumObject(EnumObject),
    Type(TypeObject),
    Dynamic(DynamicRuntimeValue),
}

impl RuntimeValue {
    pub fn get_type(&self) -> Type {
        match self {
            RuntimeValue::Int(_) => PrimitiveType::int(),
            RuntimeValue::BigInt(_) => PrimitiveType::bigint(),
            RuntimeValue::Float(_) => PrimitiveType::float(),
            RuntimeValue::Decimal(_) => PrimitiveType::decimal(),
            RuntimeValue::Str(_) => PrimitiveType::str(),
            RuntimeValue::Bool(_) => PrimitiveType::bool(),
            RuntimeValue::Null(_) => PrimitiveType::null(),
            RuntimeValue::Array(l) => Type::Array(Box::new(ArrayType {
                element_type: l.element_type.clone(),
            })),
            RuntimeValue::Map(m) => Type::Map(Box::new(MapType {
                key_type: m.key_type.clone(),
                value_type: m.value_type.clone(),
            })),
            RuntimeValue::Object(o) => o.obj_type.clone(),
            RuntimeValue::Class(c) => c.class_type.clone(),
            RuntimeValue::ClassInstance(c) => c.class_type.clone(),
            RuntimeValue::Function(f) => f.fn_type.clone(),
            RuntimeValue::NativeFunction(f) => f.fn_type.clone(),
            RuntimeValue::NativeAsyncFunction(f) => f.fn_type.clone(),
            RuntimeValue::Future(f) => Type::Future(Box::new(FutureType {
                inner_type: f.value_type.clone(),
            })),
            RuntimeValue::Enum(e) => e.enum_type.clone(),
            RuntimeValue::PrimitiveTypeObject(p) => p.type_obj.clone(),
            RuntimeValue::EnumObject(e) => e.enum_type.clone(),
            RuntimeValue::Type(t) => t.type_def.clone(),
            RuntimeValue::Dynamic(d) => d.get_type(),
        }
    }

    pub fn get_type_object(&self) -> TypeObject {
        use crate::runtime::type_object::{PrimitiveKind, TypeKind, TypeMetadata};
        use std::collections::HashMap;

        match self {
            RuntimeValue::Int(_) => TypeObject::new(
                PrimitiveType::int(),
                TypeKind::Primitive(PrimitiveKind::Int),
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::BigInt(_) => TypeObject::new(
                PrimitiveType::bigint(),
                TypeKind::Primitive(PrimitiveKind::BigInt),
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::Float(_) => TypeObject::new(
                PrimitiveType::float(),
                TypeKind::Primitive(PrimitiveKind::Float),
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::Decimal(_) => TypeObject::new(
                PrimitiveType::decimal(),
                TypeKind::Primitive(PrimitiveKind::Decimal),
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::Str(_) => TypeObject::new(
                PrimitiveType::str(),
                TypeKind::Primitive(PrimitiveKind::String),
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::Bool(_) => TypeObject::new(
                PrimitiveType::bool(),
                TypeKind::Primitive(PrimitiveKind::Bool),
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::Null(_) => TypeObject::new(
                PrimitiveType::null(),
                TypeKind::Primitive(PrimitiveKind::Null),
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::Type(t) => t.clone(),
            RuntimeValue::Class(c) => TypeObject::new(
                c.class_type.clone(),
                TypeKind::Class {
                    name: c.class_name.clone(),
                    superclass: c.declaration.superclass.as_ref().map(|s| s.clone()),
                },
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
            RuntimeValue::EnumObject(e) => {
                let variants: Vec<String> =
                    e.members.iter().map(|(name, _)| name.clone()).collect();
                TypeObject::new(
                    e.enum_type.clone(),
                    TypeKind::Enum {
                        name: e.enum_name.clone(),
                        variants,
                    },
                    HashMap::new(),
                    HashMap::new(),
                    None,
                    TypeMetadata::new(),
                )
            }
            _ => TypeObject::new(
                self.get_type(),
                TypeKind::Unknown,
                HashMap::new(),
                HashMap::new(),
                None,
                TypeMetadata::new(),
            ),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RuntimeValue::Int(v) => v.to_string(),
            RuntimeValue::BigInt(v) => v.to_string(),
            RuntimeValue::Float(v) => v.to_string(),
            RuntimeValue::Decimal(v) => v.to_string(),
            RuntimeValue::Str(v) => v.to_string(),
            RuntimeValue::Bool(v) => v.to_string(),
            RuntimeValue::Null(v) => v.to_string(),
            RuntimeValue::Array(v) => v.to_string(),
            RuntimeValue::Map(v) => v.to_string(),
            RuntimeValue::Object(v) => v.to_string(),
            RuntimeValue::Class(v) => v.to_string(),
            RuntimeValue::ClassInstance(v) => v.to_string(),
            RuntimeValue::Function(v) => v.to_string(),
            RuntimeValue::NativeFunction(v) => v.to_string(),
            RuntimeValue::NativeAsyncFunction(v) => v.to_string(),
            RuntimeValue::Future(v) => v.to_string(),
            RuntimeValue::Enum(v) => v.to_string(),
            RuntimeValue::PrimitiveTypeObject(v) => v.to_string(),
            RuntimeValue::EnumObject(v) => v.to_string(),
            RuntimeValue::Type(t) => format!("[type {}]", t.name()),
            RuntimeValue::Dynamic(d) => d.to_string(),
        }
    }

    pub fn equals(&self, other: &RuntimeValue) -> bool {
        match (self, other) {
            (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a.value == b.value,
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a.value == b.value,
            (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a.value == b.value,
            (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a.value == b.value,
            (RuntimeValue::Null(_), RuntimeValue::Null(_)) => true,
            _ => false,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            RuntimeValue::Class(c) => c.class_name.clone(),
            RuntimeValue::ClassInstance(c) => c.class_name.clone(),
            RuntimeValue::Function(f) => f.name.clone().unwrap_or_else(|| {
                if f.is_async {
                    "[async fn]".to_string()
                } else {
                    "[fn]".to_string()
                }
            }),
            RuntimeValue::NativeFunction(_) => "[Native fn]".to_string(),
            RuntimeValue::NativeAsyncFunction(_) => "[Native async fn]".to_string(),
            RuntimeValue::Future(_) => "future".to_string(),
            RuntimeValue::PrimitiveTypeObject(p) => p.type_name.clone(),
            RuntimeValue::EnumObject(e) => e.enum_name.clone(),
            RuntimeValue::Enum(e) => e.enum_name.clone(),
            RuntimeValue::Type(t) => t.name(),
            RuntimeValue::Int(_) => "int".to_string(),
            RuntimeValue::BigInt(_) => "bigint".to_string(),
            RuntimeValue::Float(_) => "float".to_string(),
            RuntimeValue::Decimal(_) => "decimal".to_string(),
            RuntimeValue::Str(_) => "str".to_string(),
            RuntimeValue::Bool(_) => "bool".to_string(),
            RuntimeValue::Null(_) => "null".to_string(),
            RuntimeValue::Array(_) => "array".to_string(),
            RuntimeValue::Map(_) => "map".to_string(),
            RuntimeValue::Object(_) => "object".to_string(),
            RuntimeValue::Dynamic(d) => d.type_name().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntValue {
    pub value: i64,
}

impl IntValue {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BigIntValue {
    pub value: i128, // Using i128 for simplicity instead of external bigint library
}

impl BigIntValue {
    pub fn new(value: i128) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        format!("{}n", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct FloatValue {
    pub value: f64,
}

impl FloatValue {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct DecimalValue {
    pub value: f64,
}

impl DecimalValue {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct StrValue {
    pub value: String,
}

impl StrValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoolValue {
    pub value: bool,
}

impl BoolValue {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        if self.value {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct NullValue;

impl NullValue {
    pub fn new() -> Self {
        Self
    }

    pub fn to_string(&self) -> String {
        "null".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ArrayValue {
    pub elements: Vec<RuntimeValue>,
    pub element_type: Type,
}

impl ArrayValue {
    pub fn new(elements: Vec<RuntimeValue>, element_type: Type) -> Self {
        Self {
            elements,
            element_type,
        }
    }

    pub fn to_string(&self) -> String {
        let elements_str: Vec<String> = self.elements.iter().map(|e| e.to_string()).collect();
        format!("[{}]", elements_str.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct MapValue {
    pub entries: HashMap<String, RuntimeValue>,
    pub key_type: Type,
    pub value_type: Type,
}

impl MapValue {
    pub fn new(entries: HashMap<String, RuntimeValue>, key_type: Type, value_type: Type) -> Self {
        Self {
            entries,
            key_type,
            value_type,
        }
    }

    pub fn to_string(&self) -> String {
        let entries_str: Vec<String> = self
            .entries
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_string()))
            .collect();
        format!("Map {{ {} }}", entries_str.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct ObjectValue {
    pub properties: HashMap<String, RuntimeValue>,
    pub obj_type: Type,
}

impl ObjectValue {
    pub fn new(properties: HashMap<String, RuntimeValue>, obj_type: Type) -> Self {
        Self {
            properties,
            obj_type,
        }
    }

    pub fn to_string(&self) -> String {
        let props_str: Vec<String> = self
            .properties
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_string()))
            .collect();
        format!("{{ {} }}", props_str.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct ClassValue {
    pub class_name: String,
    pub static_methods: HashMap<String, Box<FunctionValue>>,
    pub static_properties: HashMap<String, RuntimeValue>,
    pub class_type: Type,
    pub declaration: ClassDecl,
}

impl ClassValue {
    pub fn new(
        class_name: String,
        static_methods: HashMap<String, Box<FunctionValue>>,
        class_type: Type,
        declaration: ClassDecl,
    ) -> Self {
        Self {
            class_name,
            static_methods,
            static_properties: HashMap::new(),
            class_type,
            declaration,
        }
    }

    pub fn with_properties(
        class_name: String,
        static_methods: HashMap<String, Box<FunctionValue>>,
        static_properties: HashMap<String, RuntimeValue>,
        class_type: Type,
        declaration: ClassDecl,
    ) -> Self {
        Self {
            class_name,
            static_methods,
            static_properties,
            class_type,
            declaration,
        }
    }

    pub fn to_string(&self) -> String {
        format!("class {}", self.class_name)
    }
}

#[derive(Debug, Clone)]
pub struct ClassInstance {
    pub class_name: String,
    pub properties: Arc<RwLock<HashMap<String, RuntimeValue>>>,
    pub methods: HashMap<String, FunctionValue>,
    pub accessors: Vec<PropertyAccessor>,
    pub class_type: Type,
}

impl ClassInstance {
    pub fn new(
        class_name: String,
        properties: HashMap<String, RuntimeValue>,
        methods: HashMap<String, FunctionValue>,
        accessors: Vec<PropertyAccessor>,
        class_type: Type,
    ) -> Self {
        Self {
            class_name,
            properties: Arc::new(RwLock::new(properties)),
            methods,
            accessors,
            class_type,
        }
    }

    pub fn to_string(&self) -> String {
        let properties = self.properties.read().unwrap();
        let props_str: Vec<String> = properties
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_string()))
            .collect();
        format!("{} {{ {} }}", self.class_name, props_str.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub name: Option<String>,
    pub parameters: Vec<FnParam>,
    pub body: Vec<Stmt>,
    pub is_async: bool,
    pub fn_type: Type,
    pub decorators: Vec<DecoratorDecl>,
}

impl FunctionValue {
    pub fn new(parameters: Vec<FnParam>, body: Vec<Stmt>, is_async: bool, fn_type: Type) -> Self {
        Self {
            name: None,
            parameters,
            body,
            is_async,
            fn_type,
            decorators: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_decorators(mut self, decorators: Vec<DecoratorDecl>) -> Self {
        self.decorators = decorators;
        self
    }

    pub fn to_string(&self) -> String {
        match &self.name {
            Some(name) => {
                if self.is_async {
                    format!("[async fn {}]", name)
                } else {
                    format!("[Function {}]", name)
                }
            }
            None => {
                if self.is_async {
                    "[async fn]".to_string()
                } else {
                    "[Function]".to_string()
                }
            }
        }
    }
}

impl crate::runtime::dynamic::DynamicValue for FunctionValue {
    fn get_type(&self) -> Type {
        self.fn_type.clone()
    }

    fn to_string(&self) -> String {
        FunctionValue::to_string(self)
    }

    fn clone_boxed(&self) -> Box<dyn crate::runtime::dynamic::DynamicValue> {
        Box::new(self.clone())
    }

    fn type_name(&self) -> &str {
        if self.is_async {
            "async function"
        } else {
            "function"
        }
    }
}

pub type NativeFn = fn(Vec<RuntimeValue>) -> RuntimeValue;

#[derive(Clone)]
pub struct NativeFunctionValue {
    pub implementation: NativeFn,
    pub fn_type: Type,
}

impl NativeFunctionValue {
    pub fn new(implementation: NativeFn, fn_type: Type) -> Self {
        Self {
            implementation,
            fn_type,
        }
    }

    pub fn to_string(&self) -> String {
        "[Native Function]".to_string()
    }
}

impl fmt::Debug for NativeFunctionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeFunctionValue")
            .field("fn_type", &self.fn_type)
            .finish()
    }
}

pub type NativeAsyncFn = Arc<
    dyn Fn(Vec<RuntimeValue>) -> Pin<Box<dyn Future<Output = RuntimeValue> + 'static>>
        + Send
        + Sync
        + 'static,
>;

#[derive(Clone)]
pub struct NativeAsyncFunctionValue {
    pub implementation: NativeAsyncFn,
    pub fn_type: Type,
}

impl NativeAsyncFunctionValue {
    pub fn new(implementation: NativeAsyncFn, fn_type: Type) -> Self {
        Self {
            implementation,
            fn_type,
        }
    }

    pub fn to_string(&self) -> String {
        "[Native Async Function]".to_string()
    }
}

impl fmt::Debug for NativeAsyncFunctionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeAsyncFunctionValue")
            .field("fn_type", &self.fn_type)
            .finish()
    }
}

impl crate::runtime::dynamic::DynamicValue for NativeAsyncFunctionValue {
    fn get_type(&self) -> Type {
        self.fn_type.clone()
    }

    fn to_string(&self) -> String {
        NativeAsyncFunctionValue::to_string(self)
    }

    fn clone_boxed(&self) -> Box<dyn crate::runtime::dynamic::DynamicValue> {
        Box::new(self.clone())
    }

    fn type_name(&self) -> &str {
        "native async function"
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub enum_name: String,
    pub member_name: String,
    pub value: EnumValueData,
    pub enum_type: Type,
}

#[derive(Debug, Clone)]
pub enum EnumValueData {
    Int(i64),
    Str(String),
}

impl EnumValue {
    pub fn new(
        enum_name: String,
        member_name: String,
        value: EnumValueData,
        enum_type: Type,
    ) -> Self {
        Self {
            enum_name,
            member_name,
            value,
            enum_type,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.value {
            EnumValueData::Int(v) => v.to_string(),
            EnumValueData::Str(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimitiveTypeObject {
    pub type_name: String,
    pub static_methods: HashMap<String, Box<NativeFunctionValue>>,
    pub static_properties: HashMap<String, RuntimeValue>,
    pub type_obj: Type,
}

impl PrimitiveTypeObject {
    pub fn new(
        type_name: String,
        static_methods: HashMap<String, Box<NativeFunctionValue>>,
        static_properties: HashMap<String, RuntimeValue>,
        type_obj: Type,
    ) -> Self {
        Self {
            type_name,
            static_methods,
            static_properties,
            type_obj,
        }
    }

    pub fn to_string(&self) -> String {
        format!("<type {}>", self.type_name)
    }
}

#[derive(Debug, Clone)]
pub struct EnumObject {
    pub enum_name: String,
    pub members: HashMap<String, EnumValueData>,
    pub enum_type: Type,
}

impl EnumObject {
    pub fn new(
        enum_name: String,
        members: HashMap<String, EnumValueData>,
        enum_type: Type,
    ) -> Self {
        Self {
            enum_name,
            members,
            enum_type,
        }
    }

    pub fn to_string(&self) -> String {
        format!("<enum {}>", self.enum_name)
    }
}

#[derive(Clone)]
pub struct FutureValue {
    pub state: Arc<RwLock<FutureState>>,
    pub value_type: Type,
    pub notifier: Arc<Notify>,
}

impl fmt::Debug for FutureValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FutureValue")
            .field("state", &self.state)
            .field("value_type", &self.value_type)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum FutureState {
    Pending,
    Resolved(Box<RuntimeValue>),
    Rejected(String),
}

impl FutureValue {
    pub fn new(value_type: Type) -> Self {
        Self {
            state: Arc::new(RwLock::new(FutureState::Pending)),
            value_type,
            notifier: Arc::new(Notify::new()),
        }
    }

    pub fn new_resolved(value: RuntimeValue, value_type: Type) -> Self {
        let notifier = Arc::new(Notify::new());
        notifier.notify_waiters(); // Notify immediately since it's already resolved
        Self {
            state: Arc::new(RwLock::new(FutureState::Resolved(Box::new(value)))),
            value_type,
            notifier,
        }
    }

    pub fn new_rejected(error: String, value_type: Type) -> Self {
        let notifier = Arc::new(Notify::new());
        notifier.notify_waiters(); // Notify immediately since it's already rejected
        Self {
            state: Arc::new(RwLock::new(FutureState::Rejected(error))),
            value_type,
            notifier,
        }
    }

    pub fn resolve(&self, value: RuntimeValue) {
        *self.state.write().unwrap() = FutureState::Resolved(Box::new(value));
        self.notifier.notify_waiters();
    }

    pub fn reject(&self, error: String) {
        *self.state.write().unwrap() = FutureState::Rejected(error);
        self.notifier.notify_waiters();
    }

    pub async fn wait_for_completion(&self) -> Result<RuntimeValue, String> {
        loop {
            {
                let state = self.state.read().unwrap();
                match &*state {
                    FutureState::Resolved(value) => return Ok((**value).clone()),
                    FutureState::Rejected(error) => return Err(error.clone()),
                    FutureState::Pending => {
                        // Continue waiting
                    }
                }
            } // Release lock before awaiting

            self.notifier.notified().await;
        }
    }

    pub fn is_resolved(&self) -> bool {
        matches!(*self.state.read().unwrap(), FutureState::Resolved(_))
    }

    pub fn is_rejected(&self) -> bool {
        matches!(*self.state.read().unwrap(), FutureState::Rejected(_))
    }

    pub fn is_pending(&self) -> bool {
        matches!(*self.state.read().unwrap(), FutureState::Pending)
    }

    pub fn to_string(&self) -> String {
        match &*self.state.read().unwrap() {
            FutureState::Pending => "[Future: Pending]".to_string(),
            FutureState::Resolved(value) => format!("[Future: Resolved({})]", value.to_string()),
            FutureState::Rejected(error) => format!("[Future: Rejected({})]", error),
        }
    }
}

impl crate::runtime::dynamic::DynamicValue for FutureValue {
    fn get_type(&self) -> Type {
        Type::Future(Box::new(FutureType {
            inner_type: self.value_type.clone(),
        }))
    }

    fn to_string(&self) -> String {
        FutureValue::to_string(self)
    }

    fn clone_boxed(&self) -> Box<dyn crate::runtime::dynamic::DynamicValue> {
        Box::new(self.clone())
    }

    fn type_name(&self) -> &str {
        "future"
    }
}
