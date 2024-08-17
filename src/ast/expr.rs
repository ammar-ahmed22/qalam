use crate::token::Token;
use crate::ast::visitor::expr::ExprVisitor;
use crate::literal::Literal;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Expr {
  Assign {
    name: Token,
    value: Box<Expr>
  },
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
  },
  Variable {
    name: Token
  },
  Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>
  },
  Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>
  },
  Get {
    object: Box<Expr>,
    name: Token
  },
  Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>
  },
  This {
    keyword: Token
  }
}

impl Expr {
  pub fn accept<V>(&self, visitor: &mut V) -> V::R 
  where V: ExprVisitor
  {
    match self {
      Self::Assign { name, value } => visitor.visit_assign(name, value),
      Self::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
      Self::Grouping { expression } => visitor.visit_grouping(expression),
      Self::Literal { value } => visitor.visit_literal(value),
      Self::Unary { operator, right } => visitor.visit_unary(operator, right),
      Self::Variable { name } => visitor.visit_variable(name),
      Self::Logical { left, operator, right } => visitor.visit_logical(left, operator, right),
      Self::Call { callee, paren, arguments } => visitor.visit_call(callee, paren, arguments),
      Self::Get { object, name } => visitor.visit_get(object, name),
      Self::Set { object, name, value } => visitor.visit_set(object, name, value),
      Self::This { keyword } => visitor.visit_this(keyword)
     }
  }
}

