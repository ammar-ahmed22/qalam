use crate::native::*;


#[derive(Debug, Clone)]
pub struct PowFn {}

impl PowFn {
  pub fn init() -> Self {
    return Self {};
  }
}

impl QalamCallable for PowFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let base = &arguments[0];
      let exp = &arguments[1];
      if let (Some(base), Some(exp)) = (base, exp) {
        if let (Literal::Number(base), Literal::Number(exp)) = (base, exp) {
          return Ok(Some(Literal::Number(OrderedFloat(base.clone().powf(**exp)))))
        } else {
          return Err(RuntimeError::init(paren, format!("{} must be called with number types!", self.to_string())))
        }
      } else {
        return Err(RuntimeError::init(paren, format!("{} must be called with number types!", self.to_string())))
      }
  }

  fn arity(&self) -> usize {
      return 2;
  }

  fn to_string(&self) -> String {
      return String::from("<native amal pow(base, exp)>");
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}