use serde::Serialize;
use crate::expression::Expression;

#[derive(Debug, Clone, Serialize)]
pub enum Statement{
  Expression(Expression)
}