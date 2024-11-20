use crate::hashable::HashableRcRefCell;
use crate::literal::QalamArray;
use crate::native::is_usize;
use crate::native::*;

#[derive(Debug, Clone)]
pub struct SliceFn {}

impl SliceFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for SliceFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let arr = &arguments[0];
        let start = &arguments[1];
        let end = &arguments[2];
        if let Some(Literal::Array(arr)) = arr {
            if let Some(Literal::Number(start)) = start {
                if let Some(Literal::Number(end)) = end {
                    if !is_usize(**start) {
                        return Err(RuntimeError::init(
                            paren,
                            format!("'start' must be a positive integer!"),
                        ));
                    }

                    if !is_usize(**end) {
                        return Err(RuntimeError::init(
                            paren,
                            format!("'end' must be a positive integer!"),
                        ));
                    }

                    if end < start {
                        return Err(RuntimeError::init(
                            paren,
                            format!("'end' must be greater than or equal to 'start'."),
                        ));
                    }
                    let slice = arr.0.as_ref().borrow().elements
                        [(**start as usize)..(**end as usize)]
                        .to_vec();
                    let new_arr = QalamArray::from_vec(slice);
                    return Ok(Some(Literal::Array(HashableRcRefCell::init(new_arr))));
                } else {
                    return Err(RuntimeError::init(
                        paren,
                        format!("'end' must be a number!"),
                    ));
                }
            } else {
                return Err(RuntimeError::init(
                    paren,
                    format!("'start' must be a number!"),
                ));
            }
        } else {
            return Err(RuntimeError::init(
                paren,
                format!("'arr' must be an array!"),
            ));
        }
    }

    fn arity(&self) -> usize {
        return 3;
    }

    fn to_string(&self) -> String {
        return "<native amal slice(arr, start, end)>".to_string();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        return self;
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }
}
