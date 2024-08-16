use crate::ast::expr::Expr;
use crate::ast::visitor::expr::ExprVisitor;
use crate::ast::visitor::stmt::StmtVisitor;
use crate::ast::stmt::Stmt;
use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::stack::Stack;
use std::collections::HashMap;
use crate::token::Token;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum FunctionType {
  Function
}

pub struct Resolver {
  interpreter: Rc<RefCell<Interpreter>>,
  scopes: Stack<HashMap<String, bool>>,
  current_function: Option<FunctionType>,
}

impl Resolver {
  pub fn init(interpreter: Rc<RefCell<Interpreter>>) -> Self {
    return Self {
      interpreter,
      scopes: Stack::new(),
      current_function: None
    }
  }

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn resolve_stmt(&mut self, stmt: &mut Stmt) -> Result<(), RuntimeError> {
    stmt.accept(self)
  }

  fn resolve_expr(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
    expr.accept(self)
  }

  pub fn resolve_stmts(&mut self, statements: &mut Vec<Stmt>) -> Result<(), RuntimeError> {
    for stmt in statements.iter_mut() {
      self.resolve_stmt(stmt)?;
    }

    return Ok(())
  }

  fn resolve_function(&mut self, _name: &Token, params: &Vec<Token>, body: &mut Vec<Stmt>, func_type: Option<FunctionType>) -> Result<(), RuntimeError> {
    let enclosing_func = self.current_function.clone();
    self.current_function = func_type;
    self.begin_scope();
    for param in params.iter() {
      self.declare(param.clone())?;
      self.define(param.clone())?;
    }
    self.resolve_stmts(body)?;
    self.end_scope();
    self.current_function = enclosing_func;
    return Ok(());
  }

  fn declare(&mut self, name: Token) -> Result<(), RuntimeError> {
    if self.scopes.is_empty() {
      return Ok(());
    }

    let scope = self.scopes.peek_mut().expect("Expected a value but found None.");
    if scope.contains_key(&name.lexeme) {
      return Err(RuntimeError::init(&name, format!("Already a variable with this name in this scope.")))
    }
    scope.insert(name.lexeme, false);
    return Ok(());
  }

  fn define(&mut self, name: Token) -> Result<(), RuntimeError> {
    if self.scopes.is_empty() {
      return Ok(())
    }

    let scope = self.scopes.peek_mut().expect("Expected a value but found None.");
    scope.insert(name.lexeme, true);
    return Ok(());
  }

  fn resolve_local_expr(&mut self, expr: &Expr, name: &Token) -> Result<(), RuntimeError> {
    for i in (0..self.scopes.size()).rev() {
      let scope = self.scopes.get(i).unwrap();
      if scope.contains_key(&name.lexeme) {
        self.interpreter.borrow_mut().resolve(expr, self.scopes.size() - 1 - i)?;
        return Ok(())
      }
    }

    return Ok(())
  }
}

impl StmtVisitor for Resolver {
  type R = Result<(), RuntimeError>;

  fn visit_block(&mut self, statements: &mut Vec<Stmt>) -> Self::R {
      self.begin_scope();
      self.resolve_stmts(statements)?;
      self.end_scope();
      return Ok(())
  }

  fn visit_expression(&mut self, expression: &Expr) -> Self::R {
      self.resolve_expr(expression)?;
      return Ok(());
  }

  fn visit_function(&mut self, name: &Token, params: &Vec<Token>, body: &mut Vec<Stmt>) -> Self::R {
      self.declare(name.clone())?;
      self.define(name.clone())?;
      self.resolve_function(name, params, body, Some(FunctionType::Function))?;
      return Ok(());
  }

  fn visit_if(&mut self, condition: &Expr, then: &mut Box<Stmt>, else_branch: &mut Option<Box<Stmt>>) -> Self::R {
      self.resolve_expr(condition)?;
      self.resolve_stmt(then)?;
      if let Some(else_branch) = else_branch {
        self.resolve_stmt(else_branch)?;
      }
      return Ok(());
  }

  fn visit_print(&mut self, expression: &Expr) -> Self::R {
      self.resolve_expr(expression)?;
      return Ok(());
  }

  fn visit_return(&mut self, keyword: &Token, value: &Option<Expr>) -> Self::R {
      if let None = self.current_function {
        return Err(RuntimeError::init(keyword, format!("Can't return from top-level code.")))
      }

      if let Some(value) = value {
        self.resolve_expr(value)?;
      }

      return Ok(());
  }

  fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::R {
      self.declare(name.clone())?;
      if let Some(initializer) = initializer {
        self.resolve_expr(initializer)?;
      }
      self.define(name.clone())?;
      return Ok(());
  }

  fn visit_while(&mut self, condition: &Expr, body: &mut Box<Stmt>) -> Self::R {
      self.resolve_expr(condition)?;
      self.resolve_stmt(body)?;
      return Ok(());
  }
}

impl ExprVisitor for Resolver {
  type R = Result<(), RuntimeError>;

  fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Self::R {
      self.resolve_expr(value)?;
      self.resolve_local_expr(&Expr::Assign { name: name.clone(), value: value.clone() }, name)?;
      return Ok(());
  }

  fn visit_binary(&mut self, left: &Box<Expr>, _operator: &Token, right: &Box<Expr>) -> Self::R {
      self.resolve_expr(left)?;
      self.resolve_expr(right)?;
      return Ok(());
  }

  fn visit_call(&mut self, callee: &Box<Expr>, _paren: &Token, arguments: &Vec<Expr>) -> Self::R {
      self.resolve_expr(callee)?;
      for arg in arguments.iter() {
        self.resolve_expr(arg)?;
      }
      return Ok(());
  }

  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R {
      self.resolve_expr(expression)?;
      return Ok(());
  }

  fn visit_literal(&mut self, _expr: &Option<crate::literal::Literal>) -> Self::R {
      return Ok(());
  }

  fn visit_logical(&mut self, left: &Box<Expr>, _operator: &Token, right: &Box<Expr>) -> Self::R {
      self.resolve_expr(left)?;
      self.resolve_expr(right)?;
      return Ok(());
  }

  fn visit_unary(&mut self, _operator: &Token, right: &Box<Expr>) -> Self::R {
      self.resolve_expr(right)?;
      return Ok(());
  }

  fn visit_variable(&mut self, name: &Token) -> Self::R {
      if !self.scopes.is_empty() && self.scopes.peek().unwrap().get(&name.lexeme).is_some() && *self.scopes.peek().unwrap().get(&name.lexeme).unwrap() == false {
        return Err(RuntimeError::init(name, format!("Can't read local variable in its own initializer.")));
      }

      self.resolve_local_expr(&Expr::Variable { name: name.clone() }, name)?;
      return Ok(());
  }
}