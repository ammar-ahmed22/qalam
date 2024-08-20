use crate::native::*;


#[derive(Debug, Clone)]
pub struct RandomFn {}

impl RandomFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for RandomFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let min = &arguments[0];
      let max = &arguments[1];
      if let (Some(min), Some(max)) = (min, max) {
        if let (Literal::Number(min), Literal::Number(max)) = (min, max) {
          if min > max {
            return Err(RuntimeError::init(paren, format!("'max' must be greater than or equal to 'min'")));
          }
          let mut rng = rand::thread_rng();
          let res: f64 = rng.gen_range(**min..**max);
          return Ok(Some(Literal::Number(OrderedFloat(res))));
        } else {
          return Err(RuntimeError::init(paren, format!("All arguments must be numbers!")));
        }
      } else {
        return Err(RuntimeError::init(paren, format!("All arguments must be defined!")))
      }
  }

  fn arity(&self) -> usize {
      return 2;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone());
  }

  fn to_string(&self) -> String {
      return String::from("<native amal random(min, max)>")
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}