use crate::ast::types::PrimitiveType;
use crate::fn_type;
use crate::runtime::{
    type_object::{PrimitiveKind, TypeKind},
    type_object_builder::TypeObjectBuilder,
    Environment, FloatValue, IntValue, NativeFunctionValue, NullValue, RuntimeValue, StrValue,
};

pub fn register(env: &mut Environment) {
    register_int(env);
    register_str(env);
    register_bool(env);
    register_float(env);
}

fn register_int(env: &mut Environment) {
    let int_type = TypeObjectBuilder::new(
        PrimitiveType::int(),
        TypeKind::Primitive(PrimitiveKind::Int),
    )
    .static_method(
        "parse",
        RuntimeValue::NativeFunction(NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }
                let s = args[0].to_string();
                match s.parse::<i64>() {
                    Ok(n) => RuntimeValue::Int(IntValue::new(n)),
                    Err(_) => RuntimeValue::Null(NullValue::new()),
                }
            },
            fn_type!(PrimitiveType::str(), PrimitiveType::int()),
        )),
    )
    .static_property("MAX_VALUE", RuntimeValue::Int(IntValue::new(i64::MAX)))
    .static_property("MIN_VALUE", RuntimeValue::Int(IntValue::new(i64::MIN)))
    .documentation("Integer type (i64)")
    .build();
    let _ = env.declare("int".to_string(), RuntimeValue::Type(int_type));
}

fn register_str(env: &mut Environment) {
    let str_type = TypeObjectBuilder::new(
        PrimitiveType::str(),
        TypeKind::Primitive(PrimitiveKind::String),
    )
    .static_method(
        "fromCharCode",
        RuntimeValue::NativeFunction(NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Str(StrValue::new("".to_string()));
                }
                match &args[0] {
                    RuntimeValue::Int(i) => {
                        if let Some(c) = char::from_u32(i.value as u32) {
                            RuntimeValue::Str(StrValue::new(c.to_string()))
                        } else {
                            RuntimeValue::Str(StrValue::new("".to_string()))
                        }
                    }
                    _ => RuntimeValue::Str(StrValue::new("".to_string())),
                }
            },
            fn_type!(PrimitiveType::int(), PrimitiveType::str()),
        )),
    )
    .static_property("empty", RuntimeValue::Str(StrValue::new("".to_string())))
    .documentation("String type (UTF-8)")
    .build();
    let _ = env.declare("str".to_string(), RuntimeValue::Type(str_type));
}

fn register_bool(env: &mut Environment) {
    let bool_type = TypeObjectBuilder::new(
        PrimitiveType::bool(),
        TypeKind::Primitive(PrimitiveKind::Bool),
    )
    .documentation("Boolean type")
    .build();
    let _ = env.declare("bool".to_string(), RuntimeValue::Type(bool_type));
}

fn register_float(env: &mut Environment) {
    let float_type = TypeObjectBuilder::new(
        PrimitiveType::float(),
        TypeKind::Primitive(PrimitiveKind::Float),
    )
    .static_method(
        "parse",
        RuntimeValue::NativeFunction(NativeFunctionValue::new(
            |args: Vec<RuntimeValue>| {
                if args.is_empty() {
                    return RuntimeValue::Null(NullValue::new());
                }
                let s = args[0].to_string();
                match s.parse::<f64>() {
                    Ok(n) => RuntimeValue::Float(FloatValue::new(n)),
                    Err(_) => RuntimeValue::Null(NullValue::new()),
                }
            },
            fn_type!(PrimitiveType::str(), PrimitiveType::float()),
        )),
    )
    .static_property("MAX_VALUE", RuntimeValue::Float(FloatValue::new(f64::MAX)))
    .static_property("MIN_VALUE", RuntimeValue::Float(FloatValue::new(f64::MIN)))
    .static_property(
        "POSITIVE_INFINITY",
        RuntimeValue::Float(FloatValue::new(f64::INFINITY)),
    )
    .static_property(
        "NEGATIVE_INFINITY",
        RuntimeValue::Float(FloatValue::new(f64::NEG_INFINITY)),
    )
    .static_property("NaN", RuntimeValue::Float(FloatValue::new(f64::NAN)))
    .documentation("Floating point type (f64)")
    .build();
    let _ = env.declare("float".to_string(), RuntimeValue::Type(float_type));
}
