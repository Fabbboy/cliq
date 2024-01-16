use serde::Serialize;
use crate::expression::Expression;

pub mod assign_stmt;

#[derive(Debug, Clone, Serialize)]
pub enum Statement{
  Expression(Expression),
  Assign(assign_stmt::AssignStmt),
}