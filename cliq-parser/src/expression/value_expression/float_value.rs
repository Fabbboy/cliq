use serde::Serialize;

use crate::expression::Expression;

use super::ValueExpression;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FloatValue {
  value: f32,
}

impl FloatValue {
  pub fn new(val: f32) -> FloatValue {
    return FloatValue { value: val };
  }

  pub fn expression(val: f32) -> Expression {
    Expression::ValueExpression(ValueExpression::FloatValue(FloatValue::new(val)))
  }

  pub fn value(&self) -> f32 {
    self.value
  }
}
