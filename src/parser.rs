use std::fmt;

use crate::token::{Token, TokenType};
use crate::ast::expressions::{ Expr, Stmt };
use crate::Literal;
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
    return self.assignment();
  }

  fn assignment(&mut self) -> Result<Expr, ParseError> {
    let expr = self.or()?;
    if self.match_types(&[TokenType::Equal]) {
      let equals = Self::previous_free(&self.tokens, self.current);
      let value = self.assignment()?;
      match expr {
        Expr::Variable { name } => {
          return Ok(Expr::Assign { name, value: Box::new(value) })
        },
        _ => {
          return Err(self.error(equals, "Invalid assignment target."));
        }
      };
    }

    return Ok(expr)
  }

  fn or(&mut self) -> Result<Expr, ParseError> {
    let mut expr = self.and()?;
    while self.match_types(&[TokenType::Or]) {
      let operator = Self::previous_free(&self.tokens, self.current);
      let right = self.and()?;
      expr = Expr::Logical { left: Box::new(expr), operator: Token::copy(operator), right: Box::new(right) };
    }

    return Ok(expr)
  }

  fn and(&mut self) -> Result<Expr, ParseError> {
    let mut expr = self.equality()?;
    while self.match_types(&[TokenType::And]) {
      let operator = Self::previous_free(&self.tokens, self.current);
      let right = self.equality()?;
      expr = Expr::Logical { left: Box::new(expr), operator: Token::copy(operator), right: Box::new(right) };
    }

    return Ok(expr)
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
      return Ok(Expr::Literal { value: Some(Literal::Bool(false)) })
    }

    if self.match_types(&[TokenType::True]) {
      return Ok(Expr::Literal { value: Some(Literal::Bool(true)) })
    }

    if self.match_types(&[TokenType::Nil]) {
      return Ok(Expr::Literal { value: None })
    }

    if self.match_types(&[TokenType::String, TokenType::Number]) {
      let prev = self.previous();
      return Ok(Expr::Literal { value: prev.literal.clone() })
    }

    if self.match_types(&[TokenType::Identifier]) {
      let prev = self.previous();
      return Ok(Expr::Variable { name: Token::copy(prev) })
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

  fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
    let value = self.expression()?;
    self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
    return Ok(Stmt::Print { expression: value })
  }

  fn expression_stmt(&mut self) -> Result<Stmt, ParseError> {
    let value = self.expression()?;
    self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
    return Ok(Stmt::Expression { expression: value })
  }

  fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
    let name = self.consume(&TokenType::Identifier, "Expect variable name.")?;
    let copied = Token::copy(name);
    let mut initializer = None;
    if self.match_types(&[TokenType::Equal]) {
      initializer = Some(self.expression()?);
    }

    self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration.")?;
    return Ok(Stmt::Var { name: copied, initializer })
  }

  fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
    let mut statements: Vec<Stmt> = Vec::new();

    while !self.check(&TokenType::RightBrace) && !self.end() {
      statements.push(self.declaration()?);
    }

    self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;

    return Ok(statements);
  }

  fn statement(&mut self) -> Result<Stmt, ParseError> {
    if self.match_types(&[TokenType::If]) {
      return self.if_statement();
    }

    if self.match_types(&[TokenType::Print]) {
      return self.print_stmt();
    }

    if self.match_types(&[TokenType::LeftBrace]) {
      return Ok(Stmt::Block { statements: self.block()? })
    }

    

    return self.expression_stmt();
  }

  fn if_statement(&mut self) -> Result<Stmt, ParseError> {
    self.consume(&TokenType::LeftParen, "Expect '(' after 'shart'")?;
    let condition = self.expression()?;
    self.consume(&TokenType::RightParen, "Expect ')' after shart condition")?;

    let then = self.statement()?;
    let mut else_branch = None;
    if self.match_types(&[TokenType::Else]) {
      else_branch = Some(Box::new(self.statement()?));
    }

    return Ok(Stmt::If { condition, then: Box::new(then), else_branch })
  }

  fn declaration(&mut self) -> Result<Stmt, ParseError> {
    let res;
    if self.match_types(&[TokenType::Var]) {
      res = self.var_declaration();
    } else {
      res = self.statement();
    }    
    match res {
      Ok(r) => Ok(r),
      Err(e) => {
        self.synchronize();
        return Err(e)
      }
    }
  }

  /// Entry function
  pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
    let mut statements: Vec<Stmt> = Vec::new();
    // self.expression()
    while !self.end() {
      statements.push(self.declaration()?)
    }
    return Ok(statements);
  }
}