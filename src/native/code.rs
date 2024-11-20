use crate::native::*;

#[derive(Clone, Debug)]
pub struct CodeFn {}

impl CodeFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for CodeFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let char = &arguments[0];
        if let Some(Literal::String(char)) = char {
            if char.len() != 1 {
                return Err(RuntimeError::init(
                    paren,
                    format!("'char' must be a single character!"),
                ));
            }
            let character = char.chars().nth(0).unwrap();
            return Ok(Some(Literal::Number(OrderedFloat(character as u32 as f64))));
        } else {
            return Err(RuntimeError::init(
                paren,
                format!("'char' must be a string!"),
            ));
        }
    }

    fn arity(&self) -> usize {
        return 1;
    }

    fn to_string(&self) -> String {
        return String::from("<native amal code(char)>");
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
