use cliq_lexer::{token::Token, token_t::TokenT};

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

struct LangError {
  msg: String,
}

impl LangError {
  fn new(msg: &str) -> LangError {
    LangError { msg: msg.to_string() }
  }
}

impl std::fmt::Debug for LangError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self.msg)
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

  //this function get the 1 token before and after a given token and returns the stream
  //its used to make debugging and error printing more useful
  //like rust does with its error messages
  //also mark the token in question with a '^' to make it easier to spot
  fn get_token_stream(&self, token: &Token) -> Vec<String> {
    let mut token_stream = vec![];
    let mut token_index = 0;
    for t in self.token_stream.iter() {
      if t == token {
        token_index = token_stream.len();
      }
      token_stream.push(t.value.clone());
    }
    token_stream[token_index] = format!("^{}", token_stream[token_index]);
    token_stream
  }

  fn next(&mut self, expected_token: Vec<TokenT>, token_has_value: Option<Vec<String>>) -> Result<Token, LangError> {
    if self.current_token < self.stream_size {
      let token = &self.token_stream[self.current_token];
      if expected_token.contains(&token.token_t) {
        if let Some(values) = token_has_value {
          if values.contains(&token.value) {
            self.current_token += 1;
            return Ok(token.clone());
          } else {
            return Err(LangError::new(
              format!(
                "Expected token value: {:?} but got: {:?}\nHere: {:#?}",
                values,
                token.value,
                self.get_token_stream(token)
              )
              .as_str(),
            ));
          }
        } else {
          self.current_token += 1;
          return Ok(token.clone());
        }
      } else {
        return Err(LangError::new(
          format!(
            "Expected token type: {:?} but got: {:?}\nHere: {:#?}",
            expected_token,
            token.token_t,
            self.get_token_stream(token)
          )
          .as_str(),
        ));
      }
    } else {
      return Err(LangError::new("Unexpected end of token stream"));
    }
  }

  fn peek(&self, expected_token: Vec<TokenT>, token_has_value: Option<Vec<String>>) -> Result<bool, LangError> {
    if self.current_token < self.stream_size {
      let token = &self.token_stream[self.current_token];
      if expected_token.contains(&token.token_t) {
        if let Some(values) = token_has_value {
          if values.contains(&token.value) {
            return Ok(true);
          } else {
            return Ok(false);
          }
        } else {
          return Ok(true);
        }
      } else {
        return Ok(false);
      }
    } else {
      return Ok(false);
    }
  }

  fn parse_value(&mut self, token: Token) -> Result<Expression, LangError> {
    match token.token_t {
      TokenT::INTEGER => {
        let val = token.value.parse::<i32>().unwrap();
        Ok(ValueExpression::int_value(val))
      }
      _ => unreachable!(),
    }
  }

  fn next_expression<'a>(&mut self) -> Result<Expression, LangError> {
    let token = self.next(vec![TokenT::INTEGER, TokenT::FLOAT, TokenT::BRACKET], None)?;
    Ok(match token.token_t {
      TokenT::INTEGER | TokenT::FLOAT => self.parse_value(token)?,
      TokenT::BRACKET => {
        let value = token.value.clone();
        return match value.as_str() {
          "(" => {
            let expr = self.parse_expression()?;
            self.next(vec![TokenT::BRACKET], Some(vec![")".to_string()]))?;
            Ok(expr)
          }

          "{" => {
            unimplemented!();
          }
          _ => unreachable!(),
        };
      }

      _ => unreachable!(),
    })
  }

  fn parse_high_precedence_expr<'a>(&mut self) -> Result<Expression, LangError> {
    let mut expr = self.next_expression()?;

    while self.current_token < self.stream_size {
      match self.peek(vec![TokenT::OPERATOR], None)? {
        true => {
          let operator = self.next(vec![TokenT::OPERATOR], None)?;
          let operator_value = operator.value.clone();

          if get_precedence(&operator_value) != 2 {
            self.current_token -= 1; // Step back to re-evaluate this operator in parse_expression
            break;
          }

          let next_expr = self.next_expression()?;
          expr = match operator_value.as_str() {
            "*" => MulOpr::expression(expr, next_expr),
            "/" => DivOpr::expression(expr, next_expr),
            _ => unreachable!(),
          };
        }
        _ => break,
      }
    }
    Ok(expr)
  }

  fn parse_expression<'a>(&mut self) -> Result<Expression, LangError> {
    let mut expr = self.parse_high_precedence_expr()?;

    while self.current_token < self.stream_size {
      match self.peek(vec![TokenT::OPERATOR], None)? {
        true => {
          let operator = self.next(vec![TokenT::OPERATOR], None)?;
          let operator_value = operator.value.clone();

          if get_precedence(&operator_value) == 1 {
            let next_expr = self.parse_high_precedence_expr()?;
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
    Ok(expr)
  }

  fn parse_statement(&mut self) -> Statement {
    let token = self.next(vec![], None);
    if token.is_ok() {
      unimplemented!();
    } else {
      let expr = self.parse_expression().unwrap();
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

  use crate::{expression::{
    binary_expression::{add_opr::AddOpr, mul_opr::MulOpr},
    value_expression::ValueExpression,
    Expression,
  }, statement::Statement};

  #[test]
  fn clear_whitespaces() {
    let mut lexer = Lexer::new();
    let input = "  123 + 321 * 123 / 312 - 123";
    let tokens = lexer.lex(input).unwrap();
    let mut parser = super::Parser::new(tokens);
    parser.clear_whitespaces();
    println!("{:#?}", parser.token_stream);
    assert_eq!(parser.token_stream.len(), 9);
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

    assert_eq!(expr.is_ok(), true);
    let expr = expr.unwrap();
    assert_eq!(expr, ValueExpression::int_value(123));
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
    let input = "4 + 3 * 2 / 7 - 5";
    let tokens = lexer.lex(input).unwrap();
    let mut parser = super::Parser::new(tokens);
    let ast = parser.parse();
    println!("Parsed AST for '{}':\n{:#?}", input, ast);
  }

  #[test]
  fn test_bracketed_expression() {
    let mut lexer = Lexer::new();
    let input = "(4 + 3) * 2";
    let tokens = lexer.lex(input).unwrap();
    let mut parser = super::Parser::new(tokens);
    let ast = parser.parse();
    println!("Parsed AST for '{}':\n{:#?}", input, ast);
  }
}
