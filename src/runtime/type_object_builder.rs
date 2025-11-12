use crate::ast::types::Type;
use crate::runtime::type_object::{SourceLocation, TypeKind, TypeMetadata, TypeObject};
use crate::runtime::values::RuntimeValue;
use std::collections::HashMap;

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
            metadata: TypeMetadata::new(),
        }
    }

    pub fn static_method(mut self, name: impl Into<String>, func: RuntimeValue) -> Self {
        self.static_methods.insert(name.into(), func);
        self
    }

    pub fn static_methods(mut self, methods: HashMap<String, RuntimeValue>) -> Self {
        self.static_methods.extend(methods);
        self
    }

    pub fn static_property(mut self, name: impl Into<String>, value: RuntimeValue) -> Self {
        self.static_properties.insert(name.into(), value);
        self
    }

    pub fn static_properties(mut self, properties: HashMap<String, RuntimeValue>) -> Self {
        self.static_properties.extend(properties);
        self
    }

    pub fn constructor(mut self, func: RuntimeValue) -> Self {
        self.constructor = Some(Box::new(func));
        self
    }

    pub fn documentation(mut self, doc: impl Into<String>) -> Self {
        self.metadata.documentation = Some(doc.into());
        self
    }

    pub fn source_location(mut self, file: String, line: usize, column: usize) -> Self {
        self.metadata.source_location = Some(SourceLocation::new(file, line, column));
        self
    }

    pub fn decorator(mut self, decorator: impl Into<String>) -> Self {
        self.metadata.decorators.push(decorator.into());
        self
    }

    pub fn decorators(mut self, decorators: Vec<String>) -> Self {
        self.metadata.decorators.extend(decorators);
        self
    }

    pub fn metadata(mut self, metadata: TypeMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn build(self) -> TypeObject {
        TypeObject::new(
            self.type_def,
            self.kind,
            self.static_methods,
            self.static_properties,
            self.constructor,
            self.metadata,
        )
    }
}
