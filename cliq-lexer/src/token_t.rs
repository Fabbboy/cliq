use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TokenT {
  WHITESPACE,

  //VALUES
  INTEGER,
  FLOAT,
  IDENTIFIER,

  //OPERATOR
  OPERATOR, // + * - / % =

  //SYMBOL
  BRACKET, // ( ) [ ] { }

  //KEYWORD
  VAR, //used for variable declaration always immutable
}
