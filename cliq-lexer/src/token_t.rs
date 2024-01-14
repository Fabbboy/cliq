#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
