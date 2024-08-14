use crate::callable::QalamCallable;

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