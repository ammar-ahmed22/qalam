use crate::token::Token;
use crate::ast::visitor::{ ExprVisitor, StmtVisitor };
use crate::Literal;



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
      Self::Variable { name } => visitor.visit_variable(name)
     }
  }
}

pub enum Stmt {
  Expression {
    expression: Expr
  },
  Print {
    expression: Expr
  },
  Var {
    name: Token,
    initializer: Option<Expr>
  }
}

impl Stmt {
  pub fn accept<V>(&self, visitor: &mut V) -> V::R 
  where V: StmtVisitor
  {
    match self {
      Self::Expression { expression } => visitor.visit_expression(expression),
      Self::Print { expression } => visitor.visit_print(expression),
      Self::Var { name, initializer } => visitor.visit_var(name, initializer)
    }
  }
}