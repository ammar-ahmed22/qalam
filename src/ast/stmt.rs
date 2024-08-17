use crate::ast::expr::Expr;
use crate::token::Token;
use crate::ast::visitor::stmt::StmtVisitor;

#[derive(Clone, Debug)]
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
  },
  Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>
  },
  Return {
    keyword: Token,
    value: Option<Expr>
  },
  Class {
    name: Token,
    methods: Vec<Stmt>
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
      Self::While { condition, body } => visitor.visit_while(condition, body),
      Self::Function { name, params, body } => visitor.visit_function(name, params, body),
      Self::Return { keyword, value } => visitor.visit_return(keyword, value),
      Self::Class { name, methods } => visitor.visit_class(name, methods)
    }
  }
}