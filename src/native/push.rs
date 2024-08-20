use crate::native::*;


#[derive(Debug, Clone)]
pub struct PushFn {}

impl PushFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for PushFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arr = &arguments[0];
      let val = &arguments[1];
      if let Some(Literal::Array(arr)) = arr {
        arr.0.as_ref().borrow_mut().elements.push(val.clone());
        return Ok(None);
      } else {
        return Err(RuntimeError::init(paren, format!("'arr' must be an array!")))
      }
  }

  fn arity(&self) -> usize {
      return 2;
  }

  fn to_string(&self) -> String {
      return "<native amal push(arr, val)>".to_string()
  }

  fn as_any(&self) -> &dyn std::any::Any {
      return self;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
}