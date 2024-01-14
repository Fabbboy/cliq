use serde::Serialize;

use crate::expression::binary_expression::BinaryExpr;
use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AddOpr {
  lhs: Box<Expression>,
  rhs: Box<Expression>,
}

impl AddOpr {
  pub fn new(lhs: Expression, rhs: Expression) -> AddOpr {
    return AddOpr {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
    };
  }

  pub fn expression(lhs: Expression, rhs: Expression) -> Expression {
    Expression::BinaryExpression(BinaryExpr::AddOpr(AddOpr::new(lhs, rhs)))
  }

  pub fn lhs(&self) -> &Expression {
    &self.lhs
  }

  pub fn rhs(&self) -> &Expression {
    &self.rhs
  }
}
