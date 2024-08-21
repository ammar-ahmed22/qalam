use crate::native::*;


#[derive(Clone, Debug)]
pub struct CeilFn {}

impl CeilFn {
  pub fn init() -> Self {
    return Self {};
  }
}

impl QalamCallable for CeilFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let num = &arguments[0];
      if let Some(Literal::Number(num)) = num {
        return Ok(Some(Literal::Number(OrderedFloat(num.ceil()))))
      } else {
        return Err(RuntimeError::init(paren, format!("'num' must be a number!")))
      }
  }

  fn arity(&self) -> usize {
      return 1;
  }

  fn to_string(&self) -> String {
      return String::from("<native amal ceil(num)>")
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}