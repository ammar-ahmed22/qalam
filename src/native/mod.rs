use crate::callable::QalamCallable;
use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::token::Token;
use ordered_float::OrderedFloat;
use rand::Rng;
pub mod array_constructor;
pub mod ceil;
pub mod clock;
pub mod code;
pub mod floor;
pub mod indexof;
pub mod len;
pub mod max;
pub mod min;
pub mod num;
pub mod pop;
pub mod pow;
pub mod push;
pub mod random;
pub mod random_int;
pub mod replace;
pub mod round;
pub mod slice;
pub mod str;
pub mod substr;
pub mod typeof_func;

pub fn is_neg(num: f64) -> bool {
    return num < 0.0;
}

pub fn is_int(num: f64) -> bool {
    return num.fract() == 0.0;
}

pub fn is_usize(num: f64) -> bool {
    if is_neg(num) {
        return false;
    }

    if !is_int(num) {
        return false;
    }

    return true;
}
