pub mod binary_expression;
pub mod value_expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  ValueExpression(value_expression::ValueExpression),
  BinaryExpression(binary_expression::BinaryExpr),
}
