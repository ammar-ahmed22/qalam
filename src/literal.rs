#[derive(Debug, Clone)]
pub enum Literal {
  Number(f64),
  String(String),
  Bool(bool),
}

impl Literal {
  pub fn to_qalam_string(&self) -> String {
    match self {
      Self::Bool(val) => format!("{}", if *val { "haqq" } else { "batil" }),
      Self::Number(val) => format!("{}", val),
      Self::String(val) => val.to_owned()
    }
  }

  pub fn option_string(value: Option<Literal>) -> String {
    match value {
      Some(val) => val.to_qalam_string(),
      None => String::from("ghaib")
    }
  }
}