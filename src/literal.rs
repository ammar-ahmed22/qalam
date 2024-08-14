use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::ast::stmt::Stmt;
use crate::environment::Environment;
use crate::token::{ Token, TokenType };

pub trait QalamCallable: std::fmt::Debug {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>) -> Result<Option<Literal>, RuntimeError>;
  fn clone_box(&self) -> Box<dyn QalamCallable>;
  fn to_string(&self) -> String;
  fn arity(&self) -> usize;
}

impl Clone for Box<dyn QalamCallable> {
  fn clone(&self) -> Self {
    return self.clone_box();
  }
}

#[derive(Debug, Clone)]
pub struct QalamFunction {
  pub declaration: Stmt
}

impl QalamFunction {
  pub fn init(declaration: Stmt) -> Self {
    Self {
      declaration
    }
  }
}

impl QalamCallable for QalamFunction {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Option<Literal>>) -> Result<Option<Literal>, RuntimeError> {
      let mut env = Environment::init(Some(Box::new(interpreter.environment.clone())));
      match &mut self.declaration {
        Stmt::Function { name: _, params, body } => {
          for i in 0..params.len() {
            let param_name = &params[i].lexeme;
            let arg = &arguments[i];
            env.define(param_name.to_owned(), arg.clone());
          }
          match interpreter.execute_block(body, env) {
            Ok(_) => {},
            Err(e) => {
              return Ok(e.return_value);
            }
          }
          return Ok(None);
        },
        _ => {
          return Err(RuntimeError::init(&Token::init(TokenType::Eof, &String::from("idk"), None, -1), String::from("Something went horribly wrong in a function call.")))
        }
      }
  }

  fn arity(&self) -> usize {
      match &self.declaration {
        Stmt::Function { name: _, params, body: _ } => params.len(),
        _ => 0
      }
  }

  fn clone_box(&self) -> Box<dyn QalamCallable> {
      return Box::new(self.clone())
  }

  fn to_string(&self) -> String {
      match &self.declaration {
        Stmt::Function { name, params, body: _ } => {
          let param_string = params.iter()
            .map(|t| &t.lexeme)
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");
          return format!("<amal {}({})>", name.lexeme, param_string);
        },
        _ => {return format!("ghaib");}
      }
      
  }
}


#[derive(Debug, Clone)]
pub enum Literal {
  Number(f64),
  String(String),
  Bool(bool),
  Callable(Box<dyn QalamCallable>)
}

impl Literal {
  pub fn to_qalam_string(&self) -> String {
    match self {
      Self::Bool(val) => format!("{}", if *val { "haqq" } else { "batil" }),
      Self::Number(val) => format!("{}", val),
      Self::String(val) => val.to_owned(),
      Self::Callable(val) => val.to_string()
    }
  }

  pub fn option_string(value: Option<Literal>) -> String {
    match value {
      Some(val) => val.to_qalam_string(),
      None => String::from("ghaib")
    }
  }
}