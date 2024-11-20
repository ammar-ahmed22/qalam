use crate::native::*;

#[derive(Debug, Clone)]
pub struct StrFn {}

impl StrFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for StrFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        _paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let arg = &arguments[0];
        return Ok(Some(Literal::String(Literal::option_string(arg.clone()))));
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn arity(&self) -> usize {
        return 1;
    }

    fn to_string(&self) -> String {
        return String::from("<native amal str(arg)>");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
