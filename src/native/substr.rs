use crate::native::*;

#[derive(Debug, Clone)]
pub struct SubstrFn {}

impl SubstrFn {
    pub fn init() -> Self {
        return Self {};
    }
}

impl QalamCallable for SubstrFn {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Option<Literal>>,
        paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let arg = &arguments[0];
        let start = &arguments[1];
        let length = &arguments[2];

        if let (Some(arg), Some(start), Some(length)) = (arg, start, length) {
            if let Literal::String(arg) = arg {
                if let (Literal::Number(start), Literal::Number(length)) = (start, length) {
                    if !is_usize(**start) {
                        return Err(RuntimeError::init(
                            paren,
                            format!("'start' must be a positive integer!"),
                        ));
                    }

                    if !is_usize(**length) {
                        return Err(RuntimeError::init(
                            paren,
                            format!("'length' must be a positive integer!"),
                        ));
                    }

                    let start = **start as usize;
                    let length = **length as usize;

                    let s = &arg[start..(start + length)];
                    return Ok(Some(Literal::String(String::from(s))));
                } else {
                    return Err(RuntimeError::init(
                        paren,
                        format!(
                            "'start' = {} and 'length' = {} must be numbers!",
                            start.to_qalam_string(),
                            length.to_qalam_string()
                        ),
                    ));
                }
            } else {
                return Err(RuntimeError::init(
                    paren,
                    format!("'arg' = {} must be a string!", arg.to_qalam_string()),
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
        return 3;
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn to_string(&self) -> String {
        return String::from("<native amal substr(arg, start, length)>");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
