/// Módulo que contiene todas las funciones, objetos y constantes
/// disponibles globalmente en el lenguaje Raccoon.
///
/// Este módulo centraliza todos los builtins del lenguaje para facilitar
/// su mantenimiento y organización.

use crate::ast::types::{FunctionType, PrimitiveType, Type};
use crate::runtime::{Environment, IntValue, NativeFunctionValue, NullValue, RuntimeValue, StrValue};
use colored::*;
use std::io::{self, Write};

/// Configura todas las funciones, objetos y constantes globales
/// en el entorno proporcionado.
///
/// # Arguments
/// * `env` - El entorno donde se registrarán los builtins
pub fn setup_builtins(env: &mut Environment) {
    register_builtin_functions(env);
    register_builtin_constants(env);
}

/// Registra todas las funciones built-in disponibles globalmente
fn register_builtin_functions(env: &mut Environment) {
    register_print_function(env);
    register_println_function(env);
    register_input_function(env);
    register_len_function(env);
}

/// Registra constantes built-in disponibles globalmente
fn register_builtin_constants(_env: &mut Environment) {
    // Aquí se pueden agregar constantes globales en el futuro
    // Por ejemplo: PI, E, VERSION, etc.
}

/// Formatea un valor con colores según su tipo
fn format_colored_value(value: &RuntimeValue) -> String {
    match value {
        // Strings -> Verde
        RuntimeValue::Str(s) => s.value.green().to_string(),

        // Números -> Amarillo
        RuntimeValue::Int(i) => i.value.to_string().yellow().to_string(),
        RuntimeValue::Float(f) => f.value.to_string().yellow().to_string(),
        RuntimeValue::Decimal(d) => d.value.to_string().yellow().to_string(),

        // Booleanos -> Amarillo
        RuntimeValue::Bool(b) => b.value.to_string().yellow().to_string(),

        // Null -> Gris/Negro brillante (bright_black)
        RuntimeValue::Null(_) => "null".bright_black().to_string(),

        // Objetos, Listas, Maps -> Magenta
        RuntimeValue::Object(_) | RuntimeValue::List(_) | RuntimeValue::Map(_) => {
            value.to_string().magenta().to_string()
        }

        // Funciones -> Cyan
        RuntimeValue::Function(_) | RuntimeValue::NativeFunction(_) | RuntimeValue::NativeAsyncFunction(_) => {
            value.to_string().cyan().to_string()
        }

        // Futuros -> Azul
        RuntimeValue::Future(_) => value.to_string().blue().to_string(),

        // Clases e instancias -> Magenta
        RuntimeValue::Class(_) | RuntimeValue::ClassInstance(_) => {
            value.to_string().magenta().to_string()
        }

        // Otros -> Default
        _ => value.to_string(),
    }
}

/// Registra la función `print` que imprime valores en la consola con colores
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

/// Registra la función `println` que imprime un mensaje y añade salto de línea
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

/// Registra la función `input` que lee input del usuario
fn register_input_function(env: &mut Environment) {
    let input_fn = RuntimeValue::NativeFunction(NativeFunctionValue::new(
        |args: Vec<RuntimeValue>| {
            // Primer argumento: mensaje a mostrar
            let prompt = if !args.is_empty() {
                args[0].to_string()
            } else {
                String::new()
            };

            // Segundo argumento: si debe incluir salto de línea (default: false)
            let breakline = if args.len() > 1 {
                match &args[1] {
                    RuntimeValue::Bool(b) => b.value,
                    _ => false,
                }
            } else {
                false
            };

            // Mostrar el prompt
            if !prompt.is_empty() {
                if breakline {
                    println!("{}", prompt.cyan());
                } else {
                    print!("{}", prompt.cyan());
                    io::stdout().flush().unwrap();
                }
            }

            // Leer input
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    // Remover el salto de línea final
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

/// Registra la función `len` que retorna la longitud de strings, listas y mapas
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
