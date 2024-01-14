use serde::Serialize;

use crate::expression::Expression;

use super::ValueExpression;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IntValue {
  value: i32,
}

impl IntValue {
  pub fn new(val: i32) -> IntValue {
    return IntValue { value: val };
  }

  pub fn expression(val: i32) -> Expression {
    Expression::ValueExpression(ValueExpression::IntValue(IntValue::new(val)))
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}
