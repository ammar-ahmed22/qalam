use crate::native::*;

#[derive(Debug, Clone)]
pub struct LenFn {}

impl LenFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for LenFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let arg = &arguments[0];
        if let Some(arg) = arg {
            if let Literal::String(arg) = arg {
                return Ok(Some(Literal::Number(OrderedFloat(arg.len() as f64))));
            } else if let Literal::Array(arr) = arg {
                return Ok(Some(Literal::Number(OrderedFloat(
                    arr.0.borrow().elements.len() as f64,
                ))));
            } else {
                return Err(RuntimeError::init(
                    paren,
                    format!(
                        "{} must be called with string or array type!",
                        self.to_string()
                    ),
                ));
            }
        } else {
            return Err(RuntimeError::init(
                paren,
                format!(
                    "{} must be called with string or array type!",
                    self.to_string()
                ),
            ));
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
