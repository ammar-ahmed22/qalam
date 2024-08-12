 use crate::ast::expressions::Expr;
use crate::token::Token;
use crate::Literal;

pub trait ExprVisitor {
  type R;
  fn visit_binary(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R;
  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R;
  fn visit_literal(&mut self, expr: &Option<Literal>) -> Self::R;
  fn visit_unary(&mut self, operator: &Token, right: &Box<Expr>) -> Self::R;
}

pub trait StmtVisitor {
  type R;
  fn visit_expression(&mut self, expression: &Expr) -> Self::R;
  fn visit_print(&mut self, expression: &Expr) -> Self::R;
}