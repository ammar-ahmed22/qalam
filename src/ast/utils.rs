use crate::ast::expressions::Expr;
use crate::ast::visitor::ExprVisitor;
use crate::token::Token;
use crate::Literal;
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

impl ExprVisitor for ASTParenString {
  type R = String;
  fn visit_binary(&mut self, left: &Box<Expr>, operator: &crate::token::Token, right: &Box<Expr>) -> Self::R {
      self.parenthesize(&operator.lexeme, &[left, right])
  }

  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R {
      self.parenthesize("grouping", &[expression])
  }

  fn visit_literal(&mut self, expr: &Option<Literal>) -> Self::R {
      if let Some(expr) = expr {
        return expr.to_string()
      } else {
        return String::from("null")
      }
  }

  fn visit_unary(&mut self, operator: &crate::token::Token, right: &Box<Expr>) -> Self::R {
      self.parenthesize(&operator.lexeme, &[right])
  }

  fn visit_variable(&mut self, name: &Token) -> Self::R {
      return name.lexeme.to_owned()
  }

  fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Self::R {
      self.parenthesize(&name.lexeme, &[value])
  }
}