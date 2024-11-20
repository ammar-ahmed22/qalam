use crate::native::*;

#[derive(Debug, Clone)]
pub struct TypeofFn {}

impl TypeofFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for TypeofFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        _paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let arg = &arguments[0];
        match arg {
            Some(arg) => match arg {
                Literal::Bool(_) => Ok(Some(Literal::String(String::from("bool")))),
                Literal::Number(_) => Ok(Some(Literal::String(String::from("number")))),
                Literal::String(_) => Ok(Some(Literal::String(String::from("string")))),
                Literal::Callable(_) => Ok(Some(Literal::String(String::from("amal")))),
                Literal::Instance(instance) => {
                    Ok(Some(Literal::String(instance.0.borrow().to_string())))
                }
                Literal::Array(_) => Ok(Some(Literal::String(String::from("array")))),
            },
            None => return Ok(Some(Literal::String(String::from("ghaib")))),
        }
    }

    fn arity(&self) -> usize {
        return 1;
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn to_string(&self) -> String {
        return String::from("<native amal typeof(arg)>");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
