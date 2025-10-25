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
    Never,
    Func,
    Symbol,
    List,
    Tuple,
    Map,
    Object,
    Nullable,
    Union,
    Intersection,
    Function,
    Interface,
    Class,
    Enum,
    Future,
    TypeRef,
    Generic,
    Mapped,
    Indexed,
    KeyOf,
    TypeOfType,
    PrimitiveTypeObject,
    ClassObject,
    Date,
    Regex,
    Error,
    Readonly,
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
    Tuple(Box<TupleType>),
    Map(Box<MapType>),
    Object(Box<ObjectType>),
    Nullable(Box<NullableType>),
    Union(Box<UnionType>),
    Intersection(Box<IntersectionType>),
    Function(Box<FunctionType>),
    Interface(Box<InterfaceType>),
    Class(Box<ClassType>),
    Enum(Box<EnumType>),
    Future(Box<FutureType>),
    TypeRef(TypeReference),
    TypeParam(TypeParameter),
    Generic(Box<GenericType>),
    Mapped(Box<MappedType>),
    Indexed(Box<IndexedAccessType>),
    KeyOf(Box<KeyOfType>),
    TypeOf(Box<TypeOfType>),
    Readonly(Box<ReadonlyType>),
    PrimitiveTypeObject(Box<PrimitiveTypeObjectType>),
    ClassObject(Box<ClassObjectType>),
}

impl Type {
    pub fn kind(&self) -> TypeKind {
        match self {
            Type::Primitive(t) => t.kind.clone(),
            Type::List(_) => TypeKind::List,
            Type::Tuple(_) => TypeKind::Tuple,
            Type::Map(_) => TypeKind::Map,
            Type::Object(_) => TypeKind::Object,
            Type::Nullable(_) => TypeKind::Nullable,
            Type::Union(_) => TypeKind::Union,
            Type::Intersection(_) => TypeKind::Intersection,
            Type::Function(_) => TypeKind::Function,
            Type::Interface(_) => TypeKind::Interface,
            Type::Class(_) => TypeKind::Class,
            Type::Enum(_) => TypeKind::Enum,
            Type::Future(_) => TypeKind::Future,
            Type::TypeRef(_) => TypeKind::TypeRef,
            Type::TypeParam(_) => TypeKind::Generic,
            Type::Generic(_) => TypeKind::Generic,
            Type::Mapped(_) => TypeKind::Mapped,
            Type::Indexed(_) => TypeKind::Indexed,
            Type::KeyOf(_) => TypeKind::KeyOf,
            Type::TypeOf(_) => TypeKind::TypeOfType,
            Type::Readonly(_) => TypeKind::Readonly,
            Type::PrimitiveTypeObject(_) => TypeKind::PrimitiveTypeObject,
            Type::ClassObject(_) => TypeKind::ClassObject,
        }
    }

    pub fn equals(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Primitive(a), Type::Primitive(b)) => a.equals(b),
            (Type::List(a), Type::List(b)) => a.element_type.equals(&b.element_type),
            (Type::Tuple(a), Type::Tuple(b)) => a.equals(b),
            (Type::Map(a), Type::Map(b)) => {
                a.key_type.equals(&b.key_type) && a.value_type.equals(&b.value_type)
            }
            (Type::Object(a), Type::Object(b)) => a.equals(b),
            (Type::Nullable(a), Type::Nullable(b)) => a.inner_type.equals(&b.inner_type),
            (Type::Union(a), Type::Union(b)) => a.equals(b),
            (Type::Intersection(a), Type::Intersection(b)) => a.equals(b),
            (Type::Function(a), Type::Function(b)) => a.equals(b),
            (Type::Interface(a), Type::Interface(b)) => a.name == b.name,
            (Type::Class(a), Type::Class(b)) => a.name == b.name,
            (Type::Enum(a), Type::Enum(b)) => a.name == b.name,
            (Type::TypeRef(a), Type::TypeRef(b)) => a.name == b.name,
            (Type::TypeParam(a), Type::TypeParam(b)) => a.equals(b),
            (Type::Readonly(a), Type::Readonly(b)) => a.inner_type.equals(&b.inner_type),
            _ => false,
        }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        if matches!(self.kind(), TypeKind::Never) {
            return true;
        }

        if matches!(target.kind(), TypeKind::Never) {
            return matches!(self.kind(), TypeKind::Never);
        }

        if let Type::Function(_) = self {
            if let Type::Primitive(target_prim) = target {
                if target_prim.kind == TypeKind::Func {
                    return true;
                }
            }
        }

        match self {
            Type::Primitive(p) => p.is_assignable_to(target),
            Type::List(l) => l.is_assignable_to(target),
            Type::Tuple(t) => t.is_assignable_to(target),
            Type::Object(o) => o.is_assignable_to(target),
            Type::Nullable(n) => n.is_assignable_to(target),
            Type::Union(u) => u.is_assignable_to(target),
            Type::Intersection(i) => i.is_assignable_to(target),
            Type::Readonly(r) => r.is_assignable_to(target),
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

        if let Type::Primitive(target_prim) = target {
            if target_prim.kind == TypeKind::Func && self.kind == TypeKind::Function {
                return true;
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

    pub fn func() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Func, "func"))
    }

    pub fn never() -> Type {
        Type::Primitive(PrimitiveType::new(TypeKind::Never, "never"))
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

#[derive(Debug, Clone, PartialEq)]
pub struct TupleType {
    pub element_types: Vec<Type>,
}

impl TupleType {
    pub fn new(element_types: Vec<Type>) -> Self {
        Self { element_types }
    }

    pub fn equals(&self, other: &TupleType) -> bool {
        if self.element_types.len() != other.element_types.len() {
            return false;
        }
        self.element_types
            .iter()
            .zip(&other.element_types)
            .all(|(a, b)| a.equals(b))
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        if let Type::Tuple(target_tuple) = target {
            if self.element_types.len() != target_tuple.element_types.len() {
                return false;
            }
            return self
                .element_types
                .iter()
                .zip(&target_tuple.element_types)
                .all(|(a, b)| a.is_assignable_to(b));
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub properties: HashMap<String, ObjectProperty>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectProperty {
    pub property_type: Type,
    pub optional: bool,
    pub readonly: bool,
}

impl ObjectProperty {
    pub fn new(property_type: Type) -> Self {
        Self {
            property_type,
            optional: false,
            readonly: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
}

impl ObjectType {
    pub fn new(properties: HashMap<String, ObjectProperty>) -> Self {
        Self { properties }
    }

    pub fn equals(&self, other: &ObjectType) -> bool {
        if self.properties.len() != other.properties.len() {
            return false;
        }

        for (key, prop) in &self.properties {
            if let Some(other_prop) = other.properties.get(key) {
                if !prop.property_type.equals(&other_prop.property_type)
                    || prop.optional != other_prop.optional
                    || prop.readonly != other_prop.readonly
                {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        if let Type::Object(target_obj) = target {
            for (key, target_prop) in &target_obj.properties {
                if let Some(self_prop) = self.properties.get(key) {
                    if !self_prop
                        .property_type
                        .is_assignable_to(&target_prop.property_type)
                    {
                        return false;
                    }
                } else if !target_prop.optional {
                    return false;
                }
            }
            return true;
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntersectionType {
    pub types: Vec<Type>,
}

impl IntersectionType {
    pub fn new(types: Vec<Type>) -> Self {
        Self {
            types: Self::flatten_intersections(types),
        }
    }

    fn flatten_intersections(types: Vec<Type>) -> Vec<Type> {
        let mut flattened = Vec::new();
        for t in types {
            if let Type::Intersection(intersection) = t {
                flattened.extend(intersection.types);
            } else {
                flattened.push(t);
            }
        }
        flattened
    }

    pub fn equals(&self, other: &IntersectionType) -> bool {
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

        self.types.iter().any(|t| t.is_assignable_to(target))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadonlyType {
    pub inner_type: Type,
}

impl ReadonlyType {
    pub fn new(inner_type: Type) -> Self {
        Self { inner_type }
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if matches!(target.kind(), TypeKind::Any | TypeKind::Unknown) {
            return true;
        }

        if let Type::Readonly(target_readonly) = target {
            return self
                .inner_type
                .is_assignable_to(&target_readonly.inner_type);
        }

        self.inner_type.is_assignable_to(target)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappedType {
    pub type_parameter: String,
    pub constraint: Box<Type>,
    pub value_type: Box<Type>,
    pub optional: bool,
    pub readonly: bool,
}

impl MappedType {
    pub fn new(type_parameter: String, constraint: Type, value_type: Type) -> Self {
        Self {
            type_parameter,
            constraint: Box::new(constraint),
            value_type: Box::new(value_type),
            optional: false,
            readonly: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexedAccessType {
    pub object_type: Box<Type>,
    pub index_type: Box<Type>,
}

impl IndexedAccessType {
    pub fn new(object_type: Type, index_type: Type) -> Self {
        Self {
            object_type: Box::new(object_type),
            index_type: Box::new(index_type),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyOfType {
    pub target_type: Box<Type>,
}

impl KeyOfType {
    pub fn new(target_type: Type) -> Self {
        Self {
            target_type: Box::new(target_type),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeOfType {
    pub expression_name: String,
}

impl TypeOfType {
    pub fn new(expression_name: String) -> Self {
        Self { expression_name }
    }
}
