use crate::callable::QalamCallable;
use crate::hashable::HashableRcRefCell;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::ast::stmt::Stmt;
use crate::token::Token;
use std::rc::Rc;
use std::cell::RefCell;
use crate::callable::instance::QalamInstance;

#[derive(Debug, Clone)]
pub struct QalamFunction {
  pub declaration: Stmt,
  pub closure: Rc<RefCell<Environment>>,
  pub is_initializer: bool,
}

impl QalamFunction {
  pub fn init(declaration: Stmt, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> Self {
    Self {
      declaration,
      closure,
      is_initializer
    }
  }

  pub fn bind(&self, instance: HashableRcRefCell<QalamInstance>) -> Self {
    let mut env = Environment::init(Some(self.closure.clone()));
    env.define("nafs".to_string(), Some(Literal::Instance(instance)));
    return Self::init(self.declaration.clone(), Rc::new(RefCell::new(env)), self.is_initializer);
  }
}

impl QalamCallable for QalamFunction {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      // let env = EnvironmentContainer::init(Some(self.closure.clone()));
      let env = Rc::new(RefCell::new(Environment::init(Some(self.closure.clone()))));
      // println!("at line {}, env: {:?}", paren.line, env);
      match &mut self.declaration {
        Stmt::Function { name: _, params, body } => {
          for i in 0..params.len() {
            let param_name = &params[i].lexeme;
            let arg = &arguments[i];
            env.borrow_mut().define(param_name.to_owned(), arg.clone());
          }
          match interpreter.execute_block(body, env) {
            Ok(_) => {},
            Err(e) => {
              if e.message == String::from("dummy") {
                if self.is_initializer {
                  return Ok(Environment::get_at(self.closure.clone(), 0, "nafs".to_string())?)
                }
                return Ok(e.return_value);
              } else {
                return Err(e);
              }
            }
          }
          if self.is_initializer {
            return Ok(Environment::get_at(self.closure.clone(), 0, "nafs".to_string())?)
          }
          return Ok(None);
        },
        _ => {
          return Err(RuntimeError::init(paren, String::from("Can only call functions!")))
        }
      }
  }

  fn arity(&self) -> usize {
      match &self.declaration {
        Stmt::Function { name: _, params, body: _ } => params.len(),
        _ => 0
      }
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }

  fn to_string(&self) -> String {
      match &self.declaration {
        Stmt::Function { name, params, body: _ } => {
          let param_string = params.iter()
            .map(|t| &t.lexeme)
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");
          return format!("<amal {}({})>", name.lexeme, param_string);
        },
        _ => {return format!("ghaib");}
      }
      
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}