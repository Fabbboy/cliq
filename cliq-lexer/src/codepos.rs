use serde::Serialize;
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Codepos {
  pub line: usize,
  pub col: usize,
  pub file: Option<String>,
}

impl Codepos {
  pub fn new(line: usize, col: usize, file: Option<String>) -> Self {
    Self { line, col, file }
  }

  pub fn zero() -> Self {
    Self::new(0, 0, None)
  }
}
