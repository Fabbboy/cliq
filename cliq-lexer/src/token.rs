use crate::{token_t::TokenT, codepos::Codepos};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Token {
  pub token_t: TokenT,
  pub value: String,
  pub codepos: Codepos,
}

impl Token {
  pub fn new(token_t: TokenT, value: String, codepos: Codepos) -> Self {
    Self {
      token_t,
      value,
      codepos,
    }
  }
}