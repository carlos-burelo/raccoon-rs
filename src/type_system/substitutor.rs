use crate::ast::types::*;
use std::collections::HashMap;

pub struct TypeSubstitutor {
    substitutions: HashMap<String, Type>,
}

impl TypeSubstitutor {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
        }
    }

    pub fn from_substitutions(substitutions: HashMap<String, Type>) -> Self {
        Self { substitutions }
    }

    pub fn from_type_args(type_parameters: &[TypeParameter], type_arguments: &[Type]) -> Self {
        let mut substitutions = HashMap::new();

        for (param, arg) in type_parameters.iter().zip(type_arguments.iter()) {
            substitutions.insert(param.name.clone(), arg.clone());
        }

        Self { substitutions }
    }

    pub fn add_substitution(&mut self, type_param_name: String, concrete_type: Type) {
        self.substitutions.insert(type_param_name, concrete_type);
    }

    pub fn substitute(&self, type_: &Type) -> Type {
        match type_ {
            Type::TypeParam(type_param) => {
                if let Some(substitution) = self.substitutions.get(&type_param.name) {
                    substitution.clone()
                } else {
                    type_.clone()
                }
            }

            Type::Array(list_type) => Type::Array(Box::new(ArrayType {
                element_type: self.substitute(&list_type.element_type),
            })),

            Type::Map(map_type) => Type::Map(Box::new(MapType {
                key_type: self.substitute(&map_type.key_type),
                value_type: self.substitute(&map_type.value_type),
            })),

            Type::Nullable(nullable_type) => Type::Nullable(Box::new(NullableType {
                inner_type: self.substitute(&nullable_type.inner_type),
            })),

            Type::Union(union_type) => {
                let substituted_types: Vec<Type> = union_type
                    .types
                    .iter()
                    .map(|t| self.substitute(t))
                    .collect();

                Type::Union(Box::new(UnionType::new(substituted_types)))
            }

            Type::Function(fn_type) => {
                let substituted_params: Vec<Type> =
                    fn_type.params.iter().map(|p| self.substitute(p)).collect();

                Type::Function(Box::new(FunctionType {
                    params: substituted_params,
                    return_type: self.substitute(&fn_type.return_type),
                    is_variadic: fn_type.is_variadic,
                }))
            }

            Type::Future(future_type) => Type::Future(Box::new(FutureType {
                inner_type: self.substitute(&future_type.inner_type),
            })),

            Type::Generic(generic_type) => {
                let substituted_base = self.substitute(&generic_type.base);
                let substituted_args: Vec<Type> = generic_type
                    .type_args
                    .iter()
                    .map(|arg| self.substitute(arg))
                    .collect();

                Type::Generic(Box::new(GenericType {
                    base: substituted_base,
                    type_args: substituted_args,
                }))
            }

            Type::Class(class_type) if !class_type.type_parameters.is_empty() => {
                let mut new_properties = HashMap::new();
                for (name, info) in &class_type.properties {
                    new_properties.insert(
                        name.clone(),
                        ClassPropertyInfo {
                            property_type: self.substitute(&info.property_type),
                            access_modifier: info.access_modifier,
                        },
                    );
                }

                let mut new_methods = HashMap::new();
                for (name, info) in &class_type.methods {
                    if let Type::Function(fn_type) =
                        self.substitute(&Type::Function(Box::new(info.method_type.clone())))
                    {
                        new_methods.insert(
                            name.clone(),
                            ClassMethodInfo {
                                method_type: *fn_type,
                                access_modifier: info.access_modifier,
                                is_static: info.is_static,
                            },
                        );
                    }
                }

                let new_constructor = class_type.constructor.as_ref().map(|ctor| {
                    if let Type::Function(fn_type) =
                        self.substitute(&Type::Function(Box::new(ctor.clone())))
                    {
                        *fn_type
                    } else {
                        ctor.clone()
                    }
                });

                Type::Class(Box::new(ClassType {
                    name: class_type.name.clone(),
                    superclass: class_type.superclass.clone(),
                    properties: new_properties,
                    methods: new_methods,
                    constructor: new_constructor,
                    type_parameters: Vec::new(),
                }))
            }

            Type::Interface(interface_type) if !interface_type.type_parameters.is_empty() => {
                let mut new_properties = HashMap::new();
                for (name, info) in &interface_type.properties {
                    new_properties.insert(
                        name.clone(),
                        InterfaceProperty {
                            property_type: self.substitute(&info.property_type),
                            optional: info.optional,
                        },
                    );
                }

                Type::Interface(Box::new(InterfaceType {
                    name: interface_type.name.clone(),
                    properties: new_properties,
                    type_parameters: Vec::new(),
                }))
            }

            _ => type_.clone(),
        }
    }
}

impl Default for TypeSubstitutor {
    fn default() -> Self {
        Self::new()
    }
}
