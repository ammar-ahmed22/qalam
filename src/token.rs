use std::any::Any;
use crate::Literal;

#[derive(Debug, PartialEq, Copy, Clone)]
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
  pub literal: Option<Literal>,
  pub line: i64
}

impl Token {
  pub fn init(token_type: TokenType, lexeme: &String, literal: Option<Literal>, line: i64) -> Self {
    return Self {
      token_type,
      lexeme: lexeme.to_string(),
      literal,
      line
    }
  }

  pub fn copy_literal(literal: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
    match literal {
      Some(ref boxed) => {
        if let Some(string_val) = boxed.downcast_ref::<String>() {
          Some(Box::new(string_val.to_string()))
        } else if let Some(num_val) = boxed.downcast_ref::<f64>() {
          Some(Box::new(num_val.clone()))
        } else if let Some(bool_val) = boxed.downcast_ref::<bool>() {
          Some(Box::new(bool_val.clone()))
        } else {
          Some(Box::new(String::from("Unhandled type in copy.")))
        }
      },
      None => None
    }
  }

  pub fn copy(token: &Token) -> Self {
    return Self {
      token_type: token.token_type,
      lexeme: token.lexeme.to_string(),
      literal: token.literal.clone(),
      line: token.line
    }
  }

  pub fn get_literal_string(literal: &Option<Literal>) -> String {
    if let Some(literal) = literal {
      match literal {
        Literal::Bool(val) => format!("{}", val),
        Literal::String(val) => val.to_owned(),
        Literal::Number(val) => format!("{}", val)
      }
    } else {
      return String::from("None");
    }
  }

  pub fn to_string(&self) -> String {
    return format!("{:?} {} {:?}", self.token_type, self.lexeme, Self::get_literal_string(&self.literal))
  }
}