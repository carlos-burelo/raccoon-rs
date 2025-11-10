/// Metadata system for type reflection
/// Provides structured information about types, methods, and properties
use std::collections::HashMap;

/// Parameter metadata for method signatures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamMetadata {
    pub name: &'static str,
    pub type_name: &'static str,
    pub optional: bool,
    pub variadic: bool,
}

impl ParamMetadata {
    pub const fn new(name: &'static str, type_name: &'static str) -> Self {
        Self {
            name,
            type_name,
            optional: false,
            variadic: false,
        }
    }

    pub const fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub const fn variadic(mut self) -> Self {
        self.variadic = true;
        self
    }
}

/// Method metadata with signature information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodMetadata {
    pub name: &'static str,
    pub params: Vec<ParamMetadata>,
    pub return_type: &'static str,
    pub description: &'static str,
    pub is_async: bool,
    pub aliases: Vec<&'static str>,
}

impl MethodMetadata {
    pub const fn new(
        name: &'static str,
        return_type: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            params: Vec::new(),
            return_type,
            description,
            is_async: false,
            aliases: Vec::new(),
        }
    }

    pub fn with_params(mut self, params: Vec<ParamMetadata>) -> Self {
        self.params = params;
        self
    }

    pub fn with_alias(mut self, alias: &'static str) -> Self {
        self.aliases.push(alias);
        self
    }

    pub fn with_aliases(mut self, aliases: Vec<&'static str>) -> Self {
        self.aliases = aliases;
        self
    }

    pub const fn async_method(mut self) -> Self {
        self.is_async = true;
        self
    }

    /// Check if this method or any of its aliases matches the given name
    pub fn matches(&self, name: &str) -> bool {
        self.name == name || self.aliases.iter().any(|&alias| alias == name)
    }
}

/// Property metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyMetadata {
    pub name: &'static str,
    pub type_name: &'static str,
    pub description: &'static str,
    pub readonly: bool,
}

impl PropertyMetadata {
    pub const fn new(
        name: &'static str,
        type_name: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            type_name,
            description,
            readonly: false,
        }
    }

    pub const fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
}

/// Complete type metadata
#[derive(Debug, Clone)]
pub struct TypeMetadata {
    pub type_name: &'static str,
    pub description: &'static str,
    pub instance_methods: Vec<MethodMetadata>,
    pub static_methods: Vec<MethodMetadata>,
    pub static_properties: Vec<PropertyMetadata>,
    pub parent_type: Option<&'static str>,
    pub interfaces: Vec<&'static str>,
}

impl TypeMetadata {
    pub const fn new(type_name: &'static str, description: &'static str) -> Self {
        Self {
            type_name,
            description,
            instance_methods: Vec::new(),
            static_methods: Vec::new(),
            static_properties: Vec::new(),
            parent_type: None,
            interfaces: Vec::new(),
        }
    }

    pub fn with_instance_methods(mut self, methods: Vec<MethodMetadata>) -> Self {
        self.instance_methods = methods;
        self
    }

    pub fn with_static_methods(mut self, methods: Vec<MethodMetadata>) -> Self {
        self.static_methods = methods;
        self
    }

    pub fn with_static_properties(mut self, properties: Vec<PropertyMetadata>) -> Self {
        self.static_properties = properties;
        self
    }

    pub const fn with_parent(mut self, parent: &'static str) -> Self {
        self.parent_type = Some(parent);
        self
    }

    /// Find instance method by name (including aliases)
    pub fn find_instance_method(&self, name: &str) -> Option<&MethodMetadata> {
        self.instance_methods.iter().find(|m| m.matches(name))
    }

    /// Find static method by name (including aliases)
    pub fn find_static_method(&self, name: &str) -> Option<&MethodMetadata> {
        self.static_methods.iter().find(|m| m.matches(name))
    }

    /// Find static property by name
    pub fn find_static_property(&self, name: &str) -> Option<&PropertyMetadata> {
        self.static_properties.iter().find(|p| p.name == name)
    }

    /// Check if instance method exists (including aliases)
    pub fn has_instance_method(&self, name: &str) -> bool {
        self.find_instance_method(name).is_some()
    }

    /// Check if static method exists (including aliases)
    pub fn has_static_method(&self, name: &str) -> bool {
        self.find_static_method(name).is_some()
    }

    /// Check if async instance method exists
    pub fn has_async_instance_method(&self, name: &str) -> bool {
        self.find_instance_method(name)
            .map(|m| m.is_async)
            .unwrap_or(false)
    }

    /// Generate documentation string for this type
    pub fn generate_docs(&self) -> String {
        let mut docs = format!("# {}\n\n{}\n\n", self.type_name, self.description);

        if !self.instance_methods.is_empty() {
            docs.push_str("## Instance Methods\n\n");
            for method in &self.instance_methods {
                let params: Vec<String> = method
                    .params
                    .iter()
                    .map(|p| {
                        let mut param_str = format!("{}: {}", p.name, p.type_name);
                        if p.optional {
                            param_str.push('?');
                        }
                        if p.variadic {
                            param_str = format!("...{}", param_str);
                        }
                        param_str
                    })
                    .collect();

                docs.push_str(&format!(
                    "- `{}({})`: {} → {}\n",
                    method.name,
                    params.join(", "),
                    method.description,
                    method.return_type
                ));

                if !method.aliases.is_empty() {
                    docs.push_str(&format!("  Aliases: {}\n", method.aliases.join(", ")));
                }
            }
            docs.push('\n');
        }

        if !self.static_methods.is_empty() {
            docs.push_str("## Static Methods\n\n");
            for method in &self.static_methods {
                let params: Vec<String> = method
                    .params
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.type_name))
                    .collect();

                docs.push_str(&format!(
                    "- `{}({})`: {} → {}\n",
                    method.name,
                    params.join(", "),
                    method.description,
                    method.return_type
                ));
            }
            docs.push('\n');
        }

        if !self.static_properties.is_empty() {
            docs.push_str("## Static Properties\n\n");
            for prop in &self.static_properties {
                docs.push_str(&format!(
                    "- `{}`: {} - {}\n",
                    prop.name, prop.type_name, prop.description
                ));
            }
            docs.push('\n');
        }

        docs
    }
}

/// Global registry for type metadata
pub struct MetadataRegistry {
    types: HashMap<&'static str, TypeMetadata>,
}

impl MetadataRegistry {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub fn register(&mut self, metadata: TypeMetadata) {
        self.types.insert(metadata.type_name, metadata);
    }

    pub fn get(&self, type_name: &str) -> Option<&TypeMetadata> {
        self.types.get(type_name)
    }

    pub fn list_types(&self) -> Vec<&'static str> {
        self.types.keys().copied().collect()
    }

    pub fn generate_all_docs(&self) -> String {
        let mut all_docs = String::from("# Type System Documentation\n\n");

        let mut type_names: Vec<_> = self.types.keys().collect();
        type_names.sort();

        for type_name in type_names {
            if let Some(metadata) = self.types.get(type_name) {
                all_docs.push_str(&metadata.generate_docs());
                all_docs.push_str("---\n\n");
            }
        }

        all_docs
    }
}

impl Default for MetadataRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_metadata() {
        let method = MethodMetadata::new("split", "list<str>", "Splits a string")
            .with_params(vec![ParamMetadata::new("separator", "str")])
            .with_alias("splitBy");

        assert!(method.matches("split"));
        assert!(method.matches("splitBy"));
        assert!(!method.matches("join"));
    }

    #[test]
    fn test_type_metadata() {
        let metadata = TypeMetadata::new("str", "String type").with_instance_methods(vec![
            MethodMetadata::new("toUpper", "str", "Convert to uppercase"),
            MethodMetadata::new("toLower", "str", "Convert to lowercase"),
        ]);

        assert!(metadata.has_instance_method("toUpper"));
        assert!(metadata.has_instance_method("toLower"));
        assert!(!metadata.has_instance_method("invalid"));
    }

    #[test]
    fn test_generate_docs() {
        let metadata = TypeMetadata::new("str", "String type").with_instance_methods(vec![
            MethodMetadata::new("split", "list<str>", "Splits a string")
                .with_params(vec![ParamMetadata::new("separator", "str")]),
        ]);

        let docs = metadata.generate_docs();
        assert!(docs.contains("# str"));
        assert!(docs.contains("split"));
    }
}
