pub mod scanner;
pub mod token;
pub mod ast;
pub mod parser;
use anyhow::{Result, Context};
use std::io::{ self, Write };
use scanner::Scanner;


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

  pub fn error(&mut self, line: i64, message: &str, err_type: ErrorType) {
    self.report(line, message, err_type);
  }

  pub fn report(&mut self, line: i64, message: &str, err_type: ErrorType) {
    eprintln!("{}: {}", err_type.to_string(), message);
    eprintln!("\t at line {}", line);
    self.had_error = true;
  }
}

pub struct Qalam {
  error_reporter: ErrorReporter
}

impl Qalam {
  pub fn init() -> Self {
    return Self {
      error_reporter: ErrorReporter::init()
    }
  }

  fn run_source(&mut self, source: &String) {
    let mut scanner = Scanner::init(source, &mut self.error_reporter);
    let tokens = scanner.scan_tokens();
    for token in tokens.iter() {
        println!("{:?}", token.to_string())
    }
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
              self.error_reporter.had_error = false;
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
    if self.error_reporter.had_error {
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