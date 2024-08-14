use crate::callable::QalamCallable;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::error::RuntimeError;

#[derive(Clone, Debug)]
pub struct ClockFn {}

impl ClockFn {
  pub fn init() -> Self {
    return Self {};
  }
}

impl QalamCallable for ClockFn {
  fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<Option<Literal>>) -> Result<Option<Literal>, RuntimeError> {
      let start = std::time::SystemTime::now();
      let since_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards.");
      let millis = since_epoch.as_millis() as f64;
      return Ok(Some(Literal::Number(millis / 1000.0)));
  }

  fn arity(&self) -> usize {
      return 0;
  }

  fn to_string(&self) -> String {
      return String::from("<native fn>")
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
}