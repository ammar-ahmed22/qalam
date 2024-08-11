use crate::ast::Expr;
use crate::ast::Visitor;
use crate::ast::expressions::{ Binary, Unary, Literal, Grouping };

pub struct ASTParenString {}

impl ASTParenString {
  fn parenthesize<E, O>(&mut self, name: &String, expr: &Box<E>, other: Option<&Box<O>>) -> String
  where E: Expr, O: Expr
  {
    let mut str = String::new();
    str += "(";
    str += &name;
    str += " ";
    str += &expr.accept(self);
    match other {
      Some(exp) => {
        str += " ";
        str += &exp.accept(self);
      },
      None => {}
    }
    str += ")";
    return str;
  }

  pub fn to_string<E>(&mut self, expr: E) -> String
  where E: Expr
  {
    expr.accept(self)
  }
}

impl Visitor for ASTParenString {
  type R = String;
  fn visit_binary<LT: Expr, RT: Expr>(&mut self, expr: &Binary<LT, RT>) -> Self::R {
      self.parenthesize::<LT, RT>(&expr.operator.lexeme, &expr.left, Some(&expr.right))
  }

  fn visit_grouping<E: Expr>(&mut self, expr: &Grouping<E>) -> Self::R {
      self.parenthesize::<E, E>(&String::from("grouping"), &expr.expression, None)
  }

  fn visit_literal(&mut self, expr: &Literal) -> Self::R {
      match &expr.value {
        Some(val) => {
          if let Some(string_val) = val.downcast_ref::<String>() {
            return string_val.to_owned();
          } else if let Some(num_val) = val.downcast_ref::<f64>() {
            return format!("{}", num_val);
          } else {
            return String::from("Unhandled type.");
          }
        },
        None => {
          return String::from("null");
        }
      }
  }

  fn visit_unary<E: Expr>(&mut self, expr: &Unary<E>) -> Self::R {
      self.parenthesize::<E, E>(&expr.operator.lexeme, &expr.right, None)
  }
}