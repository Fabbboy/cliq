extern crate nom;

use cliq_lexer::{token::Token, token_t::TokenT};
use nom::{
  branch::alt,
  combinator::map,
  error::Error,
  sequence::{delimited, tuple},
  IResult,
};

use crate::{
  expression::{
    binary_expression::{add_opr::AddOpr, div_opr::DivOpr, mul_opr::MulOpr, sub_opr::SubOpr},
    value_expression::ValueExpression,
    Expression,
  },
  statement::Statement,
};

pub struct Parser {
  //module
  //stmts: Vec<Statement>,
  token_stream: Vec<Token>,
  current_token: usize,
  stream_size: usize,
}

fn get_precedence(opr: &str) -> i32 {
  match opr {
    "+" => 1,
    "-" => 1,
    "*" => 2,
    "/" => 2,
    "%" => 2,
    _ => 0,
  }
}

//we want to strictly use nom to parse the tokens
impl Parser {
  pub fn new(input_tokens: Vec<Token>) -> Parser {
    Parser {
      stream_size: input_tokens.len(),
      token_stream: input_tokens,
      current_token: 0,
    }
  }

  fn clear_whitespaces(&mut self) {
    let mut new_tokens = vec![];
    for token in self.token_stream.iter() {
      if token.token_t != TokenT::WHITESPACE {
        new_tokens.push(token.clone());
      }
    }
    self.token_stream = new_tokens;
    self.stream_size = self.token_stream.len();
  }

  fn next<'a>(&mut self, expected_token: Vec<TokenT>, token_has_value: Option<Vec<String>>) -> IResult<&'a str, Token> {
    if self.current_token < self.stream_size {
      let token = &self.token_stream[self.current_token];
      if expected_token.contains(&token.token_t) {
        if let Some(values) = token_has_value {
          if values.contains(&token.value) {
            self.current_token += 1;
            return Ok(("", token.clone()));
          } else {
            return Err(nom::Err::Error(Error::new("Unexpected value", nom::error::ErrorKind::Tag)));
          }
        } else {
          self.current_token += 1;
          return Ok(("", token.clone()));
        }
      } else {
        return Err(nom::Err::Error(Error::new("Unexpected token", nom::error::ErrorKind::Tag)));
      }
    } else {
      return Err(nom::Err::Error(Error::new("Unexpected EOF", nom::error::ErrorKind::Tag)));
    }
  }

  fn peek<'a>(&self, expected_token: Vec<TokenT>, token_has_value: Option<Vec<String>>) -> IResult<&'a str, bool> {
    if self.current_token < self.stream_size {
      let token = &self.token_stream[self.current_token];
      if expected_token.contains(&token.token_t) {
        if let Some(values) = token_has_value {
          if values.contains(&token.value) {
            return Ok(("", true));
          } else {
            return Ok(("", false));
          }
        } else {
          return Ok(("", true));
        }
      } else {
        return Ok(("", false));
      }
    } else {
      return Ok(("", false));
    }
  }

  fn parse_value<'a>(&mut self, token: Token) -> IResult<&'a str, Expression> {
    match token.token_t {
      TokenT::INTEGER => {
        let val = token.value.parse::<i32>().unwrap();
        Ok(("", ValueExpression::int_value(val)))
      }
      _ => unreachable!(),
    }
  }

  fn next_expression<'a>(&mut self) -> IResult<&'a str, Expression> {
    let token = self.next(vec![TokenT::INTEGER, TokenT::FLOAT], None)?;
    Ok(match token.1.token_t {
      TokenT::INTEGER | TokenT::FLOAT => self.parse_value(token.1)?,

      _ => unreachable!(),
    })
  }

  fn parse_high_precedence_expr<'a>(&mut self) -> IResult<&'a str, Expression> {
    let mut expr = self.next_expression()?.1;

    while self.current_token < self.stream_size {
      match self.peek(vec![TokenT::OPERATOR], None)? {
        (_, true) => {
          let operator = self.next(vec![TokenT::OPERATOR], None)?.1;
          let operator_value = operator.value.clone();

          if get_precedence(&operator_value) != 2 {
            self.current_token -= 1; // Step back to re-evaluate this operator in parse_expression
            break;
          }

          let next_expr = self.next_expression()?.1;
          expr = match operator_value.as_str() {
            "*" => MulOpr::expression(expr, next_expr),
            "/" => DivOpr::expression(expr, next_expr),
            _ => unreachable!(),
          };
        }
        _ => break,
      }
    }
    Ok(("", expr))
  }

  fn parse_expression<'a>(&mut self) -> IResult<&'a str, Expression> {
    let mut expr = self.parse_high_precedence_expr()?.1;

    while self.current_token < self.stream_size {
      match self.peek(vec![TokenT::OPERATOR], None)? {
        (_, true) => {
          let operator = self.next(vec![TokenT::OPERATOR], None)?.1;
          let operator_value = operator.value.clone();

          if get_precedence(&operator_value) == 1 {
            let next_expr = self.parse_high_precedence_expr()?.1;
            expr = match operator_value.as_str() {
              "+" => AddOpr::expression(expr, next_expr),
              "-" => SubOpr::expression(expr, next_expr),
              _ => unreachable!(),
            };
          }
        }
        _ => break,
      }
    }
    Ok(("", expr))
  }

  fn parse_statement(&mut self) -> Statement {
    let token = self.next(vec![], None);
    if token.is_ok() {
      unimplemented!();
    } else {
      //it only can be a expression
      let expr = self.parse_expression().unwrap().1;
      Statement::Expression(expr)
    }
  }

  pub fn parse(&mut self) -> Vec<Statement> {
    self.clear_whitespaces();
    let mut stmts = vec![];
    while self.token_stream.get(self.current_token) != None {
      stmts.push(self.parse_statement());
    }
    stmts
  }
}

#[cfg(test)]
mod tests {
  use cliq_lexer::lexer::Lexer;

  #[test]
  fn clear_whitespaces() {
    let mut lexer = Lexer::new();
    let input = "  123 + 321 * 123 / 312 - 123";
    let tokens = lexer.lex(input).unwrap();
    let mut parser = super::Parser::new(tokens);
    parser.clear_whitespaces();
    println!("{:#?}", parser.token_stream);
  }

  #[test]
  fn test_next_expression() {
    let mut lexer = Lexer::new();
    let input = "  123";
    let tokens = lexer.lex(input).unwrap();
    let mut parser = super::Parser::new(tokens);
    parser.clear_whitespaces();
    let expr = parser.next_expression();
    println!("{:#?}", expr);
  }

  #[test]
  fn test_binary_expr() {
    let mut lexer = Lexer::new();
    let input = "123 + 321 * 333";
    let tokens = lexer.lex(input).unwrap();
    let mut parser = super::Parser::new(tokens);
    let ast = parser.parse();
    println!("{:#?}", ast);
  }

  #[test]
  fn test_complex_expression() {
    let mut lexer = Lexer::new();
    let input = "4 + 3 - 2 + 7 - 5";
    let tokens = lexer.lex(input).unwrap();
    let mut parser = super::Parser::new(tokens);
    let ast = parser.parse();
    println!("Parsed AST for '{}':\n{:#?}", input, ast);
  }
}
