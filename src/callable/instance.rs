use crate::callable::class::QalamClass;
use crate::error::RuntimeError;
use crate::literal::Literal;
use crate::hashable::{HashableMap, HashableRcRefCell};
use crate::token::Token;
use crate::callable::function::QalamFunction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QalamInstance {
  pub class: QalamClass,
  pub fields: HashableMap<String, Option<Literal>>
}

impl QalamInstance {
  pub fn init(class: QalamClass) -> Self {
    return Self {
      class,
      fields: HashableMap::new()
    }
  }

  pub fn to_string(&self) -> String {
    return format!("<instanceof {}>", self.class.name);
  }

  pub fn get(instance: HashableRcRefCell<QalamInstance>, name: &Token) -> Result<Option<Literal>, RuntimeError> {
    if instance.0.borrow().fields.contains_key(&name.lexeme) {
      return Ok(instance.0.borrow().fields.get(&name.lexeme).unwrap().clone())
    }

    let method = instance.0.borrow().class.find_method(&name.lexeme);
    if let Some(method) = method {
      if let Some(method) = method.as_any().downcast_ref::<QalamFunction>() {
        return Ok(Some(Literal::Callable(Box::new(method.bind(instance.clone())))));
      }
      // return Ok(Some(Literal::Callable(method)));
    }

    return Err(RuntimeError::init(name, format!("Undefined property '{}'.", name.lexeme)))
  }

  pub fn set(&mut self, name: &Token, value: Option<Literal>) {
    self.fields.insert(name.lexeme.to_owned(), value);
  }
}