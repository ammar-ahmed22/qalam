use std::any::Any;

#[derive(Debug)]
pub enum TokenType {
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  Identifier, String, Number,

  And, Class, Else, False, Fun, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}

#[derive(Debug)]
pub struct Token {
  token_type: TokenType,
  lexeme: String,
  literal: Option<Box<dyn Any>>,
  line: i64
}

impl Token {
  pub fn init(token_type: TokenType, lexeme: &String, literal: Option<Box<dyn Any>>, line: i64) -> Self {
    return Self {
      token_type,
      lexeme: lexeme.to_string(),
      literal,
      line
    }
  }

  fn get_literal_string(&self) -> String {
    if let Some(literal) = &self.literal {
      if let Some(string_value) = literal.downcast_ref::<String>() {
        return string_value.to_owned();
      } else {
        return String::from("Unhandled type.")
      }
    } else {
      return String::from("None");
    }
  }

  pub fn to_string(&self) -> String {
    return format!("{:?} {} {:?}", self.token_type, self.lexeme, self.get_literal_string())
  }
}