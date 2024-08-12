pub mod scanner;
pub mod token;
pub mod ast;
pub mod parser;
pub mod interpreter;
use anyhow::{Result, Context};
use std::io::{ self, Write };
use std::cell::RefCell;
use scanner::Scanner;
use token::{Token, TokenType};
use parser::Parser;
use ast::utils::ASTParenString;


pub enum ErrorType {
  Error,
  Syntax,
}

impl ErrorType {
  pub fn to_string(&self) -> &str {
    match self {
      Self::Error => "Error",
      Self::Syntax => "SyntaxError"
    }
  }
}

pub struct ErrorReporter {
  had_error: bool
}

impl ErrorReporter {
  pub fn init() -> Self {
    return Self {
      had_error: false
    }
  }

  pub fn error_token(&mut self, token: &Token, message: &str, err_type: ErrorType) {
    match token.token_type {
      TokenType::Eof => {
        self.report(token.line, message, Some("at end"), err_type)
      },
      _ => {
        self.report(token.line, message, Some(&format!("at '{}'", token.lexeme)), err_type)
      }
    }
  }

  pub fn error(&mut self, line: i64, message: &str, loc: Option<&str>, err_type: ErrorType) {
    self.report(line, message, loc, err_type);
  }

  pub fn report(&mut self, line: i64, message: &str, loc: Option<&str>, err_type: ErrorType) {
    eprintln!("{}: {}", err_type.to_string(), message);
    eprintln!("\t at line {}", line);
    match loc {
      Some(val) => {
        eprintln!("\t {}", val)
      },
      None => {}
    };
    self.had_error = true;
  }
}

pub struct Qalam {
  error_reporter: RefCell<ErrorReporter>
}

impl Qalam {
  pub fn init() -> Self {
    return Self {
      error_reporter: RefCell::new(ErrorReporter::init())
    }
  }

  fn run_source(&mut self, source: &String) {
    let mut scanner_reporter = self.error_reporter.borrow_mut();
    let mut scanner = Scanner::init(source, &mut scanner_reporter);
    let tokens = scanner.scan_tokens();
    // let mut parser_reporter = self.error_reporter.borrow_mut();
    let mut parser = Parser::init(tokens);
    let mut ast_string_generator = ASTParenString {};
    match parser.parse() {
      Ok(expr) => {
        println!("{}", ast_string_generator.to_string(expr))
      },
      Err(e) => {
        scanner_reporter.error_token(&e.token, &e.message, ErrorType::Syntax)
        // do nothing
      }
    }
    // for token in tokens.iter() {
    //     println!("{:?}", token.to_string())
    // }
  }
  
  fn run_prompt(&mut self) {
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
              self.run_source(&input);
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
    self.run_source(&file_content);
    if self.error_reporter.borrow().had_error {
      std::process::exit(1);
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