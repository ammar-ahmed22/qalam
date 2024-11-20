use crate::ast::expr::Expr;
use crate::literal::Literal;
use crate::token::Token;

pub trait ExprVisitor {
    type R;
    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Self::R;
    fn visit_binary(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R;
    fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R;
    fn visit_literal(&mut self, expr: &Option<Literal>) -> Self::R;
    fn visit_unary(&mut self, operator: &Token, right: &Box<Expr>) -> Self::R;
    fn visit_variable(&mut self, name: &Token) -> Self::R;
    fn visit_logical(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R;
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Expr>) -> Self::R;
    fn visit_get(&mut self, object: &Box<Expr>, name: &Token) -> Self::R;
    fn visit_set(&mut self, object: &Box<Expr>, name: &Token, value: &Box<Expr>) -> Self::R;
    fn visit_this(&mut self, keyword: &Token) -> Self::R;
    fn visit_super(&mut self, keyword: &Token, method: &Token) -> Self::R;
    fn visit_array(&mut self, values: &Vec<Expr>) -> Self::R;
    fn visit_get_indexed(
        &mut self,
        object: &Box<Expr>,
        index: &Box<Expr>,
        bracket: &Token,
    ) -> Self::R;
    fn visit_set_indexed(
        &mut self,
        object: &Box<Expr>,
        index: &Box<Expr>,
        value: &Box<Expr>,
        bracket: &Token,
    ) -> Self::R;
}
