use self::int_value::IntValue;

use super::Expression;

pub mod int_value;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueExpression {
  IntValue(int_value::IntValue),
}

impl ValueExpression {
  pub fn int_value(val: i32) -> Expression {
    IntValue::expression(val)
  }

}
