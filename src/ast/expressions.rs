use crate::ast::{ Expr, Visitor };
use crate::token::Token;
use std::any::Any;

pub struct Binary<L: Expr + ?Sized, R: Expr + ?Sized> {
  pub left: Box<L>,
  pub operator: Token,
  pub right: Box<R>
}

impl <L: Expr, R: Expr> Expr for Binary<L, R> {
  fn accept<V>(&self, visitor: &mut V) -> V::R
    where V: Visitor {
      visitor.visit_binary(self)
  }
}

pub struct Grouping<E: Expr + ?Sized> {
  pub expression: Box<E>
}

impl <E: Expr> Expr for Grouping<E> {
  fn accept<V>(&self, visitor: &mut V) -> V::R
    where V: Visitor {
      visitor.visit_grouping(self)
  }
}

pub struct Literal {
  pub value: Option<Box<dyn Any>>
}

impl Expr for Literal {
  fn accept<V>(&self, visitor: &mut V) -> V::R
    where V: Visitor {
      visitor.visit_literal(self)
  }
}

pub struct Unary<E: Expr + ?Sized> {
  pub operator: Token,
  pub right: Box<E>
}

impl <E: Expr> Expr for Unary<E> {
  fn accept<V>(&self, visitor: &mut V) -> V::R
    where V: Visitor {
      visitor.visit_unary(self)
  }
}