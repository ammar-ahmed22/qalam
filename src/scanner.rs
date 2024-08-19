use crate::token::{Token, TokenType};
use crate::{ ErrorReporter, ErrorType };
use crate::literal::Literal;
use ordered_float::OrderedFloat;

pub struct Scanner<'a> {
  source: String,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: i64,
  position: i64,
  error_reporter: &'a mut ErrorReporter,
}

impl <'a> Scanner<'a> {
  pub fn init(source: &String, error_reporter: &'a mut ErrorReporter) -> Self {
    Self {
      source: source.to_string(),
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
      position: 0,
      error_reporter
    }
  }

  /// Checks if we have reached the end of the source string
  fn end(&self) -> bool {
    return self.current >= self.source.len();
  }

  /// Advances to the next character of the source string
  /// 
  /// ### Returns
  /// `char` - the character at the current spot
  fn advance(&mut self) -> char {
    let c = self.source.chars().nth(self.current);
    self.current += 1;
    return match c {
      Some(c) => c,
      None => {
        eprintln!("Scanner went too far!");
        std::process::exit(1);
      }
    };
  }

  /// Checks if the next character matches a provided value
  /// If it does match, the character is advanced
  /// ### Arguments
  /// `expected` - the character to check
  /// ### Returns
  /// `bool` - Whether it was matched or not
  fn match_next(&mut self, expected: char) -> bool {
    if self.end() {
      return false;
    }

    match self.source.chars().nth(self.current) {
      Some(c) => {
        if expected == c {
          self.current += 1;
          return true;
        } else {
          return false;
        }
      },
      None => {
        return false;
      }
    }

  }

  /// Peeks at the next character without advancing
  /// ### Returns
  /// `char` - the next character
  fn peek(&mut self) -> char {
    if self.end() {
      return '\0'
    } else {
      return match self.source.chars().nth(self.current) {
        Some(c) => c,
        None => {
          eprintln!("Cannot find character!");
          std::process::exit(1);
        }
      }
    }
  }

  /// Peeks at the next next character without advancing
  /// ### Returns
  /// `char` - the next next character
  fn peek_next(&mut self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0';
    }
    return match self.source.chars().nth(self.current + 1) {
      Some(c) => c,
      None => {
        eprintln!("Cannot find character!");
        std::process::exit(1);
      }
    }
  }

  /// Adds a token to the tokens vec
  /// ### Arguments
  /// `token_type` - the type of the token
  /// `literal` - the object literal
  fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
    let text = self.source.get(self.start..self.current);
    match text {
      Some(t) => {
        self.tokens.push(Token::init(token_type, &t.to_string(), literal, self.line, self.position));
        self.position += 1;
      },
      None => {
        eprintln!("Cannot get lexeme!");
        std::process::exit(1);
      }
    }
  }

  /// Handles tokens for strings
  fn string(&mut self) {
    // Iterate while the next character is not a quote
    while self.peek() != '"' && !self.end() {
      // Here we can also support escaping characters
      // if we peek and see a '\' we can ignore the next character
      if self.peek() == '\\' {
        self.advance();
      }
      if self.peek() == '\n' { // Supports multiline strings
        self.line += 1;
        self.position = 0;
      }
      self.advance();
    }

    // If we reached the end the string is not terminated
    if self.end() {
      self.error_reporter.error(self.line, "Unterminated string.", None,ErrorType::Syntax);
      return;
    }

    // Advance to the ending quote
    self.advance();
    let value = self.source.get((self.start + 1)..(self.current - 1));
    match value {
      Some(val) => {
        self.add_token(TokenType::String, Some(Literal::String(val.to_string())))
      },
      None => {
        eprintln!("Cannot get string!");
        std::process::exit(1);
      }
    }
  }

  /// Checks if a character is a digit
  /// ### Returns
  /// `bool` - whether it is a digit or not
  fn is_digit(&self, c: char) -> bool {
    return c >= '0' && c <= '9';
  }

  /// Checks if a character is alphabetical
  /// ### Returns
  /// `bool` - whether it is alphabetical or not
  fn is_alpha(&self, c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
  }

  /// Checks if a character is alphanumeric
  /// ### Returns
  /// `bool` - whether it is alphanumeric or not
  fn is_alphanumeric(&self, c: char) -> bool {
    return self.is_alpha(c) || self.is_digit(c)
  }

  /// Handles tokens for numbers
  fn number(&mut self) {
    let mut peek = self.peek();
    let mut is_digit = self.is_digit(peek);
    while is_digit {
      self.advance();
      peek = self.peek();
      is_digit = self.is_digit(peek);
    }

    let next_peek = self.peek_next();
    if self.peek() == '.' && self.is_digit(next_peek) {
      // Consume the "."
      self.advance();

      // Get digits after the .
      peek = self.peek();
      is_digit = self.is_digit(peek);
      while is_digit {
        self.advance();
        peek = self.peek();
        is_digit = self.is_digit(peek);
      }
    }

    let value = self.source.get(self.start..self.current);
    match value {
      Some(val) => {
        let as_float = match val.parse::<f64>() {
          Ok(f) => f,
          Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
          }
        };
        self.add_token(TokenType::Number, Some(Literal::Number(OrderedFloat(as_float))));
      },
      None => {
        eprintln!("Cannot get number string!");
        std::process::exit(1);
      }
    }
  }

  /// Handles tokens for identifiers
  fn identifier(&mut self) {
    let mut peek = self.peek();
    while self.is_alphanumeric(peek) {
      self.advance();
      peek = self.peek();
    }
    match self.source.get(self.start..self.current) {
      Some(text) => {
        match TokenType::from_keyword(text) {
          Some(token_type) => {
            self.add_token(token_type, None);
          },
          None => {
            self.add_token(TokenType::Identifier, None);
          }
        }
      },
      None => {
        eprintln!("Cannot get string!");
        std::process::exit(1);
      }
    }
  }

  /// Scans the current character to generate the token
  fn scan_token(&mut self) {
    let c = self.advance();
    match c {
      '(' => self.add_token(TokenType::LeftParen, None),
      ')' => self.add_token(TokenType::RightParen, None),
      '{' => self.add_token(TokenType::LeftBrace, None),
      '}' => self.add_token(TokenType::RightBrace, None),
      ',' => self.add_token(TokenType::Comma, None),
      '.' => self.add_token(TokenType::Dot, None),
      '-' => {
        if self.match_next('=') {
          self.add_token(TokenType::MinusEqual, None)
        } else {
          self.add_token(TokenType::Minus, None)
        }
      },
      '+' => {
        if self.match_next('=') {
          self.add_token(TokenType::PlusEqual, None)
        } else {
          self.add_token(TokenType::Plus, None)
        }
      },
      ';' => self.add_token(TokenType::Semicolon, None),
      '*' => {
        if self.match_next('=') {
          self.add_token(TokenType::StarEqual, None)
        } else {
          self.add_token(TokenType::Star, None)
        }
      },
      '!' => {
        if self.match_next('=') {
          self.add_token(TokenType::BangEqual, None)
        } else {
          self.add_token(TokenType::Bang, None)
        }
      },
      '=' => {
        if self.match_next('=') {
          self.add_token(TokenType::EqualEqual, None)
        } else {
          self.add_token(TokenType::Equal, None)
        }
      },
      '<' => {
        if self.match_next('=') {
          self.add_token(TokenType::LessEqual, None)
        } else {
          self.add_token(TokenType::Less, None)
        }
      },
      '>' => {
        if self.match_next('=') {
          self.add_token(TokenType::GreaterEqual, None)
        } else {
          self.add_token(TokenType::Greater, None)
        }
      },
      '/' => {
        if self.match_next('/') { // comment
          // comments goes to to end of line
          while self.peek() != '\n' && !self.end() {
            self.advance();
          }
        } else if self.match_next('=') {
          self.add_token(TokenType::SlashEqual, None);
        } else {
          self.add_token(TokenType::Slash, None);
        }
      },
      '&' => {
        if self.match_next('&') {
          self.add_token(TokenType::And, None)
        }
      },
      '|' => {
        if self.match_next('|') {
          self.add_token(TokenType::Or, None)
        }
      },
      ' ' | '\r' | '\t' => {},
      '\n' => {
        self.line += 1;
        self.position = 0;
      },
      '"' => {
        self.string();
      }
      _ => {
        if self.is_digit(c) {
          self.number();
        } else if self.is_alpha(c) {
          self.identifier();
        } else {
          self.error_reporter.error(self.line, "Unexpected character.", None, ErrorType::Syntax);
        } 
      }
    }
  }

  pub fn scan_tokens(&mut self) -> &Vec<Token> {
    while !self.end() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(Token::init(TokenType::Eof, &String::from(""), None, self.line, self.position + 1));
    return &self.tokens;
  }
}