pub mod visitor;
pub mod expressions;
pub mod utils;
use visitor::Visitor;

pub trait Expr {
  fn accept<V>(&self, visitor: &mut V) -> V::R
  where V: Visitor;
}