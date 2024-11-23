pub mod args;
pub mod ast;
pub mod callable;
pub mod environment;
pub mod error;
pub mod hashable;
pub mod interpreter;
pub mod literal;
pub mod native;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod stack;
pub mod token;
use anyhow::{Context, Result};
use args::Args;
use error::{ErrorReporter, ErrorType};
use interpreter::Interpreter;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use std::cell::RefCell;
use std::rc::Rc;
use token::Token;
use rustyline::error::ReadlineError;
use rustyline::Editor;
// use std::cell::RefCell;

pub struct Qalam {
    error_reporter: RefCell<ErrorReporter>,
}

impl Qalam {
    pub fn init() -> Self {
        return Self {
            error_reporter: RefCell::new(ErrorReporter::init()),
        };
    }

    fn run_source(&mut self, source: &String, interpreter: Rc<RefCell<Interpreter>>) {
        let mut reporter = self.error_reporter.borrow_mut();
        let mut scanner = Scanner::init(source, &mut reporter);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::init(tokens);
        match parser.parse() {
            Ok(mut statements) => {
                let mut resolver = Resolver::init(interpreter.clone());
                if let Err(e) = resolver.resolve_stmts(&mut statements) {
                    reporter.runtime_error(&e.token, &e.message, ErrorType::Resolution);
                    return;
                }
                if let Err(e) = interpreter.clone().borrow_mut().interpret(statements) {
                    reporter.runtime_error(&e.token, &e.message, ErrorType::Runtime);
                }
            }
            Err(e) => reporter.error_token(&e.token, &e.message, ErrorType::Syntax),
        }
    }

    fn run_prompt(&mut self) {
        let interpreter = Rc::new(RefCell::new(Interpreter::init()));
        let mut rl= Editor::<(), rustyline::history::FileHistory>::new().expect("Failed to initialize input reader.");

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(input) => {
                    let input = input.trim();
                    if input == "exit()" {
                        break;
                    }
                    let _ = rl.add_history_entry(input);
                    self.run_source(&input.to_string(), interpreter.clone());
                    self.error_reporter.borrow_mut().had_error = false;
                },
                Err(ReadlineError::Interrupted) => {
                    println!("Exiting...");
                    break;
                },
                Err(ReadlineError::Eof) => {
                    println!("Exiting...");
                    break;
                },
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    break;
                }
            }
        }
    }

    fn run_file(&mut self, path: &String) -> Result<()> {
        let file_content =
            std::fs::read_to_string(path).with_context(|| format!("Cannot read file"))?;
        self.run_source(&file_content, Rc::new(RefCell::new(Interpreter::init())));
        if self.error_reporter.borrow().had_error {
            std::process::exit(65);
        }

        if self.error_reporter.borrow().had_runtime_error {
            std::process::exit(75);
        }
        return Ok(());
    }

    pub fn run(&mut self, args: Args) -> Result<()> {
        if let Some(raw) = args.raw {
            // raw takes priority
            // run the raw string
            self.run_source(&raw, Rc::new(RefCell::new(Interpreter::init())));
            if self.error_reporter.borrow().had_error {
                std::process::exit(1);
            }

            if self.error_reporter.borrow().had_runtime_error {
                std::process::exit(1);
            }
            return Ok(());
            // return;
        }

        if let Some(file_path) = args.file_path {
            // check the file type
            let path = std::path::Path::new(&file_path);
            let is_qlm = path.extension().map(|ext| ext == "qlm").unwrap_or(false);
            if !is_qlm {
                return Err(anyhow::anyhow!("Only .qlm files are allowed!"));
            }
            // run the file
            self.run_file(&file_path)?;
            return Ok(());
        }

        self.run_prompt();

        return Ok(());
    }
}
