use crate::error::ParseError;
use crate::token::{Token, TokenType};
use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::literal::Literal;


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

    return self.call();
  }

  fn call(&mut self) -> Result<Expr, ParseError> {
    let mut expr = self.primary()?;
    loop {
      if self.match_types(&[TokenType::LeftParen]) {
        expr = self.finish_call(expr)?;
      } else {
        break;
      }
    }

    return Ok(expr);
  }

  fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
    let mut arguments: Vec<Expr> = Vec::new();
    if !self.check(&TokenType::RightParen) {
      loop {
        if arguments.len() >= 255 {
          return Err(self.error(&Token::copy(self.peek()), "Can't have more than 255 arguments."))
        }
        arguments.push(self.expression()?);

        if !self.match_types(&[TokenType::Comma]) {
          break;
        }
      }
    }

    let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;
    return Ok(Expr::Call { callee: Box::new(callee), paren: Token::copy(paren), arguments });
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
    if self.match_types(&[TokenType::For]) {
      return self.for_statement();
    }

    if self.match_types(&[TokenType::If]) {
      return self.if_statement();
    }

    if self.match_types(&[TokenType::Print]) {
      return self.print_stmt();
    }

    if self.match_types(&[TokenType::Return]) {
      return self.return_stmt();
    }

    if self.match_types(&[TokenType::While]) {
      return self.while_statement();
    }

    if self.match_types(&[TokenType::LeftBrace]) {
      return Ok(Stmt::Block { statements: self.block()? })
    }


    return self.expression_stmt();
  }

  fn return_stmt(&mut self) -> Result<Stmt, ParseError> {
    let keyword = Self::previous_free(&self.tokens, self.current);
    let mut value = None;
    if !self.check(&TokenType::Semicolon) {
      value = Some(self.expression()?);
    }

    self.consume(&TokenType::Semicolon, "Expect ';' after return value.")?;
    return Ok(Stmt::Return { keyword: keyword.clone(), value });
  }

  fn for_statement(&mut self) -> Result<Stmt, ParseError> {
    self.consume(&TokenType::LeftParen, "Expect '(' after 'tawaf'")?;
    let initializer;
    if self.match_types(&[TokenType::Semicolon]) {
      initializer = None;
    } else if self.match_types(&[TokenType::Var]) {
      initializer = Some(self.var_declaration()?);
    } else {
      initializer = Some(self.expression_stmt()?);
    }

    let mut condition = None;
    if !self.check(&TokenType::Semicolon) {
      condition = Some(self.expression()?);
    }

    self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;
    let mut increment = None;
    if !self.check(&TokenType::RightParen) {
      increment = Some(self.expression()?);
    }
    self.consume(&TokenType::RightParen, "Expect ')' after 'tawaf' clauses.")?;
    let mut body = self.statement()?;

    match increment {
      Some(inc) => {
        body = Stmt::Block { statements: vec![body, Stmt::Expression { expression: inc }] }
      },
      None => {}
    }

    let while_cond = match condition {
      Some(c) => c,
      None => Expr::Literal { value: Some(Literal::Bool(true)) }
    };

    body = Stmt::While { condition: while_cond, body: Box::new(body) };

    match initializer {
      Some(initializer) => {
        body = Stmt::Block { statements: vec![initializer, body] };
      },
      None => {}
    }

    return Ok(body);
  }

  fn while_statement(&mut self) -> Result<Stmt, ParseError> {
    self.consume(&TokenType::LeftParen, "Expect '(' after 'baynama'")?;
    let condition = self.expression()?;
    self.consume(&TokenType::RightParen, "Expect ')' after condition")?;
    let body = self.statement()?;
    return Ok(Stmt::While { condition, body: Box::new(body) })
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

  fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
    let name = Token::copy(self.consume(&TokenType::Identifier, &format!("Expect {} name.", kind))?);
    self.consume(&TokenType::LeftParen, &format!("Expect '(' after {} name.", kind))?;
    let mut params = Vec::new();
    if !self.check(&TokenType::RightParen) {
      loop {
        if params.len() >= 255 {
          return Err(self.error(&Token::copy(self.peek()), "Can't have more than 255 parameters."));
        }

        params.push(Token::copy(self.consume(&TokenType::Identifier, "Expect parameter name.")?));
        if !self.match_types(&[TokenType::Comma]) {
          break;
        }
      }
    }
    self.consume(&TokenType::RightParen, "Expect ')' after parameters.")?;
    self.consume(&TokenType::LeftBrace, &format!("Expect '{{' before {} body.", kind))?;

    let body = self.block()?;
    return Ok(Stmt::Function { name, params, body })
  }

  fn declaration(&mut self) -> Result<Stmt, ParseError> {
    let res;
    if self.match_types(&[TokenType::Fun]) {
      res = self.function("function");
    } else if self.match_types(&[TokenType::Var]) {
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