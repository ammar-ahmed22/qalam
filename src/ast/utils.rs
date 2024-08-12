use crate::ast::expressions::Expr;
use crate::ast::visitor::Visitor;
// use crate::ast::expressions::{ Binary, Unary, Literal, Grouping };

pub struct ASTParenString {}

impl ASTParenString {
  fn parenthesize(&mut self, name: &str, exprs: &[&Box<Expr>]) -> String {
    let mut str = String::new();
    str += "(";
    str += &name;
    for expr in exprs.iter() {
      str += " ";
      str += &expr.accept(self);
    }
    str += ")";
    return str;
  }

  pub fn to_string(&mut self, expr: Expr) -> String
  {
    expr.accept(self)
  }
}

impl Visitor for ASTParenString {
  type R = String;
  fn visit_binary(&mut self, left: &Box<Expr>, operator: &crate::token::Token, right: &Box<Expr>) -> Self::R {
      self.parenthesize(&operator.lexeme, &[left, right])
  }

  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R {
      self.parenthesize("grouping", &[expression])
  }

  fn visit_literal(&mut self, expr: &Option<Box<dyn std::any::Any>>) -> Self::R {
      match expr {
        Some(val) => {
          if let Some(string_val) = val.downcast_ref::<&str>() {
            return string_val.to_string()
          } else if let Some(num_val) = val.downcast_ref::<f64>() {
            return format!("{}", num_val);
          } else if let Some(bool_val) = val.downcast_ref::<bool>() {
            return format!("{}", bool_val)
          } else {
            return "Unhandled type".to_string()
          }
        },
        None => {
          return "null".to_string();
        }
      }
  }

  fn visit_unary(&mut self, operator: &crate::token::Token, right: &Box<Expr>) -> Self::R {
      self.parenthesize(&operator.lexeme, &[right])
  }
}