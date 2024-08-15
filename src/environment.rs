use std::collections::HashMap;
use crate::literal::Literal;
use crate::error::RuntimeError;
use crate::Token;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Environment {
  pub enclosing: Option<Rc<RefCell<Environment>>>,
  values: HashMap<String, Option<Literal>>
}

impl Environment {
  pub fn init(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
    Self {
      enclosing,
      values: HashMap::new()
    }
  }

  pub fn define(&mut self, name: String, value: Option<Literal>) {
    self.values.insert(name, value);
  }

  pub fn assign(&mut self, name: &Token, value: Option<Literal>) -> Result<(), RuntimeError> {
    if self.values.contains_key(&name.lexeme) {
      self.values.insert(name.lexeme.to_owned(), value);
      return Ok(());
    }

    match &mut self.enclosing {
      Some(enclosed) => {
        return enclosed.borrow_mut().assign(name, value);
      },
      None => {}
    }

    return Err(RuntimeError::init(name, format!("Undefined variable '{}'.", name.lexeme)))
  }

  pub fn get(&self, name: &Token) -> Result<Option<Literal>, RuntimeError> {
    match self.values.get(&name.lexeme) {
      Some(value) => {
        return Ok(value.clone())
      },
      None => {
        match &self.enclosing {
          Some(enclosed) => {
            return enclosed.borrow_mut().get(name)
          },
          None => {}
        }
        return Err(RuntimeError::init(name, format!("Undefined variable '{}'.", name.lexeme)))
      }
    }
  }
}