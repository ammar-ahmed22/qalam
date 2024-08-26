pub mod scanner;
pub mod token;
pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod environment;
pub mod literal;
pub mod error;
pub mod callable;
pub mod resolver;
pub mod stack;
pub mod hashable;
pub mod native;
use anyhow::{Result, Context};
use std::io::{ self, Write };
use std::cell::RefCell;
use scanner::Scanner;
use token::Token;
use parser::Parser;
use interpreter::Interpreter;
use error::{ ErrorReporter, ErrorType };
use resolver::Resolver;
use std::rc::Rc;
// use std::cell::RefCell;

pub struct Qalam {
  error_reporter: RefCell<ErrorReporter>
}

impl Qalam {
  pub fn init() -> Self {
    return Self {
      error_reporter: RefCell::new(ErrorReporter::init())
    }
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
      },
      Err(e) => {
        reporter.error_token(&e.token, &e.message, ErrorType::Syntax)
      }
    }
  }
  
  fn run_prompt(&mut self) {
    let interpreter = Rc::new(RefCell::new(Interpreter::init()));
    loop {
      print!("> ");
      io::stdout().flush().unwrap();
      let mut input = String::new();
      match io::stdin().read_line(&mut input) {
          Ok(_) => {
              input = input.trim().to_string();
              if input == "exit()" {
                  break;
              }
              self.run_source(&input, interpreter.clone());
              self.error_reporter.borrow_mut().had_error = false;
          },
          Err(err) => {
              eprintln!("Error reading input: {}", err);
          }
      }
    }
  }

  fn run_file(&mut self, path: &String) -> Result<()> {
    let file_content = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read file"))?;
    self.run_source(&file_content, Rc::new(RefCell::new(Interpreter::init())));
    if self.error_reporter.borrow().had_error {
      std::process::exit(65);
    }

    if self.error_reporter.borrow().had_runtime_error {
      std::process::exit(75);
    }
    return Ok(());
  }


  

  pub fn run(&mut self, args: Vec<String>) -> Result<()> {
    if args.len() > 2 {
      println!("Usage: qalam [script]");
      std::process::exit(1);
    } else if args.len() == 2 {
        self.run_file(&args[1])?
    } else {
        self.run_prompt();
    }

    return Ok(());
  }
}