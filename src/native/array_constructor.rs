use crate::hashable::HashableRcRefCell;
use crate::literal::QalamArray;
use crate::native::*;

#[derive(Clone, Debug)]
pub struct ArrayConstructorFn {}

impl ArrayConstructorFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for ArrayConstructorFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let size = &arguments[0];
        let value = &arguments[1];
        if let Some(Literal::Number(size)) = size {
            if !is_usize(**size) {
                return Err(RuntimeError::init(
                    paren,
                    format!("'size' must be a positive integer!"),
                ));
            }
            let arr = QalamArray::construct(**size as usize, value.clone());
            return Ok(Some(Literal::Array(HashableRcRefCell::init(arr))));
        } else {
            return Err(RuntimeError::init(
                paren,
                format!("'size' must be a number!"),
            ));
        }
    }

    fn arity(&self) -> usize {
        return 2;
    }

    fn to_string(&self) -> String {
        return String::from("<native amal Array(size, value)>");
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
