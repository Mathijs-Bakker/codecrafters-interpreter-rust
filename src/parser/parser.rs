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
pub enum TokenType<'a> {
    Bool(bool),
    Number(f32),
    Nil,
    String(&'a str),
    Parenthesize(&'a str, Vec<TokenType<'a>>),
}

impl<'a> fmt::Display for TokenType<'a> {
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
            TokenType::String(s) => write!(f, "{}", s.trim_matches('"')),
            TokenType::Parenthesize(s, tokentree) => {
                write!(f, "({s}")?;

                for tokentype in tokentree {
                    write!(f, " {tokentype}")?
                }

                write!(f, ")")
            }
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

        let expr = match expression {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(e),
            None => return Err(Box::new(ParseError {})),
        };

        let xpr = match expr {
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
            Token {
                kind: TokenKind::String,
                character,
            } => TokenType::String(character),
            Token {
                kind: TokenKind::LeftParen,
                ..
            } => {
                let xpr = self.parse_expression()?;
                TokenType::Parenthesize("group", vec![xpr])
            }
            Token {
                kind: TokenKind::Bang,
                ..
            } => {
                let xpr = self.parse_expression()?;
                TokenType::Parenthesize("!", vec![xpr])
            }
            Token {
                kind: TokenKind::Minus,
                ..
            } => {
                let xpr = self.parse_expression()?;
                TokenType::Parenthesize("-", vec![xpr])
            }
            _ => todo!(),
        };

        Ok(xpr)
    }
}
