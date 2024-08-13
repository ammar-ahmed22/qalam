use std::collections::HashMap;
use crate::Literal;
use crate::interpreter::RuntimeError;
use crate::Token;

#[derive(Debug, Clone)]
pub struct Environment {
  enclosing: Option<Box<Environment>>,
  values: HashMap<String, Option<Literal>>
}

impl Environment {
  pub fn init(enclosing: Option<Box<Environment>>) -> Self {
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
        enclosed.assign(name, value)?;
        return Ok(());
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
            return enclosed.get(name)
          },
          None => {}
        }
        return Err(RuntimeError::init(name, format!("Undefined variable '{}'.", name.lexeme)))
      }
    }
  }
}