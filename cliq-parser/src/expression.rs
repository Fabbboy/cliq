use serde::Serialize;

pub mod binary_expression;
pub mod value_expression;
pub mod variable_expression;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expression {
  ValueExpression(value_expression::ValueExpression),
  BinaryExpression(binary_expression::BinaryExpr),
  VariableExpression(variable_expression::VariableExpression),
}
