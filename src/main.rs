use clap::{Parser as ClapParser, Subcommand};
use raccoon_lang::{Colors, Interpreter, Lexer, Parser, Styles, Token};
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

        #[arg(long, short = 'n')]
        no_interpret: bool,
    },
    Repl {
        #[arg(long, short)]
        tokens: bool,

        #[arg(long, short)]
        interpret: bool,
    },
}

fn main() {
    // Increase stack size to handle deeper recursion (256MB)
    // This is needed because async_recursion creates large futures on the stack
    // Each recursive call with async_recursion can consume ~500KB-1MB of stack space
    // With 256MB stack, we can support ~250-500 levels of recursion
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_stack_size(256 * 1024 * 1024)
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async_main());
}

async fn async_main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run {
            file,
            tokens,
            ast,
            no_interpret,
        }) => run_file(file, tokens, ast, !no_interpret).await,
        Some(Commands::Repl { tokens, interpret }) => run_repl(tokens, interpret).await,
        None => run_repl(false, true).await,
    }
}

async fn run_file(path: PathBuf, show_tokens: bool, show_ast: bool, should_interpret: bool) {
    let file = Some(path.to_string_lossy().to_string());
    let source = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!(
                "{}Error:{} Failed to read file '{}': {}",
                Colors::RED,
                Colors::RESET,
                path.display(),
                error
            );
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(source, file.clone());

    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(error) => {
            eprintln!("{}Lexer Error:{} {}", Colors::RED, Colors::RESET, error);
            process::exit(1);
        }
    };

    if show_tokens {
        println!("{}", Styles::section_title("TOKENS"));
        print_tokens(&tokens);
        println!();
    }

    let mut parser = Parser::new(tokens, file.clone());

    match parser.parse() {
        Ok(program) => {
            if show_ast {
                println!("{}", Styles::section_title("AST"));
                println!("{:#?}", program);
                println!();
            }

            if should_interpret {
                let mut interpreter = Interpreter::new(file.clone());
                match interpreter.interpret(&program).await {
                    Ok(_result) => {
                        // Success - output already printed by print() statements
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                        process::exit(1);
                    }
                }
            } else if !show_ast && !show_tokens {
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}

async fn run_repl(show_tokens: bool, should_interpret: bool) {
    println!();
    println!(
        "{}{}🦝 Raccoon Language Interactive Shell{}",
        Colors::BRIGHT_CYAN,
        Colors::BOLD,
        Colors::RESET
    );
    println!(
        "{}v1.0.0 | Type '.help' for assistance | '.exit' to quit{}\n",
        Colors::DIM,
        Colors::RESET
    );

    let mut rl = DefaultEditor::new().expect("Error al inicializar REPL");
    let history_path = ".raccoon_history";
    let mut interpreter = Interpreter::new(None);

    if rl.load_history(history_path).is_err() {
        println!("{}(No previous history){}", Colors::DIM, Colors::RESET);
    }

    loop {
        let prompt = format!("{}raccoon ➜ {}", Colors::BRIGHT_CYAN, Colors::RESET);
        let readline = rl.readline(&prompt);

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
                        eprintln!("{}Lexer Error:{} {}", Colors::RED, Colors::RESET, error);
                        continue;
                    }
                };

                if show_tokens {
                    println!("{}", Styles::section_title("TOKENS"));
                    print_tokens(&tokens);
                }

                let mut parser = Parser::new(tokens, None);

                match parser.parse() {
                    Ok(program) => {
                        if !should_interpret {
                            println!("{}", Styles::section_title("AST"));
                            println!("{:#?}", program);
                            println!("{}", Styles::success("Parse successful"));
                        } else {
                            match interpreter.interpret(&program).await {
                                Ok(result) => {
                                    let result_str = result.to_string();
                                    if result_str != "null" && !result_str.contains("Future") {
                                        println!("{}{}", Colors::GREEN, result_str);
                                        println!("{}", Colors::RESET);
                                    }
                                }
                                Err(error) => {
                                    eprintln!("{}", error);
                                }
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n{}^C{}", Colors::YELLOW, Colors::RESET);
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("\n{}^D{}", Colors::YELLOW, Colors::RESET);
                break;
            }
            Err(err) => {
                eprintln!("{}Error:{} {:?}", Colors::RED, Colors::RESET, err);
                break;
            }
        }
    }

    let _ = rl.save_history(history_path);
    println!(
        "\n{}{}👋 Goodbye!{}\n",
        Colors::BRIGHT_CYAN,
        Colors::BOLD,
        Colors::RESET
    );
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
    let mut w_index = 3usize;
    let mut w_type = 4usize;
    let mut w_lexeme = 6usize;
    let mut w_line = 4usize;
    let mut w_col = 3usize;
    let mut w_range = 7usize;

    for (i, t) in tokens.iter().enumerate() {
        let ty = format!("{:?}", t.token_type);
        let lx = if t.lexeme.is_empty() {
            "<empty>"
        } else {
            &t.lexeme
        };
        let rg = format!(
            "{}:{}..{}:{}",
            t.range.start.0, t.range.start.1, t.range.end.0, t.range.end.1
        );

        w_index = w_index.max(i.to_string().len());
        w_type = w_type.max(ty.len());
        w_lexeme = w_lexeme.max(lx.len());
        w_line = w_line.max(t.position.0.to_string().len());
        w_col = w_col.max(t.position.1.to_string().len());
        w_range = w_range.max(rg.len());
    }

    println!(
        "{:<w_i$}  {:<w_t$}  {:<w_lx$}  {:<w_ln$}  {:<w_cl$}  {:<w_rg$}",
        "IDX",
        "TYPE",
        "LEXEME",
        "LINE",
        "COL",
        "RANGE",
        w_i = w_index,
        w_t = w_type,
        w_lx = w_lexeme,
        w_ln = w_line,
        w_cl = w_col,
        w_rg = w_range
    );

    for (i, t) in tokens.iter().enumerate() {
        let ty = format!("{:?}", t.token_type);
        let lx = if t.lexeme.is_empty() {
            "<empty>"
        } else {
            &t.lexeme
        };
        let rg = format!(
            "{}:{}..{}:{}",
            t.range.start.0, t.range.start.1, t.range.end.0, t.range.end.1
        );

        println!(
            "{:<w_i$}  {:<w_t$}  {:<w_lx$}  {:<w_ln$}  {:<w_cl$}  {:<w_rg$}",
            i,
            ty,
            // v,
            lx,
            t.position.0,
            t.position.1,
            rg,
            w_i = w_index,
            w_t = w_type,
            // w_v = w_value,
            w_lx = w_lexeme,
            w_ln = w_line,
            w_cl = w_col,
            w_rg = w_range
        );
    }
}
