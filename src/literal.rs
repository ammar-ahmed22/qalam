use crate::callable::QalamCallable;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Eq, Hash)]
pub enum Literal {
  Number(OrderedFloat<f64>),
  String(String),
  Bool(bool),
  Callable(Box<dyn QalamCallable>)
}

impl PartialEq for Literal {
  fn eq(&self, other: &Self) -> bool {
      match (self, other) {
        (Literal::Number(a), Literal::Number(b)) => a == b,
        (Literal::String(a), Literal::String(b)) => a == b,
        (Literal::Bool(a), Literal::Bool(b)) => a == b,
        (Literal::Callable(a), Literal::Callable(b)) => {
          std::ptr::eq(&**a, &**b)
        },
        _ => false
      }
  }
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