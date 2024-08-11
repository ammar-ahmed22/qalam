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

  And, Class, Else, ElseIf, False, Fun, If, Nil, Or,
  Print, Return, Super, This, True, Var, While, For,
  Break, Continue,

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
      "super" => Some(Self::Super),
      "this" => Some(Self::This),
      "haqq" => Some(Self::True),
      "niyya" => Some(Self::Var),
      "baynama" => Some(Self::While),
      "iftar" => Some(Self::Break),
      "safar" => Some(Self::Continue),
      _ => None
    }
  }
}

#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<Box<dyn Any>>,
  pub line: i64
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
      } else if let Some(float_value) = literal.downcast_ref::<f64>() {
        return format!("{}", float_value);
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