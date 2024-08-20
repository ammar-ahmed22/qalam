use crate::native::*;


#[derive(Debug, Clone)]
pub struct PopFn {}

impl PopFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for PopFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arr = &arguments[0];
      if let Some(Literal::Array(arr)) = arr {
        let val = arr.0.as_ref().borrow_mut().elements.pop();
        if let Some(val) = val {
          return Ok(val)
        } else {
          return Ok(None);
        }
      } else {
        return Err(RuntimeError::init(paren, format!("'arr' must be an array!")))
      }
  }

  fn arity(&self) -> usize {
      return 1;
  }

  fn to_string(&self) -> String {
      return "<native amal pop(arr)>".to_string()
  }

  fn as_any(&self) -> &dyn std::any::Any {
      return self;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
}