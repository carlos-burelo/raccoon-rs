use crate::ast::types::PrimitiveType;
use crate::runtime::types::TypeHandler;
use crate::runtime::values::{BoolValue, ListValue, NullValue, RuntimeValue, StrValue};
use crate::{Position, RaccoonError};
use async_trait::async_trait;

pub struct TypeType;

impl TypeType {
    pub fn new() -> Self {
        TypeType
    }
}

#[async_trait]
impl TypeHandler for TypeType {
    fn type_name(&self) -> &str {
        "Type"
    }

    fn call_instance_method(
        &self,
        value: &mut RuntimeValue,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        if let RuntimeValue::Type(type_obj) = value {
            match method {
                "name" => Ok(RuntimeValue::Str(StrValue::new(type_obj.name()))),
                "kind" => Ok(RuntimeValue::Str(StrValue::new(
                    type_obj.get_kind().kind_name().to_string(),
                ))),
                "isPrimitive" => Ok(RuntimeValue::Bool(BoolValue::new(type_obj.is_primitive()))),
                "isClass" => Ok(RuntimeValue::Bool(BoolValue::new(type_obj.is_class()))),
                "isInterface" => Ok(RuntimeValue::Bool(BoolValue::new(type_obj.is_interface()))),
                "isEnum" => Ok(RuntimeValue::Bool(BoolValue::new(type_obj.is_enum()))),
                "isFunction" => Ok(RuntimeValue::Bool(BoolValue::new(type_obj.is_function()))),
                "hasMethod" => {
                    if args.len() != 1 {
                        return Err(RaccoonError::new(
                            "hasMethod() requires 1 argument",
                            position,
                            file,
                        ));
                    }
                    let method_name = args[0].to_string();
                    Ok(RuntimeValue::Bool(BoolValue::new(
                        type_obj.has_static_method(&method_name),
                    )))
                }
                "hasProperty" => {
                    if args.len() != 1 {
                        return Err(RaccoonError::new(
                            "hasProperty() requires 1 argument",
                            position,
                            file,
                        ));
                    }
                    let prop_name = args[0].to_string();
                    Ok(RuntimeValue::Bool(BoolValue::new(
                        type_obj.has_static_property(&prop_name),
                    )))
                }
                "getMethods" => {
                    let methods = type_obj.get_all_static_methods();
                    let list = methods
                        .into_iter()
                        .map(|m| RuntimeValue::Str(StrValue::new(m)))
                        .collect();
                    Ok(RuntimeValue::List(ListValue::new(
                        list,
                        PrimitiveType::str(),
                    )))
                }
                "getProperties" => {
                    let properties = type_obj.get_all_static_properties();
                    let list = properties
                        .into_iter()
                        .map(|p| RuntimeValue::Str(StrValue::new(p)))
                        .collect();
                    Ok(RuntimeValue::List(ListValue::new(
                        list,
                        PrimitiveType::str(),
                    )))
                }
                "getDocumentation" => match type_obj.get_documentation() {
                    Some(doc) => Ok(RuntimeValue::Str(StrValue::new(doc.clone()))),
                    None => Ok(RuntimeValue::Null(NullValue::new())),
                },
                "toString" => Ok(RuntimeValue::Str(StrValue::new(format!(
                    "[Type {}]",
                    type_obj.name()
                )))),
                _ => Err(RaccoonError::new(
                    format!("Method '{}' not found on Type", method),
                    position,
                    file,
                )),
            }
        } else {
            Err(RaccoonError::new(
                format!("Cannot call method '{}' on non-Type value", method),
                position,
                file,
            ))
        }
    }

    fn call_static_method(
        &self,
        method: &str,
        args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        match method {
            "typeOf" => {
                if args.is_empty() {
                    return Err(RaccoonError::new(
                        "Type.typeOf() requires at least 1 argument",
                        position,
                        file,
                    ));
                }

                let value = &args[0];
                let type_obj = value.get_type_object();
                Ok(RuntimeValue::Type(type_obj))
            }
            "name" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "Type.name() requires 1 argument",
                        position,
                        file,
                    ));
                }

                if let RuntimeValue::Type(type_obj) = &args[0] {
                    Ok(RuntimeValue::Str(StrValue::new(type_obj.name())))
                } else {
                    Err(RaccoonError::new(
                        "Type.name() requires a Type argument",
                        position,
                        file,
                    ))
                }
            }
            "isInstance" => {
                if args.len() != 2 {
                    return Err(RaccoonError::new(
                        "Type.isInstance() requires 2 arguments: (value, type)",
                        position,
                        file,
                    ));
                }

                let value = &args[0];
                let value_type = value.get_type_object();

                if let RuntimeValue::Type(target_type) = &args[1] {
                    let is_instance = value_type.name() == target_type.name();
                    Ok(RuntimeValue::Bool(BoolValue::new(is_instance)))
                } else {
                    Err(RaccoonError::new(
                        "Type.isInstance() second argument must be a Type",
                        position,
                        file,
                    ))
                }
            }
            "isPrimitive" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "Type.isPrimitive() requires 1 argument",
                        position,
                        file,
                    ));
                }

                if let RuntimeValue::Type(type_obj) = &args[0] {
                    Ok(RuntimeValue::Bool(BoolValue::new(type_obj.is_primitive())))
                } else {
                    Err(RaccoonError::new(
                        "Type.isPrimitive() requires a Type argument",
                        position,
                        file,
                    ))
                }
            }
            "isClass" => {
                if args.len() != 1 {
                    return Err(RaccoonError::new(
                        "Type.isClass() requires 1 argument",
                        position,
                        file,
                    ));
                }

                if let RuntimeValue::Type(type_obj) = &args[0] {
                    Ok(RuntimeValue::Bool(BoolValue::new(type_obj.is_class())))
                } else {
                    Err(RaccoonError::new(
                        "Type.isClass() requires a Type argument",
                        position,
                        file,
                    ))
                }
            }
            _ => Err(RaccoonError::new(
                format!("Static method '{}' not found on Type", method),
                position,
                file,
            )),
        }
    }

    fn get_static_property(
        &self,
        _property: &str,
        position: Position,
        file: Option<String>,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            "Type has no static properties",
            position,
            file,
        ))
    }

    fn has_instance_method(&self, method: &str) -> bool {
        matches!(
            method,
            "name"
                | "kind"
                | "isPrimitive"
                | "isClass"
                | "isInterface"
                | "isEnum"
                | "isFunction"
                | "hasMethod"
                | "hasProperty"
                | "getMethods"
                | "getProperties"
                | "getDocumentation"
                | "toString"
        )
    }

    fn has_static_method(&self, method: &str) -> bool {
        matches!(
            method,
            "typeOf" | "name" | "isInstance" | "isPrimitive" | "isClass"
        )
    }

    async fn call_async_instance_method(
        &self,
        _value: &mut RuntimeValue,
        _method: &str,
        _args: Vec<RuntimeValue>,
        position: Position,
        file: Option<String>,
        _callback_executor: &crate::runtime::types::CallbackExecutor,
    ) -> Result<RuntimeValue, RaccoonError> {
        Err(RaccoonError::new(
            "Type has no async instance methods",
            position,
            file,
        ))
    }

    fn has_async_instance_method(&self, _method: &str) -> bool {
        false
    }
}
