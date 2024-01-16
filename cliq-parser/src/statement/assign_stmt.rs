use serde::Serialize;

use crate::expression::Expression;

use super::Statement;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AssignStmt{
  pub variable: Expression,
  pub value: Expression,
  pub mutable: bool,
}

impl AssignStmt{
  pub fn new(variable: Expression, value: Expression,mutable: bool) -> Self{
    Self{
      variable,
      value,
      mutable,
    }
  }

  pub fn statement(variable: Expression, value: Expression, mutable: bool) -> Statement{
    Statement::Assign(Self::new(variable, value, mutable))
  }
}