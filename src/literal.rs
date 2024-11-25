use crate::callable::instance::QalamInstance;
use crate::callable::QalamCallable;
use crate::hashable::HashableRcRefCell;
use colored::Colorize;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QalamArray {
    pub elements: Vec<Option<Literal>>,
}

impl QalamArray {
    pub fn init() -> Self {
        return Self {
            elements: Vec::new(),
        };
    }

    pub fn construct(size: usize, val: Option<Literal>) -> Self {
        return Self {
            elements: vec![val; size],
        };
    }

    pub fn from_vec(vec: Vec<Option<Literal>>) -> Self {
        return Self { elements: vec };
    }

    pub fn to_string(&self) -> String {
        let values = self
            .elements
            .iter()
            .map(|opt| Literal::option_string(opt.clone()))
            .collect::<Vec<String>>()
            .join(", ");
        return format!("[ {} ]", values);
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub enum Literal {
    Number(OrderedFloat<f64>),
    String(String),
    Bool(bool),
    Callable(Box<dyn QalamCallable>),
    Instance(HashableRcRefCell<QalamInstance>),
    Array(HashableRcRefCell<QalamArray>),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => a == b,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            (Literal::Callable(a), Literal::Callable(b)) => std::ptr::eq(&**a, &**b),
            (Literal::Array(a), Literal::Array(b)) => a == b,
            (Literal::Instance(a), Literal::Instance(b)) => a == b,
            _ => false,
        }
    }
}

impl Literal {
    pub fn to_qalam_string(&self) -> String {
        match self {
            Self::Bool(val) => format!(
                "{}",
                if *val {
                    "haqq".yellow().to_string()
                } else {
                    "batil".yellow().to_string()
                }
            ),
            Self::Number(val) => format!("{}", val).yellow().to_string(),
            Self::String(val) => val.to_owned(),
            Self::Callable(val) => val.to_string().cyan().to_string(),
            Self::Instance(val) => val.0.borrow().to_string().cyan().to_string(),
            Self::Array(val) => val.0.borrow().to_string(),
        }
    }

    pub fn option_string(value: Option<Literal>) -> String {
        match value {
            Some(val) => val.to_qalam_string(),
            None => String::from("ghaib"),
        }
    }
}
