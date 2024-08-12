use crate::{ast::{expressions::Expr, visitor::Visitor}, token::TokenType, Literal};
use crate::token::Token;

pub struct Interpreter {}

impl Interpreter {
  pub fn init() -> Self {
    return Self {}
  }

  fn evaluate(&mut self, expr: &Expr) -> Option<Literal> {
    expr.accept(self)
  }

  fn flip_bool(value: bool, flip: bool) -> bool {
    if flip {
      return !value;
    } else {
      return value;
    }
  }

  fn is_truthy(&mut self, value: Option<Literal>, flip: bool) -> Option<Literal> {
    match value {
      Some(val) => {
        if let Literal::Bool(bool_val) = val {
          return Some(Literal::Bool(Self::flip_bool(bool_val, flip)))
        }
      },
      None => {
        return Some(Literal::Bool(Self::flip_bool(false, flip)))
      }
    }
    return Some(Literal::Bool(Self::flip_bool(true, flip)));
  }

  fn is_equal(&mut self, a: Option<Literal>, b: Option<Literal>, flip: bool) -> Option<Literal> {
    match a {
      Some(a_val) => {
        match b {
          Some(b_val) => {
            if let (Literal::Number(a_val), Literal::Number(b_val)) = (a_val.clone(), b_val.clone()) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            if let (Literal::Bool(a_val), Literal::Bool(b_val)) = (a_val.clone(), b_val.clone()) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            if let (Literal::String(a_val), Literal::String(b_val)) = (a_val, b_val) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }
          },
          None => {
            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          }
        }
      },
      None => {
        match b {
          Some(_) => {
            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          },
          None => {
            return Some(Literal::Bool(Self::flip_bool(true, flip)))
          }
        }
      }
    }

    return None;
  }
}

impl Visitor for Interpreter {
  type R = Option<Literal>;
  fn visit_literal(&mut self, expr: &Option<Literal>) -> Self::R {
      return expr.clone()
  }

  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R {
      return self.evaluate(expression)
  }

  fn visit_unary(&mut self, operator: &Token, right: &Box<Expr>) -> Self::R {
      let right_val = self.evaluate(right);
      match operator.token_type {
        TokenType::Minus => {
          if let Some(val) = right_val {
            match val {
              Literal::Number(num) => {
                return Some(Literal::Number(-num))
              },
              _ => {
                // throw error wrong type
              }
            }
          } else {
            // throw error undefined
          }
        },
        TokenType::Bang => {
          return self.is_truthy(right_val, true)
        },
        _ => {}
      }
      return None;
  }

  fn visit_binary(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R {
      let left_val = self.evaluate(left);
      let right_val = self.evaluate(right);

      if let (Some(left_val), Some(right_val)) = (left_val.clone(), right_val.clone()) {
        match operator.token_type {
          TokenType::Minus => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Some(Literal::Number(left_val - right_val));
            }
          },
          TokenType::Slash => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Some(Literal::Number(left_val / right_val));
            }
          },
          TokenType::Star => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Some(Literal::Number(left_val * right_val));
            }
          },
          TokenType::Plus => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val.clone(), right_val.clone()) {
              return Some(Literal::Number(left_val + right_val));
            }

            if let (Literal::String(left_val), Literal::String(right_val)) = (left_val, right_val) {
              return Some(Literal::String(left_val + right_val.as_str()));
            }
          },
          TokenType::Greater => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Some(Literal::Bool(left_val > right_val));
            }
          },
          TokenType::GreaterEqual=> {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Some(Literal::Bool(left_val >= right_val));
            }
          },
          TokenType::Less => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Some(Literal::Bool(left_val < right_val));
            }
          },
          TokenType::LessEqual => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Some(Literal::Bool(left_val <= right_val));
            }
          },
          TokenType::BangEqual => {
            return self.is_equal(Some(left_val), Some(right_val), true)
          },
          TokenType::EqualEqual => {
            return self.is_equal(Some(left_val), Some(right_val), false)
          }
          _ => {}
        }
      } else {
        // handle equality here for both null
        match operator.token_type {
          TokenType::EqualEqual => {
            return self.is_equal(left_val, right_val, false)
          },
          TokenType::BangEqual => {
            return self.is_equal(left_val, right_val, true)
          },
          _ => {}
        }
      }
    
      return None;
  }
}