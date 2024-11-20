use crate::native::*;

#[derive(Debug, Clone)]
pub struct IndexOfFn {}

impl IndexOfFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for IndexOfFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let arg = &arguments[0];
        let substring = &arguments[1];
        if let (Some(arg), Some(substring)) = (arg, substring) {
            if let (Literal::String(arg), Literal::String(substring)) = (arg, substring) {
                if let Some(index) = arg.find(substring) {
                    return Ok(Some(Literal::Number(OrderedFloat(index as f64))));
                } else {
                    return Ok(Some(Literal::Number(OrderedFloat(-1.0))));
                }
            } else {
                return Err(RuntimeError::init(
                    paren,
                    format!("'arg' and 'substring' must be strings!"),
                ));
            }
        } else {
            return Err(RuntimeError::init(
                paren,
                format!("All arguments must be defined!"),
            ));
        }
    }

    fn arity(&self) -> usize {
        return 2;
    }

    fn to_string(&self) -> String {
        return String::from("<native amal index_of(arg, substring)>");
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
