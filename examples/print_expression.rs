use qalam::ast::utils::ASTParenString;
use qalam::ast::expressions::Expr;
use qalam::token::{ Token, TokenType };

fn main() {
  let mut generator = ASTParenString {};
  
  // Equivalent to -123 * (45.67)
  let expr = Expr::Binary {
    left: Box::new(
      Expr::Unary {
        operator: Token::init(TokenType::Minus, &String::from("-"), None, 1),
        right: Box::new(Expr::Literal { value: Some(Box::new(123.0)) })
      }
    ),
    operator: Token::init(TokenType::Star, &String::from("*"), None, 1),
    right: Box::new(
      Expr::Grouping { expression: Box::new(Expr::Literal { value: Some(Box::new(45.67)) }) }
    )
  };

  println!("{}", generator.to_string(expr));
}