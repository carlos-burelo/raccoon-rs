use crate::{error::RaccoonError, tokens::AccessModifier};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Int,
    Float,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Decimal,
    Str,
    Bool,
    Null,
    Void,
    Any,
    Unknown,
    Symbol,
    List,
    Map,
    Nullable,
    Union,
    Function,
    Interface,
    Class,
    Enum,
    Future,
    TypeRef,
    Generic,
    PrimitiveTypeObject,
    ClassObject,
    Date,
    Regex,
    Error,
}

pub trait TypeTrait {
    fn kind(&self) -> TypeKind;
    fn equals(&self, other: &Type) -> bool;
    fn is_assignable_to(&self, target: &Type) -> bool;
    fn to_string(&self) -> String;
    fn is_nullable(&self) -> bool {
        matches!(self.kind(), TypeKind::Nullable | TypeKind::Union)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    List(Box<ListType>),
    Map(Box<MapType>),
    Nullable(Box<NullableType>),
    Union(Box<UnionType>),
    Function(Box<FunctionType>),
    Interface(Box<InterfaceType>),
    Class(Box<ClassType>),
    Enum(Box<EnumType>),
    Future(Box<FutureType>),
    TypeRef(TypeReference),
    TypeParam(TypeParameter),
    Generic(Box<GenericType>),
    PrimitiveTypeObject(Box<PrimitiveTypeObjectType>),
    ClassObject(Box<ClassObjectType>),
}

impl Type {
    pub fn kind(&self) -> TypeKind {
        match self {
            Type::Primitive(t) => t.kind.clone(),
            Type::List(_) => TypeKind::List,
            Type::Map(_) => TypeKind::Map,
            Type::Nullable(_) => TypeKind::Nullable,
            Type::Union(_) => TypeKind::Union,
            Type::Function(_) => TypeKind::Function,
            Type::Interface(_) => TypeKind::Interface,
            Type::Class(_) => TypeKind::Class,
            Type::Enum(_) => TypeKind::Enum,
            Type::Future(_) => TypeKind::Future,
            Type::TypeRef(_) => TypeKind::TypeRef,
            Type::TypeParam(_) => TypeKind::Generic,
            Type::Generic(_) => TypeKind::Generic,
            Type::PrimitiveTypeObject(_) => TypeKind::PrimitiveTypeObject,
            Type::ClassObject(_) => TypeKind::ClassObject,
        }
    }

    pub fn equals(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Primitive(a), Type::Primitive(b)) => a.equals(b),
            (Type::List(a), Type::List(b)) => a.element_type.equals(&b.element_type),
            (Type::Map(a), Type::Map(b)) => {
                a.key_type.equals(&b.key_type) && a.value_type.equals(&b.value_type)
            }
            (Type::Nullable(a), Type::Nullable(b)) => a.inner_type.equals(&b.inner_type),
            (Type::Union(a), Type::Union(b)) => a.equals(b),
            (Type::Function(a), Type::Function(b)) => a.equals(b),
            (Type::TypeRef(a), Type::TypeRef(b)) => a.name == b.name,
            (Type::TypeParam(a), Type::TypeParam(b)) => a.equals(b),
            _ => false,
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        match self {
            Type::Primitive(p) => p.is_assignable_to(target),
            Type::List(l) => l.is_assignable_to(target),
            Type::Nullable(n) => n.is_assignable_to(target),
            Type::Union(u) => u.is_assignable_to(target),
            _ => self.equals(target),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveType {
    pub kind: TypeKind,
    pub name: String,
}

impl PrimitiveType {
    pub fn new(kind: TypeKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
        }
    }

    pub fn equals(&self, other: &PrimitiveType) -> bool {
        self.kind == other.kind
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        if let Type::Primitive(target_prim) = target {
            if self.equals(target_prim) {
                return true;
            }

            if self.kind == TypeKind::Int && target_prim.kind == TypeKind::Float {
                return true;
            }

            if self.is_numeric() && target_prim.is_numeric() {
                return self.can_widen_to(target_prim);
            }
        }

        if let Type::Nullable(nullable) = target {
            return self.is_assignable_to(&nullable.inner_type);
        }

        if let Type::Union(union) = target {
            return union.types.iter().any(|t| self.is_assignable_to(t));
        }

        false
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self.kind,
            TypeKind::Int
                | TypeKind::Float
                | TypeKind::I8
                | TypeKind::I16
                | TypeKind::I32
                | TypeKind::I64
                | TypeKind::U8
                | TypeKind::U16
                | TypeKind::U32
                | TypeKind::U64
                | TypeKind::F32
                | TypeKind::F64
                | TypeKind::Decimal
        )
    }

    pub fn can_widen_to(&self, target: &PrimitiveType) -> bool {
        let widening_rules: HashMap<TypeKind, Vec<TypeKind>> = HashMap::from([
            (
                TypeKind::I8,
                vec![
                    TypeKind::I16,
                    TypeKind::I32,
                    TypeKind::I64,
                    TypeKind::Int,
                    TypeKind::Float,
                ],
            ),
            (
                TypeKind::I16,
                vec![TypeKind::I32, TypeKind::I64, TypeKind::Int, TypeKind::Float],
            ),
            (
                TypeKind::I32,
                vec![TypeKind::I64, TypeKind::Int, TypeKind::Float],
            ),
            (TypeKind::Int, vec![TypeKind::Float, TypeKind::I64]),
        ]);

        widening_rules
            .get(&self.kind)
            .map(|targets| targets.contains(&target.kind))
            .unwrap_or(false)
    }

    pub fn int() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Int, "int"))
    }

    pub fn float() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Float, "float"))
    }

    pub fn decimal() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Decimal, "decimal"))
    }

    pub fn str() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Str, "str"))
    }

    pub fn bool() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Bool, "bool"))
    }

    pub fn null() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Null, "null"))
    }

    pub fn void() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Void, "void"))
    }

    pub fn any() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Any, "any"))
    }

    pub fn unknown() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Unknown, "unknown"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListType {
    pub element_type: Type,
}

impl ListType {
    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        if let Type::List(target_list) = target {
            return self
                .element_type
                .is_assignable_to(&target_list.element_type);
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapType {
    pub key_type: Type,
    pub value_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullableType {
    pub inner_type: Type,
}

impl NullableType {
    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        if let Type::Nullable(target_nullable) = target {
            return self
                .inner_type
                .is_assignable_to(&target_nullable.inner_type);
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionType {
    pub types: Vec<Type>,
}

impl UnionType {
    pub fn new(types: Vec<Type>) -> Self {
        Self {
            types: Self::flatten_unions(types),
        }
    }

    fn flatten_unions(types: Vec<Type>) -> Vec<Type> {
        let mut flattened = Vec::new();
        for t in types {
            if let Type::Union(union) = t {
                flattened.extend(union.types);
            } else {
                flattened.push(t);
            }
        }
        flattened
    }

    pub fn equals(&self, other: &UnionType) -> bool {
        if self.types.len() != other.types.len() {
            return false;
        }
        self.types
            .iter()
            .all(|t| other.types.iter().any(|ot| t.equals(ot)))
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        self.types.iter().all(|t| t.is_assignable_to(target))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub return_type: Type,
    pub is_variadic: bool,
}

impl FunctionType {
    pub fn equals(&self, other: &FunctionType) -> bool {
        if self.params.len() != other.params.len() {
            return false;
        }
        if !self.return_type.equals(&other.return_type) {
            return false;
        }
        self.params
            .iter()
            .zip(&other.params)
            .all(|(a, b)| a.equals(b))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceType {
    pub name: String,
    pub properties: HashMap<String, InterfaceProperty>,
    pub type_parameters: Vec<TypeParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceProperty {
    pub property_type: Type,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassType {
    pub name: String,
    pub superclass: Option<Box<ClassType>>,
    pub properties: HashMap<String, ClassPropertyInfo>,
    pub methods: HashMap<String, ClassMethodInfo>,
    pub constructor: Option<FunctionType>,
    pub type_parameters: Vec<TypeParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassPropertyInfo {
    pub property_type: Type,
    pub access_modifier: AccessModifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethodInfo {
    pub method_type: FunctionType,
    pub access_modifier: AccessModifier,
    pub is_static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType {
    pub name: String,
    pub members: HashMap<String, EnumValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
    Int(i64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FutureType {
    pub inner_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub name: String,
    pub file: Option<String>,
}

impl TypeReference {
    pub fn is_assignable_to(
        &self,
        _target: &Type,
        file: Option<String>,
    ) -> Result<bool, RaccoonError> {
        Err(RaccoonError::new(
            format!(
                "Cannot check assignability of unresolved type reference: {}",
                self.name
            ),
            (0, 0),
            file,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    pub name: String,
    pub constraint: Option<Box<Type>>,
}

impl TypeParameter {
    pub fn equals(&self, other: &TypeParameter) -> bool {
        if self.name != other.name {
            return false;
        }
        match (&self.constraint, &other.constraint) {
            (Some(a), Some(b)) => a.equals(b),
            (None, None) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericType {
    pub base: Type,
    pub type_args: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveTypeObjectType {
    pub primitive_type: PrimitiveType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassObjectType {
    pub class_type: ClassType,
}
