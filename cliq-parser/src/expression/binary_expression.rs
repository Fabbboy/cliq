use serde::Serialize;

pub mod add_opr;
pub mod div_opr;
pub mod mul_opr;
pub mod sub_opr;


#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum BinaryExpr {
  AddOpr(add_opr::AddOpr),
  SubOpr(sub_opr::SubOpr),
  MulOpr(mul_opr::MulOpr),
  DivOpr(div_opr::DivOpr),
}