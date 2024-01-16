use serde::Serialize;

use super::Expression;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VariableExpression {
  pub name: String,
}

impl VariableExpression {
  pub fn new(name: String) -> Self {
    Self { name }
  }

  pub fn expression(name:String) -> Expression{
    Expression::VariableExpression(Self::new(name))
  }
}