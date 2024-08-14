use crate::token::Token;
use crate::ast::expr::Expr;
use crate::Literal;

pub trait ExprVisitor {
  type R;
  fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Self::R;
  fn visit_binary(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R;
  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R;
  fn visit_literal(&mut self, expr: &Option<Literal>) -> Self::R;
  fn visit_unary(&mut self, operator: &Token, right: &Box<Expr>) -> Self::R;
  fn visit_variable(&mut self, name: &Token) -> Self::R;
  fn visit_logical(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R;
}