use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::{
    Environment, IntValue, NativeFunctionValue, NullValue, RuntimeValue, StrValue,
};
use colored::*;
use std::io::{self, Write};

pub fn setup_builtins(env: &mut Environment) {
    register_builtin_functions(env);
    register_builtin_constants(env);
}

fn register_builtin_functions(env: &mut Environment) {
    register_print_function(env);
    register_println_function(env);
    register_input_function(env);
    register_len_function(env);
}

fn register_builtin_constants(_env: &mut Environment) {}

fn format_colored_value(value: &RuntimeValue) -> String {
    match value {
        RuntimeValue::Str(s) => s.value.green().to_string(),
        RuntimeValue::Int(i) => i.value.to_string().yellow().to_string(),
        RuntimeValue::Float(f) => f.value.to_string().yellow().to_string(),
        RuntimeValue::Decimal(d) => d.value.to_string().yellow().to_string(),
        RuntimeValue::Bool(b) => b.value.to_string().yellow().to_string(),
        RuntimeValue::Null(_) => "null".bright_black().to_string(),
        RuntimeValue::Future(_) => value.to_string().blue().to_string(),
        RuntimeValue::Object(_) | RuntimeValue::List(_) | RuntimeValue::Map(_) => {
            value.to_string().magenta().to_string()
        }
        RuntimeValue::Class(_) | RuntimeValue::ClassInstance(_) => {
            value.to_string().magenta().to_string()
        }
        RuntimeValue::Function(_)
        | RuntimeValue::NativeFunction(_)
        | RuntimeValue::NativeAsyncFunction(_) => value.to_string().cyan().to_string(),
        _ => value.to_string(),
    }
}

fn register_print_function(env: &mut Environment) {
    let print_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", format_colored_value(arg));
            }
            println!();
            RuntimeValue::Null(NullValue::new())
        },
        Type::Function(Box::new(FunctionType {
            params: vec![],
            return_type: PrimitiveType::void(),
            is_variadic: true,
        })),
    ));
    let _ = env.declare("print".to_string(), print_fn);
}

fn register_println_function(env: &mut Environment) {
    let println_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.is_empty() {
                println!();
            } else {
                let msg = args[0].to_string();
                println!("{}", msg.green());
            }
            RuntimeValue::Null(NullValue::new())
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::str()],
            return_type: PrimitiveType::void(),
            is_variadic: false,
        })),
    ));
    let _ = env.declare("println".to_string(), println_fn);
}

fn register_input_function(env: &mut Environment) {
    let input_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
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
                    println!("{}", prompt.cyan());
                } else {
                    print!("{}", prompt.cyan());
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
    ));
    let _ = env.declare("input".to_string(), input_fn);
}

fn register_len_function(env: &mut Environment) {
    let len_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            if args.len() != 1 {
                return RuntimeValue::Null(NullValue::new());
            }
            match &args[0] {
                RuntimeValue::Str(s) => RuntimeValue::Int(IntValue::new(s.value.len() as i64)),
                RuntimeValue::List(l) => RuntimeValue::Int(IntValue::new(l.elements.len() as i64)),
                RuntimeValue::Map(m) => RuntimeValue::Int(IntValue::new(m.entries.len() as i64)),
                _ => RuntimeValue::Null(NullValue::new()),
            }
        },
        Type::Function(Box::new(FunctionType {
            params: vec![PrimitiveType::any()],
            return_type: PrimitiveType::int(),
            is_variadic: false,
        })),
    ));
    let _ = env.declare("len".to_string(), len_fn);
}
