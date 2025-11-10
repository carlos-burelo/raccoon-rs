use raccoon_lang::{Interpreter, Lexer, Parser};
use std::env;
use std::fs;
use std::process;

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_stack_size(256 * 1024 * 1024)
        .enable_time()
        .build()
        .expect("Failed to create Tokio runtime");

    let local = tokio::task::LocalSet::new();
    runtime.block_on(local.run_until(async_main()));
}

async fn async_main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: raccoon <file.rcc>");
        eprintln!("Example: cargo run -- examples/test.rcc");
        process::exit(1);
    }

    let file_path = &args[1];
    run_file(file_path).await;
}

async fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error: Failed to read file '{}': {}", path, error);
            process::exit(1);
        }
    };

    let file = Some(path.to_string());
    let mut lexer = Lexer::new(source, file.clone());

    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(error) => {
            eprintln!("Lexer Error: {}", error);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens, file.clone());

    match parser.parse() {
        Ok(program) => {
            let mut interpreter = Interpreter::new(file.clone());
            match interpreter.interpret(&program).await {
                Ok(_result) => {}
                Err(error) => {
                    eprintln!("{}", error);
                    process::exit(1);
                }
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}
