use std::any::Any;
use crate::ast::expressions::Expr;
use crate::token::Token;


pub trait Visitor {
  type R;
  fn visit_binary(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R;
  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R;
  fn visit_literal(&mut self, expr: &Option<Box<dyn Any>>) -> Self::R;
  fn visit_unary(&mut self, operator: &Token, right: &Box<Expr>) -> Self::R;
}