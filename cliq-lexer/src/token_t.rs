use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TokenT {
  WHITESPACE,

  //VALUES
  INTEGER,
  FLOAT,

  //OPERATOR
  OPERATOR, // + * - / % =

  //SYMBOL
  BRACKET, // ( ) [ ] { }
}
