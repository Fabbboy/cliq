use crate::expression::binary_expression::BinaryExpr;
use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct DivOpr {
  lhs: Box<Expression>,
  rhs: Box<Expression>,
}

impl DivOpr {
  pub fn new(lhs: Expression, rhs: Expression) -> DivOpr {
    return DivOpr {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
    };
  }

  pub fn expression(lhs: Expression, rhs: Expression) -> Expression {
    Expression::BinaryExpression(BinaryExpr::DivOpr(DivOpr::new(lhs, rhs)))
  }

  pub fn lhs(&self) -> &Expression {
    &self.lhs
  }

  pub fn rhs(&self) -> &Expression {
    &self.rhs
  }
}
