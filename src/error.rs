use crate::literal::Literal;
use crate::token::{Token, TokenType};
use std::fmt;

pub enum ErrorType {
    Error,
    Syntax,
    Runtime,
    Resolution,
}

impl ErrorType {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Error => "Error",
            Self::Syntax => "SyntaxError",
            Self::Runtime => "RuntimeError",
            Self::Resolution => "ResolutionError",
        }
    }
}

pub struct ErrorReporter {
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl ErrorReporter {
    pub fn init() -> Self {
        return Self {
            had_error: false,
            had_runtime_error: false,
        };
    }

    pub fn error_token(&mut self, token: &Token, message: &str, err_type: ErrorType) {
        match token.token_type {
            TokenType::Eof => self.report(token.line, message, Some("at end"), err_type),
            _ => self.report(
                token.line,
                message,
                Some(&format!("at '{}'", token.lexeme)),
                err_type,
            ),
        }
    }

    pub fn error(&mut self, line: i64, message: &str, loc: Option<&str>, err_type: ErrorType) {
        self.report(line, message, loc, err_type);
    }

    pub fn runtime_error(&mut self, token: &Token, message: &str, err_type: ErrorType) {
        eprintln!("{}: {}", err_type.to_string(), message);
        eprintln!("\t at line {}", token.line);
        self.had_runtime_error = true;
    }

    pub fn report(&mut self, line: i64, message: &str, loc: Option<&str>, err_type: ErrorType) {
        eprintln!("{}: {}", err_type.to_string(), message);
        eprintln!("\t at line {}", line);
        match loc {
            Some(val) => {
                eprintln!("\t {}", val)
            }
            None => {}
        };
        self.had_error = true;
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError")
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn init(token: Token, message: String) -> Self {
        return Self { token, message };
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub token: Token,
    pub return_value: Option<Literal>,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuntimeError")
    }
}

impl std::error::Error for RuntimeError {}

impl RuntimeError {
    pub fn init(token: &Token, message: String) -> Self {
        return Self {
            token: Token::copy(token),
            message,
            return_value: None,
        };
    }

    pub fn init_return(value: Option<Literal>) -> Self {
        return Self {
            token: Token::dummy(),
            message: String::from("dummy"),
            return_value: value,
        };
    }
}
