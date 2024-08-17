pub mod function;
pub mod global;
pub mod class;
pub mod instance;
use std::hash::Hash;

use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::error::RuntimeError;
use crate::token::Token;
use std::any::Any;

pub trait QalamCallable: std::fmt::Debug + Any {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError>;
  fn clone_box(&self) -> Box<dyn QalamCallable>;
  fn to_string(&self) -> String;
  fn arity(&self) -> usize;
  fn as_any(&self) -> &dyn Any;
}

impl Clone for Box<dyn QalamCallable> {
  fn clone(&self) -> Self {
    return self.clone_box();
  }
}

impl Hash for Box<dyn QalamCallable> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      self.to_string().hash(state)
  }
}

impl PartialEq for Box<dyn QalamCallable> {
  fn eq(&self, other: &Self) -> bool {
      self.clone().to_string() == other.clone().to_string()
  }

  fn ne(&self, other: &Self) -> bool {
      return !self.eq(other)
  }
}

impl Eq for Box<dyn QalamCallable> {
  
}