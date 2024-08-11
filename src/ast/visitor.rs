use crate::ast::Expr;
use crate::ast::expressions::{ Binary, Grouping, Literal, Unary };

pub trait Visitor {
  type R;
  fn visit_binary<LT: Expr, RT: Expr>(&mut self, expr: &Binary<LT, RT>) -> Self::R;
  fn visit_grouping<E: Expr>(&mut self, expr: &Grouping<E>) -> Self::R;
  fn visit_literal(&mut self, expr: &Literal) -> Self::R;
  fn visit_unary<E: Expr>(&mut self, expr: &Unary<E>) -> Self::R;
}