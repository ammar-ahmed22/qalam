use std::fmt;

use crate::token::{Token, TokenType};
use crate::ast::expressions::Expr;
// use crate::ast::expressions::{ Binary, Unary, Literal, Grouping };


// /// Parsing tokens into an AST using the below expression grammar:
// /// 
// /// expression     → equality ;  
// /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;  
// /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;  
// /// term           → factor ( ( "-" | "+" ) factor )* ;  
// /// factor         → unary ( ( "/" | "*" ) unary )* ;  
// /// unary          → ( "!" | "-" ) unary  
// ///                | primary ;  
// /// primary        → NUMBER | STRING | "true" | "false" | "nil"  
// ///               | "(" expression ")"

#[derive(Debug)]
pub struct ParseError {
  pub token: Token,
  pub message: String
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "ParseError")
  }
}

impl std::error::Error for ParseError {
  
}

impl ParseError {
  fn init(token: Token, message: String) -> Self {
    return Self {
      token,
      message
    }
  }
}


pub struct Parser<'a> {
  tokens: &'a Vec<Token>,
  current: usize,
  // error_reporter: &'a mut ErrorReporter,
}

impl <'a> Parser<'a> {
  pub fn init(tokens: &'a Vec<Token>) -> Self {
    Self {
      tokens,
      current: 0,
    }
  }

  /// Checks if we have reached the end of the tokens
  fn end(&self) -> bool {
    return self.peek().token_type == TokenType::Eof;
  }

  /// Peeks at the current token
  /// ### Returns
  /// `&Token` - Reference to the token
  fn peek(&self) -> &Token {
    return self.tokens.get(self.current).unwrap();
  }

  /// Gets the previous token (static)
  /// ### Returns
  /// `&Token` - Reference to the token 
  fn previous_free(tokens: &'a Vec<Token>, current: usize) -> &Token {
    return tokens.get(current - 1).unwrap();
  } 

  /// Gets the previous token
  /// ### Returns
  /// `&Token` - Reference to the token 
  fn previous(&self) -> &Token {
    return self.tokens.get(self.current - 1).unwrap();
  }

  /// Checks if the current token is a specified type
  /// ### Arguments
  /// `token_type` - type to check
  fn check(&self, token_type: &TokenType) -> bool {
    if self.end() {
      return false;
    } else {
      return &self.peek().token_type == token_type;
    }
  }

  /// Advances the token pointer and returns the previous value
  /// ### Returns
  /// `&Token` - Reference to the previous value after advancing
  fn advance(&mut self) -> &Token {
    if !self.end() {
      self.current += 1;
    }
    return self.previous();
  }

  /// Checks if the current token matches any of the given token types  
  /// On the first match, token is advanced and `true` returned
  /// ### Arguments
  /// `types` - Token types to check
  fn match_types(&mut self, types: &[TokenType]) -> bool {
    for token_type in types.iter() {
      if self.check(token_type) {
        self.advance();
        return true;
      }
    }

    return false;
  }

  /// Parses an expression
  fn expression(&mut self) -> Result<Expr, ParseError>
  {
    return self.equality();
  }

  /// Parses an equality
  fn equality(&mut self) -> Result<Expr, ParseError>
  {
    let mut expr = self.comparison()?;
    while self.match_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
      let operator = Self::previous_free(&self.tokens, self.current);
      let right = self.comparison()?;
      let prev = expr;
      expr = Expr::Binary {
        left: Box::new(prev),
        operator: Token::copy(operator),
        right: Box::new(right)
      };
    }

    return Ok(expr);
  }

  /// Parses a comparison
  fn comparison(&mut self) -> Result<Expr, ParseError>
  {
    let mut expr = self.term()?;
    while self.match_types(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
      let operator = Self::previous_free(&self.tokens, self.current);
      let right = self.term()?;
      expr = Expr::Binary {
        left: Box::new(expr),
        operator: Token::copy(operator),
        right: Box::new(right)
      }
    }

    return Ok(expr);
  }

  /// Parses a term (addition/subtraction)
  fn term(&mut self) -> Result<Expr, ParseError> {
    let mut expr = self.factor()?;
    while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
      let operator = Self::previous_free(&self.tokens, self.current);
      let right = self.factor()?;
      expr = Expr::Binary {
        left: Box::new(expr),
        operator: Token::copy(operator),
        right: Box::new(right)
      }
    }

    return Ok(expr);
  }

  /// Parses a factor (multiplication/division)
  fn factor(&mut self) -> Result<Expr, ParseError> {
    let mut expr = self.unary()?;
    while self.match_types(&[TokenType::Slash, TokenType::Star]) {
      let operator = Self::previous_free(&self.tokens, self.current);
      let right = self.unary()?;
      expr = Expr::Binary {
        left: Box::new(expr),
        operator: Token::copy(operator),
        right: Box::new(right)
      }
    }

    return Ok(expr);
  }

  /// Parses a unary operation (negation/boolean flip)
  fn unary(&mut self) -> Result<Expr, ParseError> {
    if self.match_types(&[TokenType::Bang, TokenType::Minus]) {
      let operator = Self::previous_free(&self.tokens, self.current);
      let right = self.unary()?;
      return Ok(Expr::Unary {
        right: Box::new(right),
        operator: Token::copy(operator)
      })
    }

    return self.primary();
  }

  /// Parses a primary value 
  fn primary(&mut self) -> Result<Expr, ParseError> {
    if self.match_types(&[TokenType::False]) {
      return Ok(Expr::Literal { value: Some(Box::new(false)) })
    }

    if self.match_types(&[TokenType::True]) {
      return Ok(Expr::Literal { value: Some(Box::new(true)) })
    }

    if self.match_types(&[TokenType::Nil]) {
      return Ok(Expr::Literal { value: None })
    }

    if self.match_types(&[TokenType::String, TokenType::Number]) {
      let prev = self.previous();
      return Ok(Expr::Literal { value: Token::copy_literal(&prev.literal) })
    }

    if self.match_types(&[TokenType::LeftParen]) {
      let expr = self.expression()?;
      self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
      return Ok(Expr::Grouping { expression: Box::new(expr) })
    }

    return Err(self.error(&Token::copy(self.peek()), "Expect expression."))
  }

  /// Checks a token type at the current and advances  
  /// If incorrect type, throws Error
  /// ### Arguments
  /// `token_type` - type to check
  /// `message` - error message
  fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {
    if self.check(token_type) {
      return Ok(self.advance());
    }

    return Err(self.error(&Token::copy(self.peek()), message))
  }

  /// Creates parsing error
  fn error(&mut self, token: &Token, message: &str) -> ParseError {
    // self.error_reporter.error_token(token, message, ErrorType::Syntax);
    return ParseError::init(Token::copy(token), message.to_string());
  }

  #[allow(dead_code)]
  fn synchronize(&mut self) {
    self.advance();

    while !self.end() {
      let prev = self.previous();
      match prev.token_type {
        TokenType::Semicolon => {
          return;
        },
        _ => {}
      };
      let peek = self.peek();
      match peek.token_type {
        TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
          return;
        },
        _ => {}
      };
      self.advance();
    }

  }

  /// Entry function
  pub fn parse(&mut self) -> Result<Expr, ParseError> {
    self.expression()
  }
}