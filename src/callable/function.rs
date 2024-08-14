use crate::callable::QalamCallable;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::ast::stmt::Stmt;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct QalamFunction {
  pub declaration: Stmt
}

impl QalamFunction {
  pub fn init(declaration: Stmt) -> Self {
    Self {
      declaration
    }
  }
}

impl QalamCallable for QalamFunction {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let mut env = Environment::init(Some(Box::new(interpreter.environment.clone())));
      match &mut self.declaration {
        Stmt::Function { name: _, params, body } => {
          for i in 0..params.len() {
            let param_name = &params[i].lexeme;
            let arg = &arguments[i];
            env.define(param_name.to_owned(), arg.clone());
          }
          match interpreter.execute_block(body, env) {
            Ok(_) => {},
            Err(e) => {
              return Ok(e.return_value);
            }
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
}