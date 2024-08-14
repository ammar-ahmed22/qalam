use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::ast::visitor::expr::ExprVisitor;
use crate::ast::visitor::stmt::StmtVisitor;
use crate::token::{ Token, TokenType };
use crate::Literal;
use crate::environment::Environment;

#[derive(Debug)]
pub struct RuntimeError {
  pub message: String,
  pub token: Token
}

impl std::fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RuntimeError")
  }
}

impl std::error::Error for RuntimeError {}

impl RuntimeError {
  pub fn init(token: &Token, message: String) -> Self {
    return Self {
      token: Token::copy(token),
      message
    }
  }
}

pub struct Interpreter {
  environment: Environment
}

impl Interpreter {
  pub fn init() -> Self {
    return Self {
      environment: Environment::init(None)
    }
  }

  fn evaluate(&mut self, expr: &Expr) -> Result<Option<Literal>, RuntimeError> {
    expr.accept(self)
  }

  fn flip_bool(value: bool, flip: bool) -> bool {
    if flip {
      return !value;
    } else {
      return value;
    }
  }

  fn is_truthy(value: Option<Literal>, flip: bool) -> Option<Literal> {
    match value {
      Some(val) => {
        if let Literal::Bool(bool_val) = val {
          return Some(Literal::Bool(Self::flip_bool(bool_val, flip)))
        }
      },
      None => {
        return Some(Literal::Bool(Self::flip_bool(false, flip)))
      }
    }
    return Some(Literal::Bool(Self::flip_bool(true, flip)));
  }

  fn is_equal(&mut self, a: Option<Literal>, b: Option<Literal>, flip: bool) -> Option<Literal> {
    match a {
      Some(a_val) => {
        match b {
          Some(b_val) => {
            if let (Literal::Number(a_val), Literal::Number(b_val)) = (a_val.clone(), b_val.clone()) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            if let (Literal::Bool(a_val), Literal::Bool(b_val)) = (a_val.clone(), b_val.clone()) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            if let (Literal::String(a_val), Literal::String(b_val)) = (a_val, b_val) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          },
          None => {
            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          }
        }
      },
      None => {
        match b {
          Some(_) => {
            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          },
          None => {
            return Some(Literal::Bool(Self::flip_bool(true, flip)))
          }
        }
      }
    }
  }

  fn execute_block(&mut self, statements: &mut Vec<Stmt>, environment: Environment) -> Result<(), RuntimeError> {
    let previous = self.environment.clone();
    // println!("execute_block, previous env: {:?}", previous);
    self.environment = environment;
    for stmt in statements.iter_mut() {
      match self.execute(stmt) {
        Ok(_) => {},
        Err(e) => {
          self.environment = previous.clone();
          return Err(e)
        }
      }
    }

    self.environment = match &self.environment.enclosing {
      Some(env) => *env.clone(),
      None => previous.clone()
    };
    return Ok(());
  }

  pub fn interpret(&mut self, mut statements: Vec<Stmt>) -> Result<(), RuntimeError> {
    for stmt in statements.iter_mut() {
      self.execute(stmt)?;
    }
    Ok(())
  }

  fn execute(&mut self, stmt: &mut Stmt) -> Result<(), RuntimeError> {
    stmt.accept(self)
  }
}

impl ExprVisitor for Interpreter {
  type R = Result<Option<Literal>, RuntimeError>;
  fn visit_literal(&mut self, expr: &Option<Literal>) -> Self::R {
      return Ok(expr.clone())
  }

  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R {
      return self.evaluate(expression)
  }

  fn visit_unary(&mut self, operator: &Token, right: &Box<Expr>) -> Self::R {
      let right_val = self.evaluate(right)?;
      match operator.token_type {
        TokenType::Minus => {
          if let Some(val) = right_val {
            match val {
              Literal::Number(num) => {
                return Ok(Some(Literal::Number(-num)))
              },
              _ => {
                return Err(RuntimeError::init(operator, String::from("Operand must be a number.")))
              }
            }
          } else {
            return Err(RuntimeError::init(operator, String::from("Operand must be a number.")))
          }
        },
        TokenType::Bang => {
          return Ok(Self::is_truthy(right_val, true))
        },
        _ => {}
      }
      return Ok(None); // idk about this??
  }

  fn visit_binary(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R {
      let left_val = self.evaluate(left)?;
      let right_val = self.evaluate(right)?;

      if let (Some(left_val), Some(right_val)) = (left_val.clone(), right_val.clone()) {
        match operator.token_type {
          TokenType::Minus => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Number(left_val - right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Slash => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Number(left_val / right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Star => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Number(left_val * right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Plus => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val.clone(), right_val.clone()) {
              return Ok(Some(Literal::Number(left_val + right_val)));
            }

            if let (Literal::String(left_val), Literal::String(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::String(left_val + right_val.as_str())));
            }

            return Err(RuntimeError::init(operator, String::from("Operands must be two numbers or two strings.")))
          },
          TokenType::Greater => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val > right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::GreaterEqual=> {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val >= right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Less => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val < right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::LessEqual => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val <= right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::BangEqual => {
            return Ok(self.is_equal(Some(left_val), Some(right_val), true))
          },
          TokenType::EqualEqual => {
            return Ok(self.is_equal(Some(left_val), Some(right_val), false))
          }
          _ => {}
        }
      } else {
        // handle equality here for both null
        match operator.token_type {
          TokenType::EqualEqual => {
            return Ok(self.is_equal(left_val, right_val, false))
          },
          TokenType::BangEqual => {
            return Ok(self.is_equal(left_val, right_val, true))
          },
          _ => {
            return Err(RuntimeError::init(operator, String::from("Invalid operation. This should not happen!")))
          }
        }
      }
    
      return Ok(None); // idk about this??
  }
  
  fn visit_variable(&mut self, name: &Token) -> Self::R {
      return Ok(self.environment.get(name)?);
  }

  fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Self::R {
      let value = self.evaluate(value)?;
      self.environment.assign(name, value.clone())?;
      return Ok(value)
  }

  fn visit_logical(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R {
      let left = self.evaluate(left)?;
      match operator.token_type {
        TokenType::Or => {
          match Self::is_truthy(left.clone(), false) {
            Some(val) => {
              match val {
                Literal::Bool(val) => {
                  if val {
                    return Ok(left)
                  }
                },
                _ => {
                  eprintln!("Something went wrong in Interpreter.visit_logical.");
                  // this should never happen, is_truthy always returns bool literal
                }
              }
            },
            None => {
              eprintln!("Something went wrong in Interpreter.visit_logical.");
              // this should never happen, is_truthy always returns bool literal
            }
          }
        },
        _ => {
          match Self::is_truthy(left.clone(), true) {
            Some(val) => {
              match val {
                Literal::Bool(val) => {
                  if val {
                    return Ok(left)
                  }
                },
                _ => {
                  eprintln!("Something went wrong in Interpreter.visit_logical.");
                  // this should never happen, is_truthy always returns bool literal
                }
              }
            },
            None => {
              eprintln!("Something went wrong in Interpreter.visit_logical.");
              // this should never happen, is_truthy always returns bool literal
            }
          }
        }
      };

      return self.evaluate(right);
  }
}

impl StmtVisitor for Interpreter {
  type R = Result<(), RuntimeError>;
  fn visit_expression(&mut self, expression: &Expr) -> Self::R {
      match self.evaluate(expression) {
        Ok(_) => {
          return Ok(())
        },
        Err(e) => {
          return Err(e)
        }
      }
  }

  fn visit_print(&mut self, expression: &Expr) -> Self::R {
      let value = match self.evaluate(expression) {
        Ok(val) => val,
        Err(e) => {
          return Err(e)
        }
      };
      if let Some(val) = value {
        println!("{}", val.to_qalam_string());
      } else {
        println!("ghaib")
      }
      Ok(())
  }

  fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::R {
      let value = match initializer {
        Some(val) => {
          self.evaluate(val)?
        },
        None => None
      };
      self.environment.define(name.lexeme.to_owned(), value);
      Ok(())
  }

  fn visit_block(&mut self, statements: &mut Vec<Stmt>) -> Self::R {
      self.execute_block(statements, Environment::init(Some(Box::new(self.environment.clone()))))?;
      return Ok(())
  }

  fn visit_if(&mut self, condition: &Expr, then: &mut Box<Stmt>, else_branch: &mut Option<Box<Stmt>>) -> Self::R {
      let eval_cond = self.evaluate(condition)?;
      match Self::is_truthy(eval_cond, false) {
        Some(val) => {
          match val {
            Literal::Bool(cond) => {
              if cond {
                self.execute( then)?;
              } else {
                match else_branch {
                  Some(else_stmt) => {
                    self.execute(else_stmt)?;
                  },
                  None => {
                    // do nothing
                  }
                }
              }
            },
            _ => {
              eprintln!("Something went wrong in Interpreter.visit_if")
              // this should never happen, is_truthy always returns a bool literal
            }
          }
        },
        None => {
          eprintln!("Something went wrong in Interpreter.visit_if")
          // this should never happen, is_truthy always returns a bool literal
        }
      }

      return Ok(())
  }

  fn visit_while(&mut self, condition: &Expr, body: &mut Box<Stmt>) -> Self::R {
      // let eval = self.evaluate(condition)?;
      // match self.is_truthy(eval, false) {
      //   Some(val) => {
      //     match val {
      //       Literal::Bool(val) => {
      //         while val {
      //           self.execute(body)?;
      //         }
      //       },
      //       _ => {}
      //     }
      //   },
      //   None => {}
      // }
      let mut iterate = match Self::is_truthy(self.evaluate(condition)?, false) {
        Some(val) => {
          match val {
            Literal::Bool(val) => val,
            _ => false
          }
        },
        None => false
      };
      while iterate {
        self.execute(body)?;
        iterate = match Self::is_truthy(self.evaluate(condition)?, false) {
          Some(val) => {
            match val {
              Literal::Bool(val) => val,
              _ => false
            }
          },
          None => false
        };
      }
      return Ok(())
  }
}