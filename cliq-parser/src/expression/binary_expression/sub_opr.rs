use serde::Serialize;

use crate::expression::binary_expression::BinaryExpr;
use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SubOpr {
  lhs: Box<Expression>,
  rhs: Box<Expression>,
}

impl SubOpr {
  pub fn new(lhs: Expression, rhs: Expression) -> SubOpr {
    return SubOpr {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
    };
  }

  pub fn expression(lhs: Expression, rhs: Expression) -> Expression {
    Expression::BinaryExpression(BinaryExpr::SubOpr(SubOpr::new(lhs, rhs)))
  }

  pub fn lhs(&self) -> &Expression {
    &self.lhs
  }

  pub fn rhs(&self) -> &Expression {
    &self.rhs
  }
}
