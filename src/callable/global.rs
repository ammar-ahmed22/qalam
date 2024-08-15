use crate::callable::QalamCallable;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::error::RuntimeError;
use crate::token::Token;
use rand::Rng;


fn is_neg(num: f64) -> bool {
  return num < 0.0;
}

fn is_int(num: f64) -> bool {
  return num.fract() == 0.0;
}

fn is_usize(num: f64) -> bool {
  if is_neg(num) {
    return false;
  }

  if !is_int(num) {
    return false;
  }

  return true;
}

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


#[derive(Debug, Clone)]
pub struct SubstrFn {}

impl SubstrFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for SubstrFn {
  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arg = &arguments[0];
      let start = &arguments[1];
      let length = &arguments[2];

      if let (Some(arg), Some(start), Some(length)) = (arg, start, length) {
        if let Literal::String(arg) = arg {
          if let (Literal::Number(start), Literal::Number(length)) = (start, length) {
            if !is_usize(*start) {
              return Err(RuntimeError::init(paren, format!("'start' must be a positive integer!")));
            }

            if !is_usize(*length) {
              return Err(RuntimeError::init(paren, format!("'length' must be a positive integer!")));
            }

            let start = *start as usize;
            let length = *length as usize;

            let s = &arg[start..(start + length)];
            return Ok(Some(Literal::String(String::from(s))));
          } else {
            return Err(RuntimeError::init(paren, format!("'start' = {} and 'length' = {} must be numbers!", start.to_qalam_string(), length.to_qalam_string())))
          }
        } else {
          return Err(RuntimeError::init(paren, format!("'arg' = {} must be a string!", arg.to_qalam_string())))
        }
      } else {
        return Err(RuntimeError::init(paren, format!("All arguments must be defined!")));
      }
  }

  fn arity(&self) -> usize {
      return 3;
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }

  fn to_string(&self) -> String {
      return String::from("<native amal substr(arg, start, length)>")
  }
}

#[derive(Debug, Clone)]
pub struct IndexOfFn {}

impl IndexOfFn {
  pub fn init() -> Self {
    return Self {}
  }
}

impl QalamCallable for IndexOfFn {

  fn call(&mut self, _interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>, paren: &Token) -> Result<Option<Literal>, RuntimeError> {
      let arg = &arguments[0];
      let substring = &arguments[1];
      if let (Some(arg), Some(substring)) = (arg, substring) {
        if let (Literal::String(arg), Literal::String(substring)) = (arg, substring) {
          if let Some(index) = arg.find(substring) {
            return Ok(Some(Literal::Number(index as f64)))
          } else {
            return Ok(Some(Literal::Number(-1.0)))
          }
        } else {
          return Err(RuntimeError::init(paren, format!("'arg' and 'substring' must be strings!")));
        }
      } else {
        return Err(RuntimeError::init(paren, format!("All arguments must be defined!")));
      }
  }

  fn arity(&self) -> usize {
      return 2;
  }

  fn to_string(&self) -> String {
      return String::from("<native amal index_of(arg, substring)>")
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }
}

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
}

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
          let res: f64 = rng.gen_range(*min..*max);
          return Ok(Some(Literal::Number(res)));
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
}

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
        if !is_int(*min) {
          return Err(RuntimeError::init(paren, format!("'min' must be an integer!")));
        }

        if !is_int(*max) {
          return Err(RuntimeError::init(paren, format!("'max' must be an integer!")));
        }
        let min = *min as i32;
        let max = *max as i32;
        if min > max {
          return Err(RuntimeError::init(paren, format!("'max' must be greater than or equal to 'min'")));
        }
        let mut rng = rand::thread_rng();
        let res: i32 = rng.gen_range(min..max);
        return Ok(Some(Literal::Number(res as f64)));
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
}