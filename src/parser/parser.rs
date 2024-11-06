use crate::lexer::analyzer::Scanner;
use crate::lexer::analyzer::Token;
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

pub struct Parser<'a> {
    pub scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            scanner: Scanner::new(input),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Token, Box<dyn Error + Send + Sync>> {
        let expression = self.scanner.next();

        match expression {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e),
            None => Err(Box::new(ParseError {})),
        }
    }
}
