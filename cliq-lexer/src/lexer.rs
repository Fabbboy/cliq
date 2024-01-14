extern crate nom;
extern crate regex;

use nom::{
  character::complete::{digit1, multispace1, one_of},
  IResult,
};

use regex::Regex;

use crate::codepos::Codepos;
use crate::token::Token;
use crate::token_t::TokenT;

pub struct Lexer {
  line: usize,
  col: usize,
}

impl Lexer {
  pub fn new() -> Lexer {
    Lexer { line: 1, col: 1 }
  }

  fn update_codepos(&mut self, text: &str) {
    for c in text.chars() {
      if c == '\n' {
        self.line += 1;
        self.col = 1;
      } else {
        self.col += 1;
      }
    }
  }

  fn parse_and_format_float(input: &str) -> Result<String, std::num::ParseFloatError> {
    let formatted_input = if input.starts_with('.') {
      format!("0{}", input)
    } else {
      input.to_string()
    };
    let float_value: f64 = formatted_input.parse()?;
    Ok(float_value.to_string())
  }

  fn lex_whitespace<'a>(&mut self, input: &'a str) -> IResult<&'a str, Token> {
    let (input, matched) = multispace1(input)?;
    self.update_codepos(matched);
    let codepos = Codepos::new(self.line, self.col, None);
    let token = Token::new(TokenT::WHITESPACE, "".to_string(), codepos);
    Ok((input, token))
  }

  fn lex_integer<'a>(&mut self, input: &'a str) -> IResult<&'a str, Token> {
    let (input, value) = digit1(input)?;
    let value = value.to_string();
    self.update_codepos(&value);
    let codepos = Codepos::new(self.line, self.col, None);
    let token = Token::new(TokenT::INTEGER, value, codepos);
    Ok((input, token))
  }

  fn lex_float<'a>(&mut self, input: &'a str) -> IResult<&'a str, Token> {
    let float_regex = Regex::new(r"^\d*\.\d+").unwrap();
    if let Some(mat) = float_regex.find(input) {
      let matched = &input[..mat.end()];
      self.update_codepos(matched);
      let codepos = Codepos::new(self.line, self.col, None);
      let corrected = Lexer::parse_and_format_float(matched).unwrap();
      let token = Token::new(TokenT::FLOAT, corrected, codepos);
      Ok((&input[mat.end()..], token))
    } else {
      Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))
    }
  }

  fn lex_operator<'a>(&mut self, input: &'a str) -> IResult<&'a str, Token> {
    let (input, matched) = one_of("+-*/%=")(input)?;
    self.update_codepos(String::from(matched).as_str());
    let codepos = Codepos::new(self.line, self.col, None);
    let token = Token::new(TokenT::OPERATOR, matched.to_string(), codepos);
    Ok((input, token))
  }

  fn lex_bracket<'a>(&mut self, input: &'a str) -> IResult<&'a str, Token> {
    //=> (), [], {}
    let (input, matched) = one_of("()[]{}")(input)?;
    self.update_codepos(String::from(matched).as_str());
    let codepos = Codepos::new(self.line, self.col, None);
    let token = Token::new(TokenT::BRACKET, matched.to_string(), codepos);
    Ok((input, token))
  }

  pub fn lex(&mut self, input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current_input = input;

    while !current_input.is_empty() {
      let token_result = if let Ok(result) = self.lex_whitespace(current_input) {
        result
      } else if let Ok(result) = self.lex_float(current_input) {
        result
      } else if let Ok(result) = self.lex_integer(current_input) {
        result
      } else if let Ok(result) = self.lex_operator(current_input) {
        result
      } else if let Ok(result) = self.lex_bracket(current_input) {
        result
      } else {
        return Err(format!("Unexpected character at line {}, column {}", self.line, self.col));
      };

      let (next_input, token) = token_result;
      current_input = next_input;
      tokens.push(token);
    }

    Ok(tokens)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lexer() {
    let mut lexer = Lexer::new();
    let input = "  123 + 321 * 123 / 312 - 123";
    let tokens = lexer.lex(input).unwrap();
    println!("{:#?}", tokens);
  }

  #[test]
  fn test_lexer_bracket() {
    let mut lexer = Lexer::new();
    let input = "  (123 + 321) * 123 / 312 - 123";
    let tokens = lexer.lex(input).unwrap();
    println!("{:#?}", tokens);
  }
}
