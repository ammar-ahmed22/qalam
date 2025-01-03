use crate::callable::instance::QalamInstance;
use crate::callable::QalamCallable;
use crate::error::RuntimeError;
use crate::hashable::HashableMap;
use crate::hashable::HashableRcRefCell;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::token::Token;

use super::function::QalamFunction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QalamClass {
    pub name: String,
    pub methods: HashableMap<String, Box<dyn QalamCallable>>,
    pub superclass: Option<Box<QalamClass>>,
}

impl QalamClass {
    pub fn init(
        name: String,
        methods: HashableMap<String, Box<dyn QalamCallable>>,
        superclass: Option<Box<QalamClass>>,
    ) -> Self {
        return Self {
            name,
            methods,
            superclass,
        };
    }

    pub fn find_method(&self, name: &String) -> Option<Box<dyn QalamCallable>> {
        if self.methods.contains_key(name) {
            return Some(self.methods.get(name).unwrap().clone());
        }

        if let Some(superclass) = &self.superclass {
            return superclass.find_method(name);
        }

        return None;
    }
}

impl QalamCallable for QalamClass {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Option<Literal>>,
        _paren: &Token,
    ) -> Result<Option<Literal>, RuntimeError> {
        let instance = HashableRcRefCell::init(QalamInstance::init(self.clone()));
        // let instance = QalamInstance::init(self.clone());
        let initializer = self.find_method(&String::from("banao"));
        if let Some(initializer) = initializer {
            if let Some(initializer) = initializer.as_any().downcast_ref::<QalamFunction>() {
                initializer
                    .bind(instance.clone())
                    .call(_interpreter, _arguments, _paren)?;
            }
        }
        return Ok(Some(Literal::Instance(instance)));
    }

    fn arity(&self) -> usize {
        let initializer = self.find_method(&String::from("banao"));
        if let Some(initializer) = initializer {
            if let Some(initializer) = initializer.as_any().downcast_ref::<QalamFunction>() {
                return initializer.arity();
            }
        }
        return 0;
    }

    fn clone_box(&self) -> Box<dyn QalamCallable> {
        return Box::new(self.clone());
    }

    fn to_string(&self) -> String {
        return self.name.to_owned();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
