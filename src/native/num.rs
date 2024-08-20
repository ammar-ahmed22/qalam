use crate::native::*;


#[derive(Debug, Clone)]
pub struct NumFn {}

impl NumFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for NumFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arg = &arguments[0];
      if let Some(arg) = arg {
        if let Literal::String(arg) = arg {
          match arg.parse::<f64>() {
            Ok(num) => {
              return Ok(Some(Literal::Number(OrderedFloat(num))))
            },
            Err(_e) => {
              return Err(RuntimeError::init(paren, format!("Cannot convert \"{}\" to number.", arg)))
            }
          }
        } else {
          return Err(RuntimeError::init(paren, format!("{} must be called with string type.", self.to_string())))
        }
      } else {
        return Err(RuntimeError::init(paren, format!("{} must be called with string type.", self.to_string())))
      }
  }

  fn arity(&self) -> usize {
      return 1;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone());
  }

  fn to_string(&self) -> String {
      return String::from("<native amal str2num(arg)>")
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}