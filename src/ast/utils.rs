use crate::ast::expressions::Expr;
use crate::ast::visitor::Visitor;
// use crate::ast::expressions::{ Binary, Unary, Literal, Grouping };

pub struct ASTParenString {}

impl ASTParenString {
  fn parenthesize(&mut self, name: &str, exprs: &[&Box<Expr>]) -> String {
    let mut str = String::new();
    str += "(";
    str += &name;
    for expr in exprs.iter() {
      str += " ";
      str += &expr.accept(self);
    }
    str += ")";
    return str;
  }

  pub fn to_string(&mut self, expr: Expr) -> String
  {
    expr.accept(self)
  }
}

impl Visitor for ASTParenString {
  type R = String;
  fn visit_binary(&mut self, left: &Box<Expr>, operator: &crate::token::Token, right: &Box<Expr>) -> Self::R {
      self.parenthesize(&operator.lexeme, &[left, right])
  }

  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R {
      self.parenthesize("grouping", &[expression])
  }

  fn visit_literal(&mut self, expr: &Option<Box<dyn std::any::Any>>) -> Self::R {
      match expr {
        Some(val) => {
          if let Some(string_val) = val.downcast_ref::<&str>() {
            return string_val.to_string()
          } else if let Some(num_val) = val.downcast_ref::<f64>() {
            return format!("{}", num_val);
          } else if let Some(bool_val) = val.downcast_ref::<bool>() {
            return format!("{}", bool_val)
          } else {
            return "Unhandled type".to_string()
          }
        },
        None => {
          return "null".to_string();
        }
      }
  }

  fn visit_unary(&mut self, operator: &crate::token::Token, right: &Box<Expr>) -> Self::R {
      self.parenthesize(&operator.lexeme, &[right])
  }
  // fn visit_binary<LT: Expr, RT: Expr>(&mut self, expr: &Binary<LT, RT>) -> Self::R {
  //     self.parenthesize::<LT, RT>(&expr.operator.lexeme, &expr.left, Some(&expr.right))
  // }

  // fn visit_grouping<E: Expr>(&mut self, expr: &Grouping<E>) -> Self::R {
  //     self.parenthesize::<E, E>(&String::from("grouping"), &expr.expression, None)
  // }

  // fn visit_literal(&mut self, expr: &Literal) -> Self::R {
  //     match &expr.value {
  //       Some(val) => {
  //         if let Some(string_val) = val.downcast_ref::<String>() {
  //           return string_val.to_owned();
  //         } else if let Some(num_val) = val.downcast_ref::<f64>() {
  //           return format!("{}", num_val);
  //         } else {
  //           return String::from("Unhandled type.");
  //         }
  //       },
  //       None => {
  //         return String::from("null");
  //       }
  //     }
  // }

  // fn visit_unary<E: Expr>(&mut self, expr: &Unary<E>) -> Self::R {
  //     self.parenthesize::<E, E>(&expr.operator.lexeme, &expr.right, None)
  // }
}