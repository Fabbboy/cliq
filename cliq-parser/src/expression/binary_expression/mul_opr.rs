use crate::expression::binary_expression::BinaryExpr;
use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct MulOpr {
  lhs: Box<Expression>,
  rhs: Box<Expression>,
}

impl MulOpr {
  pub fn new(lhs: Expression, rhs: Expression) -> MulOpr {
    return MulOpr {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
    };
  }

  pub fn expression(lhs: Expression, rhs: Expression) -> Expression {
    Expression::BinaryExpression(BinaryExpr::MulOpr(MulOpr::new(lhs, rhs)))
  }

  pub fn lhs(&self) -> &Expression {
    &self.lhs
  }

  pub fn rhs(&self) -> &Expression {
    &self.rhs
  }
}
