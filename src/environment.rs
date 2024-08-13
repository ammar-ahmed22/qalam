use std::collections::HashMap;
use crate::Literal;
use crate::interpreter::RuntimeError;
use crate::Token;

pub struct Environment {
  values: HashMap<String, Option<Literal>>
}

impl Environment {
  pub fn init() -> Self {
    Self {
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

    return Err(RuntimeError::init(name, format!("Undefined variable '{}'.", name.lexeme)))
  }

  pub fn get(&self, name: &Token) -> Result<Option<Literal>, RuntimeError> {
    match self.values.get(&name.lexeme) {
      Some(value) => {
        return Ok(value.clone())
      },
      None => {
        return Err(RuntimeError::init(name, format!("Undefined variable '{}'.", name.lexeme)))
      }
    }
  }
}