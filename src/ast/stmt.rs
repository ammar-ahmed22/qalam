use crate::ast::expr::Expr;
use crate::token::Token;
use crate::ast::visitor::stmt::StmtVisitor;

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
  },
  Block {
    statements: Vec<Stmt>
  },
  If {
    condition: Expr,
    then: Box<Stmt>,
    else_branch: Option<Box<Stmt>>
  },
  While {
    condition: Expr,
    body: Box<Stmt>
  }
}

impl Stmt {
  pub fn accept<V>(&mut self, visitor: &mut V) -> V::R 
  where V: StmtVisitor
  {
    match self {
      Self::Expression { expression } => visitor.visit_expression(expression),
      Self::Print { expression } => visitor.visit_print(expression),
      Self::Var { name, initializer } => visitor.visit_var(name, initializer),
      Self::Block { statements } => visitor.visit_block(statements),
      Self::If { condition, then, else_branch } => visitor.visit_if(condition, then, else_branch),
      Self::While { condition, body } => visitor.visit_while(condition, body)
    }
  }
}