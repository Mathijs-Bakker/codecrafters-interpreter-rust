use crate::lexer::analyzer::Scanner;
use crate::lexer::analyzer::Token;
use crate::lexer::token::TokenKind;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError",)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub enum TokenType {
    Bool(bool),
    Number(f32),
    Nil,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Bool(b) => write!(f, "{b}"),
            TokenType::Number(n) => {
                if *n == n.trunc() {
                    write!(f, "{n}.0")
                } else {
                    write!(f, "{n}")
                }
            }
            TokenType::Nil => write!(f, "nil"),
        }
    }
}

pub struct Parser<'a> {
    pub scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            scanner: Scanner::new(input),
        }
    }

    pub fn parse_expression(&mut self) -> Result<TokenType, Box<dyn Error + Send + Sync>> {
        let expression = self.scanner.next();

        let xpr = match expression {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(e),
            None => return Err(Box::new(ParseError {})),
        };

        let xpr = match xpr {
            Token {
                kind: TokenKind::True,
                ..
            } => TokenType::Bool(true),
            Token {
                kind: TokenKind::False,
                ..
            } => TokenType::Bool(false),
            Token {
                kind: TokenKind::Number(n),
                ..
            } => TokenType::Number(n),
            Token {
                kind: TokenKind::Nil,
                ..
            } => TokenType::Nil,
            _ => todo!(),
        };

        Ok(xpr)
    }
}
