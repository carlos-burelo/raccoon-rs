use crate::ast::types::Type;
use crate::runtime::type_object::{TypeObject, TypeKind, TypeMetadata, SourceLocation};
use crate::runtime::values::RuntimeValue;
use std::collections::HashMap;

/// Builder para crear TypeObjects de manera consistente
pub struct TypeObjectBuilder {
    type_def: Type,
    kind: TypeKind,
    static_methods: HashMap<String, RuntimeValue>,
    static_properties: HashMap<String, RuntimeValue>,
    constructor: Option<Box<RuntimeValue>>,
    metadata: TypeMetadata,
}

impl TypeObjectBuilder {
    /// Crea un nuevo builder con un tipo y kind
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

    /// Agrega un método estático
    pub fn static_method(mut self, name: impl Into<String>, func: RuntimeValue) -> Self {
        self.static_methods.insert(name.into(), func);
        self
    }

    /// Agrega múltiples métodos estáticos
    pub fn static_methods(mut self, methods: HashMap<String, RuntimeValue>) -> Self {
        self.static_methods.extend(methods);
        self
    }

    /// Agrega una propiedad estática
    pub fn static_property(mut self, name: impl Into<String>, value: RuntimeValue) -> Self {
        self.static_properties.insert(name.into(), value);
        self
    }

    /// Agrega múltiples propiedades estáticas
    pub fn static_properties(mut self, properties: HashMap<String, RuntimeValue>) -> Self {
        self.static_properties.extend(properties);
        self
    }

    /// Establece el constructor
    pub fn constructor(mut self, func: RuntimeValue) -> Self {
        self.constructor = Some(Box::new(func));
        self
    }

    /// Agrega documentación
    pub fn documentation(mut self, doc: impl Into<String>) -> Self {
        self.metadata.documentation = Some(doc.into());
        self
    }

    /// Agrega ubicación en el código fuente
    pub fn source_location(mut self, file: String, line: usize, column: usize) -> Self {
        self.metadata.source_location = Some(SourceLocation::new(file, line, column));
        self
    }

    /// Agrega un decorador
    pub fn decorator(mut self, decorator: impl Into<String>) -> Self {
        self.metadata.decorators.push(decorator.into());
        self
    }

    /// Agrega múltiples decoradores
    pub fn decorators(mut self, decorators: Vec<String>) -> Self {
        self.metadata.decorators.extend(decorators);
        self
    }

    /// Establece metadata completa
    pub fn metadata(mut self, metadata: TypeMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Construye el TypeObject
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::PrimitiveType;
    use crate::runtime::type_object::PrimitiveKind;
    use crate::runtime::values::{IntValue, NullValue};

    #[test]
    fn test_builder_basic() {
        let type_obj = TypeObjectBuilder::new(
            PrimitiveType::int(),
            TypeKind::Primitive(PrimitiveKind::Int),
        )
        .documentation("Integer type")
        .build();

        assert_eq!(type_obj.name(), "int");
        assert_eq!(type_obj.metadata.documentation, Some("Integer type".to_string()));
    }

    #[test]
    fn test_builder_with_static_method() {
        let type_obj = TypeObjectBuilder::new(
            PrimitiveType::int(),
            TypeKind::Primitive(PrimitiveKind::Int),
        )
        .static_method("parse", RuntimeValue::Null(NullValue))
        .static_property("MAX_VALUE", RuntimeValue::Int(IntValue::new(i64::MAX)))
        .build();

        assert!(type_obj.has_static_method("parse"));
        assert!(type_obj.has_static_property("MAX_VALUE"));
    }

    #[test]
    fn test_builder_with_decorators() {
        let type_obj = TypeObjectBuilder::new(
            PrimitiveType::int(),
            TypeKind::Primitive(PrimitiveKind::Int),
        )
        .decorator("@sealed")
        .decorator("@deprecated")
        .build();

        assert_eq!(type_obj.metadata.decorators.len(), 2);
        assert!(type_obj.metadata.decorators.contains(&"@sealed".to_string()));
    }
}
