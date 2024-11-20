use crate::native::*;

#[derive(Debug, Clone)]
pub struct MaxFn {}

impl MaxFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for MaxFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
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
                return Ok(Some(Literal::Number(res)));
            } else {
                return Err(RuntimeError::init(
                    paren,
                    format!("{} must be called with number types!", self.to_string()),
                ));
            }
        } else {
            return Err(RuntimeError::init(
                paren,
                format!("{} must be called with number types!", self.to_string()),
            ));
        }
    }

    fn arity(&self) -> usize {
        return 2;
    }

    fn to_string(&self) -> String {
        return String::from("<native amal max(a, b)>");
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
