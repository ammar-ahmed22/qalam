use crate::callable::QalamCallable;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::error::RuntimeError;
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct ClockFn {}

impl ClockFn {
  pub fn init() -> Self {
    return Self {};
  }
}

impl QalamCallable for ClockFn {
  fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<Option<Literal>>, _paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let start = std::time::SystemTime::now();
      let since_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards.");
      let millis = since_epoch.as_millis() as f64;
      return Ok(Some(Literal::Number(millis / 1000.0)));
  }

  fn arity(&self) -> usize {
      return 0;
  }

  fn to_string(&self) -> String {
      return String::from("<native amal clock>")
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
}

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
          return Ok(Some(Literal::Number(base.clone().powf(*exp))))
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
}

#[derive(Debug, Clone)]
pub struct MaxFn {}

impl MaxFn {
  pub fn init() -> Self {
    return Self {};
  }
}

impl QalamCallable for MaxFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let a = &arguments[0];
      let b = &arguments[1];
      if let (Some(a), Some(b)) = (a, b) {
        if let (Literal::Number(a), Literal::Number(b)) = (a, b) {
          let res;
          if a > b {
            res = *a;
          } else {
            res = *b;
          }
          return Ok(Some(Literal::Number(res)))
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
      return String::from("<native amal max(a, b)>");
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
}

#[derive(Debug, Clone)]
pub struct MinFn {}

impl MinFn {
  pub fn init() -> Self {
    return Self {};
  }
}

impl QalamCallable for MinFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let a = &arguments[0];
      let b = &arguments[1];
      if let (Some(a), Some(b)) = (a, b) {
        if let (Literal::Number(a), Literal::Number(b)) = (a, b) {
          let res;
          if a < b {
            res = *a;
          } else {
            res = *b;
          }
          return Ok(Some(Literal::Number(res)))
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
      return String::from("<native amal min(a, b)>");
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
}


#[derive(Debug, Clone)]
pub struct LenFn {}

impl LenFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for LenFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
    let arg = &arguments[0];
    if let Some(arg) = arg {
      if let Literal::String(arg) = arg {
        return Ok(Some(Literal::Number(arg.len() as f64)));
      } else {
        return Err(RuntimeError::init(paren, format!("{} must be called with string type!", self.to_string())));
      }
    } else {
      return Err(RuntimeError::init(paren, format!("{} must be called with string type!", self.to_string())))
    }  
  }

  fn arity(&self) -> usize {
      return 1;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone());
  }

  fn to_string(&self) -> String {
      return String::from("<native amal len(arg)>");
  }
}


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
              return Ok(Some(Literal::Number(num)))
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
}

#[derive(Debug, Clone)]
pub struct StrFn {}

impl StrFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for StrFn {

  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, _paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arg = &arguments[0];
      return Ok(Some(Literal::String(Literal::option_string(arg.clone()))));
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
  
  fn arity(&self) -> usize {
      return 1;
  }

  fn to_string(&self) -> String {
      return String::from("<native amal str(arg)>")
  }
}

#[derive(Debug, Clone)]
pub struct TypeofFn {}

impl TypeofFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for TypeofFn {

  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, _paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arg = &arguments[0];
      match arg {
        Some(arg) => {
          match arg {
            Literal::Bool(_) => Ok(Some(Literal::String(String::from("bool")))),
            Literal::Number(_) => Ok(Some(Literal::String(String::from("number")))),
            Literal::String(_) => Ok(Some(Literal::String(String::from("string")))),
            Literal::Callable(_) => Ok(Some(Literal::String(String::from("amal")))) 
          }
        },
        None => {
          return Ok(Some(Literal::String(String::from("ghaib"))))
        }
      }
  }

  fn arity(&self) -> usize {
      return 1;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone());
  }

  fn to_string(&self) -> String {
      return String::from("<native amal typeof(arg)>")
  }
}