use crate::native::*;


#[derive(Debug, Clone)]
pub struct RandomIntFn {}

impl RandomIntFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for RandomIntFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
    let min = &arguments[0];
    let max = &arguments[1];
    if let (Some(min), Some(max)) = (min, max) {
      if let (Literal::Number(min), Literal::Number(max)) = (min, max) {
        if !is_int(**min) {
          return Err(RuntimeError::init(paren, format!("'min' must be an integer!")));
        }

        if !is_int(**max) {
          return Err(RuntimeError::init(paren, format!("'max' must be an integer!")));
        }
        let min = **min as i32;
        let max = **max as i32;
        if min > max {
          return Err(RuntimeError::init(paren, format!("'max' must be greater than or equal to 'min'")));
        }
        let mut rng = rand::thread_rng();
        let res: i32 = rng.gen_range(min..max);
        return Ok(Some(Literal::Number(OrderedFloat(res as f64))));
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
      return String::from("<native amal random_int(min, max)>")
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}