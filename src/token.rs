use crate::literal::Literal;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum TokenType {
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  Identifier, String, Number,

  And, Class, Else, ElseIf, False, Fun, If, Nil, Or,
  Print, Return, Super, This, True, Var, While, For,
  Break, Continue, Inherits,

  PlusEqual, MinusEqual, StarEqual, SlashEqual, 

  Increment, Decrement,

  Eof
}

impl TokenType {
  pub fn from_keyword(keyword: &str) -> Option<Self> {
    match keyword {
      "wa" => Some(Self::And),
      "kitab" => Some(Self::Class),
      "illa" => Some(Self::Else),
      "ilshart" => Some(Self::ElseIf),
      "batil" => Some(Self::False),
      "tawaf" => Some(Self::For),
      "amal" => Some(Self::Fun),
      "shart" => Some(Self::If),
      "ghaib" => Some(Self::Nil),
      "aw" => Some(Self::Or),
      "qul" => Some(Self::Print),
      "radd" => Some(Self::Return),
      "ulya" => Some(Self::Super),
      "nafs" => Some(Self::This),
      "haqq" => Some(Self::True),
      "niyya" => Some(Self::Var),
      "baynama" => Some(Self::While),
      "iftar" => Some(Self::Break),
      "safar" => Some(Self::Continue),
      "la" => Some(Self::Bang),
      "ibn" => Some(Self::Inherits),
      _ => None
    }
  }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<Literal>,
  pub line: i64,
  pub position: i64,
}

impl Clone for Token {
  fn clone(&self) -> Self {
      return Self {
        token_type: self.token_type,
        lexeme: self.lexeme.to_string(),
        literal: self.literal.clone(),
        line: self.line,
        position: self.position
      }
  }
}

impl Token {
  pub fn init(token_type: TokenType, lexeme: &String, literal: Option<Literal>, line: i64, position: i64) -> Self {
    return Self {
      token_type,
      lexeme: lexeme.to_string(),
      literal,
      line,
      position
    }
  }

  pub fn dummy() -> Self {
    return Self {
      token_type: TokenType::Eof,
      lexeme: String::from("dummy"),
      literal: None,
      line: -1,
      position: 0
    }
  }

  pub fn copy(token: &Token) -> Self {
    return Self {
      token_type: token.token_type,
      lexeme: token.lexeme.to_string(),
      literal: token.literal.clone(),
      line: token.line,
      position: token.position
    }
  }

  pub fn to_string(&self) -> String {
    return format!("{:?} {} {:?}", self.token_type, self.lexeme, Literal::option_string(self.literal.clone()))
  }
}