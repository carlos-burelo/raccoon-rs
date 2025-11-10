use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::fn_type;
use crate::runtime::{Environment, IntValue, NullValue, RuntimeValue, StrValue};
use std::io::{self, Write};

pub fn register(env: &mut Environment) {
    let _ = env.declare("print".to_string(), print_fn());
    let _ = env.declare("println".to_string(), println_fn());
    let _ = env.declare("eprint".to_string(), eprint_fn());
    let _ = env.declare("input".to_string(), input_fn());
    let _ = env.declare("len".to_string(), len_fn());
}

fn print_fn() -> RuntimeValue {
    RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", arg.to_string());
            }
            println!();
            RuntimeValue::Null(NullValue::new())
        },
        fn_type!(variadic, PrimitiveType::void()),
    ))
}

fn println_fn() -> RuntimeValue {
    RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                println!();
            } else {
                println!("{}", args[0].to_string());
            }
            RuntimeValue::Null(NullValue::new())
        },
        fn_type!(PrimitiveType::str(), PrimitiveType::void()),
    ))
}

fn eprint_fn() -> RuntimeValue {
    RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    eprint!(" ");
                }
                eprint!("{}", arg.to_string());
            }
            eprintln!();
            RuntimeValue::Null(NullValue::new())
        },
        fn_type!(variadic, PrimitiveType::void()),
    ))
}

fn input_fn() -> RuntimeValue {
    RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            let prompt = if !args.is_empty() {
                args[0].to_string()
            } else {
                String::new()
            };

            let breakline = if args.len() > 1 {
                match &args[1] {
                    RuntimeValue::Bool(b) => b.value,
                    _ => false,
                }
            } else {
                false
            };

            if !prompt.is_empty() {
                if breakline {
                    println!("{}", prompt);
                } else {
                    print!("{}", prompt);
                    io::stdout().flush().unwrap();
                }
            }

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim_end_matches(&['\r', '\n'][..]).to_string();
                    RuntimeValue::Str(StrValue::new(trimmed))
                }
                Err(_) => RuntimeValue::Null(NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::str(), PrimitiveType::bool()],
            return_type: PrimitiveType::str(),
            is_variadic: false,
        })),
    ))
}

fn len_fn() -> RuntimeValue {
    RuntimeValue::NativeFunction(crate::runtime::NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                return RuntimeValue::Int(IntValue::new(0));
            }

            match &args[0] {
                RuntimeValue::Str(s) => RuntimeValue::Int(IntValue::new(s.value.len() as i64)),
                RuntimeValue::Array(a) => RuntimeValue::Int(IntValue::new(a.elements.len() as i64)),
                RuntimeValue::Map(m) => RuntimeValue::Int(IntValue::new(m.entries.len() as i64)),
                _ => RuntimeValue::Int(IntValue::new(0)),
            }
        },
        fn_type!(PrimitiveType::any(), PrimitiveType::int()),
    ))
}
