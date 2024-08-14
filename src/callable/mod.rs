pub mod function;
pub mod global;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::error::RuntimeError;

pub trait QalamCallable: std::fmt::Debug {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>) -> Result<Option<Literal>, RuntimeError>;
  fn clone_box(&self) -> Box<dyn QalamCallable>;
  fn to_string(&self) -> String;
  fn arity(&self) -> usize;
}

impl Clone for Box<dyn QalamCallable> {
  fn clone(&self) -> Self {
    return self.clone_box();
  }
}