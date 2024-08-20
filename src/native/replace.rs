use crate::native::*;


#[derive(Debug, Clone)]
pub struct ReplaceFn {}

impl ReplaceFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for ReplaceFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arg = &arguments[0];
      let old_substr = &arguments[1];
      let new_substr = &arguments[2];

      if let (Some(arg), Some(old_substr), Some(new_substr)) = (arg, old_substr, new_substr) {
        if let (Literal::String(arg), Literal::String(old_substr), Literal::String(new_substr)) = (arg, old_substr, new_substr) {
          let res = arg.replace(old_substr, &new_substr);
          return Ok(Some(Literal::String(res)));
        } else {
          return Err(RuntimeError::init(paren, format!("All arguments must be strings!")));
        }
      } else {
        return Err(RuntimeError::init(paren, format!("All arguments must be defined!")));
      }
  }

  fn arity(&self) -> usize {
      return 3;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone());
  }

  fn to_string(&self) -> String {
      return String::from("<native amal replace(arg, old_substr, new_substr)>")
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}