use serde::Serialize;

use self::float_value::FloatValue;
use self::int_value::IntValue;

use super::Expression;

pub mod float_value;
pub mod int_value;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ValueExpression {
  IntValue(int_value::IntValue),
  FloatValue(float_value::FloatValue),
}

impl ValueExpression {
  pub fn int_value(val: i32) -> Expression {
    IntValue::expression(val)
  }

  pub fn float_value(val: f32) -> Expression {
    FloatValue::expression(val)
  }
}
