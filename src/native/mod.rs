use crate::callable::QalamCallable;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::error::RuntimeError;
use crate::token::Token;
use rand::Rng;
use ordered_float::OrderedFloat;
pub mod clock;
pub mod pow;
pub mod max;
pub mod min;
pub mod num;
pub mod pop;
pub mod push;
pub mod random_int;
pub mod random;
pub mod replace;
pub mod str;
pub mod substr;
pub mod typeof_func;
pub mod len;
pub mod indexof;
pub mod array_constructor;
pub mod code;
pub mod floor;
pub mod ceil;
pub mod round;
pub mod slice;


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