use crate::token::Token;
use crate::ast::visitor::Visitor;
use crate::Literal;

pub enum Expr {
  Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>
  },
  Grouping {
    expression: Box<Expr>
  },
  Literal {
    value: Option<Literal>
  },
  Unary {
    operator: Token,
    right: Box<Expr>
  }
}

impl Expr {
  pub fn accept<V>(&self, visitor: &mut V) -> V::R 
  where V: Visitor
  {
    match self {
      Self::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
      Self::Grouping { expression } => visitor.visit_grouping(expression),
      Self::Literal { value } => visitor.visit_literal(value),
      Self::Unary { operator, right } => visitor.visit_unary(operator, right)
     }
  }
}

// pub struct Binary<L: Expr, R: Expr> {
//   pub left: Box<L>,
//   pub operator: Token,
//   pub right: Box<R>
// }

// impl <L: Expr, R: Expr> Expr for Binary<L, R> {
//   fn accept<V>(&self, visitor: &mut V) -> V::R
//     where V: Visitor {
//       visitor.visit_binary(self)
//   }
// }

// pub struct Grouping<E: Expr> {
//   pub expression: Box<E>
// }

// impl <E: Expr> Expr for Grouping<E> {
//   fn accept<V>(&self, visitor: &mut V) -> V::R
//     where V: Visitor {
//       visitor.visit_grouping(self)
//   }
// }

// pub struct Literal {
//   pub value: Option<Box<dyn Any>>
// }

// impl Expr for Literal {
//   fn accept<V>(&self, visitor: &mut V) -> V::R
//     where V: Visitor {
//       visitor.visit_literal(self)
//   }
// }

// pub struct Unary<E: Expr> {
//   pub operator: Token,
//   pub right: Box<E>
// }

// impl <E: Expr> Expr for Unary<E> {
//   fn accept<V>(&self, visitor: &mut V) -> V::R
//     where V: Visitor {
//       visitor.visit_unary(self)
//   }
// }