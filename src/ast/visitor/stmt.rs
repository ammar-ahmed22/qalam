use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::token::Token;

pub trait StmtVisitor {
    type R;
    fn visit_expression(&mut self, expression: &Expr) -> Self::R;
    fn visit_print(&mut self, expression: &Expr) -> Self::R;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::R;
    fn visit_block(&mut self, statements: &mut Vec<Stmt>) -> Self::R;
    fn visit_if(
        &mut self,
        condition: &Expr,
        then: &mut Box<Stmt>,
        else_branch: &mut Option<Box<Stmt>>,
    ) -> Self::R;
    fn visit_while(&mut self, condition: &Expr, body: &mut Box<Stmt>) -> Self::R;
    fn visit_function(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &mut Vec<Stmt>,
    ) -> Self::R;
    fn visit_return(&mut self, keyword: &Token, value: &Option<Expr>) -> Self::R;
    fn visit_class(
        &mut self,
        name: &Token,
        methods: &mut Vec<Stmt>,
        superclass: &Option<Expr>,
    ) -> Self::R;
}
