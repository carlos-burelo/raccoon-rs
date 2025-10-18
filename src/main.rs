use clap::{Parser as ClapParser, Subcommand};
use raccoon_lang::{Interpreter, Lexer, Parser, Token};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(ClapParser)]
#[command(name = "raccoon")]
#[command(about = "Raccoon Language Lexer & Parser", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(value_name = "FILE")]
        file: PathBuf,

        #[arg(long, short)]
        tokens: bool,

        #[arg(long, short)]
        ast: bool,

        #[arg(long, short)]
        interpret: bool,
    },
    Repl {
        #[arg(long, short)]
        tokens: bool,

        #[arg(long, short)]
        interpret: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run {
            file,
            tokens,
            ast,
            interpret,
        }) => run_file(file, tokens, ast, interpret),
        Some(Commands::Repl { tokens, interpret }) => run_repl(tokens, interpret),
        None => run_repl(false, true),
    }
}

fn run_file(path: PathBuf, show_tokens: bool, show_ast: bool, should_interpret: bool) {
    let file = Some(path.to_string_lossy().to_string());
    let source = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error al leer el archivo '{}': {}", path.display(), error);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(source, file.clone());

    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(error) => {
            eprintln!("Error de tokenización: {}", error);
            process::exit(1);
        }
    };

    if show_tokens {
        println!("=== TOKENS ===");
        print_tokens(&tokens);
        println!();
    }

    let mut parser = Parser::new(tokens, file.clone());

    match parser.parse() {
        Ok(program) => {
            if show_ast {
                println!("=== AST ===");
                println!("{:#?}", program);
                println!();
            }

            if should_interpret {
                let mut interpreter = Interpreter::new(file.clone());
                match interpreter.interpret(&program) {
                    Ok(result) => {
                        println!("{}", result.to_string());
                    }
                    Err(error) => {
                        eprintln!("Error de ejecución: {}", error);
                        process::exit(1);
                    }
                }
            } else if !show_ast && !show_tokens {
            }
        }
        Err(error) => {
            eprintln!("Error de parseo: {}", error);
            process::exit(1);
        }
    }
}

fn run_repl(show_tokens: bool, should_interpret: bool) {
    println!("Raccoon REPL");
    if should_interpret {
        println!("Modo de interpretación activado");
    }
    println!("Escribe código para parsearlo o '.help' para ayuda\n");

    let mut rl = DefaultEditor::new().expect("Error al inicializar REPL");
    let history_path = ".raccoon_history";
    let mut interpreter = Interpreter::new(None);

    if rl.load_history(history_path).is_err() {
        println!("Sin historial previo.");
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(&line);

                if trimmed.starts_with('.') {
                    if !handle_repl_command(trimmed) {
                        break;
                    }
                    continue;
                }

                let mut lexer = Lexer::new(line, None);

                let tokens = match lexer.tokenize() {
                    Ok(t) => t,
                    Err(error) => {
                        eprintln!("Error de tokenización: {}", error);
                        continue;
                    }
                };

                if show_tokens {
                    println!("\n=== TOKENS ===");
                    print_tokens(&tokens);
                }

                let mut parser = Parser::new(tokens, None);

                match parser.parse() {
                    Ok(program) => {
                        if !should_interpret {
                            println!("\n=== AST ===");
                            println!("{:#?}", program);
                            println!("\n✓ Parseo exitoso");
                        } else {
                            match interpreter.interpret(&program) {
                                Ok(result) => {
                                    println!("{}", result.to_string());
                                }
                                Err(error) => {
                                    eprintln!("Error de ejecución: {}", error);
                                }
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("Error de parseo: {}", error);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(history_path);
    println!("\n¡Hasta luego!");
}

fn handle_repl_command(command: &str) -> bool {
    match command {
        ".exit" | ".quit" | ".q" => {
            return false;
        }
        ".help" | ".h" => {
            println!("Comandos disponibles:");
            println!("  .help, .h      - Mostrar esta ayuda");
            println!("  .exit, .quit   - Salir del REPL");
            println!("  .clear, .c     - Limpiar pantalla");
            println!("  .example       - Mostrar ejemplo de código");
        }
        ".clear" | ".c" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        ".example" => {
            println!("Ejemplo de código Raccoon:");
            println!();
            println!("  let x: int = 42;");
            println!("  const message: str = \"Hello, Raccoon!\";");
            println!();
            println!("  fn add(a: int, b: int): int {{");
            println!("      return a + b;");
            println!("  }}");
            println!();
            println!("  class Person {{");
            println!("      name: str;");
            println!();
            println!("      constructor(name: str) {{");
            println!("          this.name = name;");
            println!("      }}");
            println!("  }}");
            println!();
            println!("  const template = `Value: ${{x * 2}}`;");
        }
        _ => {
            println!("Comando desconocido: {}", command);
            println!("Usa '.help' para ver los comandos disponibles");
        }
    }

    true
}

fn print_tokens(tokens: &[Token]) {
    for (i, token) in tokens.iter().enumerate() {
        println!(
            "[{}] {:?} | {} | L{}:C{}",
            i,
            token.token_type,
            if token.value.is_empty() {
                "<empty>"
            } else {
                &token.value
            },
            token.position.0,
            token.position.1
        );
    }
}
