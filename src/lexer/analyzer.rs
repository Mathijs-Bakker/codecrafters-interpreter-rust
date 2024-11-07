pub(crate) use crate::lexer::scanner_error;
pub(crate) use crate::lexer::token::{Token, TokenKind};
use std::error::Error;

#[derive(Debug)]
pub struct Scanner<'a> {
    source_code: &'a str,
    lox_remaining: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source_code: input,
            lox_remaining: input,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, Box<dyn Error + Send + Sync>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut chars = self.lox_remaining.chars();
            let c = chars.next()?;
            let c_str = &self.lox_remaining[..c.len_utf8()];
            let chars_remaining = self.lox_remaining;
            self.lox_remaining = chars.as_str();

            enum LongLexemes {
                OperatorOrSingleChar(TokenKind, TokenKind),
                Slash,
                String,
                Number,
                Identifier,
            }

            let build_token = move |kind: TokenKind| {
                Some(Ok(Token {
                    kind,
                    character: c_str,
                }))
            };

            let longer_lexemes = match c {
                '(' => return build_token(TokenKind::LeftParen),
                ')' => return build_token(TokenKind::RightParen),
                '{' => return build_token(TokenKind::LeftBrace),
                '}' => return build_token(TokenKind::RightBrace),
                '*' => return build_token(TokenKind::Star),
                '.' => return build_token(TokenKind::Dot),
                ',' => return build_token(TokenKind::Comma),
                '+' => return build_token(TokenKind::Plus),
                '-' => return build_token(TokenKind::Minus),
                ';' => return build_token(TokenKind::Semicolon),
                '!' => LongLexemes::OperatorOrSingleChar(TokenKind::BangEqual, TokenKind::Bang),
                '<' => LongLexemes::OperatorOrSingleChar(TokenKind::LessEqual, TokenKind::Less),
                '>' => {
                    LongLexemes::OperatorOrSingleChar(TokenKind::GreaterEqual, TokenKind::Greater)
                }

                '=' => LongLexemes::OperatorOrSingleChar(TokenKind::EqualEqual, TokenKind::Equal),
                '/' => LongLexemes::Slash,
                '"' => LongLexemes::String,
                '0'..='9' => LongLexemes::Number,
                'a'..='z' | 'A'..='Z' | '_' => LongLexemes::Identifier,
                c if c.is_whitespace() => continue,
                c => {
                    return Some(Err(Box::new(scanner_error::SingleTokenError {
                        token: c,
                        source_code: self.source_code.to_string(),
                        source_code_idx: self.source_code.len() - chars_remaining.len(),
                    })));
                }
            };

            match longer_lexemes {
                LongLexemes::OperatorOrSingleChar(operator, single_char) => {
                    if self.lox_remaining.starts_with('=') {
                        let operator_str = &chars_remaining[..2];
                        self.lox_remaining = &self.lox_remaining[1..];

                        return Some(Ok(Token {
                            kind: operator,
                            character: operator_str,
                        }));
                    } else {
                        return build_token(single_char);
                    }
                }
                LongLexemes::Slash => {
                    if self.lox_remaining.starts_with('/') {
                        // Is comment
                        let line_end = self
                            .lox_remaining
                            .find('\n')
                            .unwrap_or(self.lox_remaining.len());
                        self.lox_remaining = &self.lox_remaining[line_end..];

                        continue;
                    } else {
                        return build_token(TokenKind::Slash);
                    }
                }
                LongLexemes::String => {
                    match self.lox_remaining.find('"') {
                        Some(end_quote_idx) => {
                            let n_quotes = 2;
                            let string_literal = &chars_remaining[..end_quote_idx + n_quotes];

                            self.lox_remaining = &chars_remaining[end_quote_idx + n_quotes..];

                            return Some(Ok(Token {
                                kind: TokenKind::String,
                                character: string_literal,
                            }));
                        }
                        None => {
                            self.lox_remaining = &self.lox_remaining[self.lox_remaining.len()..];

                            return Some(Err(Box::new(scanner_error::UnterminatedStringError {
                                source_code: self.source_code.to_string(),
                                source_code_idx: self.source_code.len() - chars_remaining.len(),
                            })));
                        }
                    };
                }
                LongLexemes::Number => {
                    let end_of_number = chars_remaining
                        .find(|c| !matches!(c, '.' | '0'..='9'))
                        .unwrap_or(chars_remaining.len());

                    let mut number_literal = &chars_remaining[..end_of_number];

                    let mut split = number_literal.split('.');

                    if let Some(n) = split.nth(2) {
                        number_literal = &number_literal[..n.len()];
                    }

                    self.lox_remaining = &chars_remaining[end_of_number..];

                    return Some(Ok(Token {
                        kind: TokenKind::Number(number_literal.parse().unwrap()),
                        character: number_literal,
                    }));
                }
                LongLexemes::Identifier => {
                    let end_of_identifier = chars_remaining
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
                        .unwrap_or(chars_remaining.len());

                    let identifier_literal = &chars_remaining[..end_of_identifier];

                    let ident = match identifier_literal {
                        "and" => TokenKind::And,
                        "class" => TokenKind::Class,
                        "else" => TokenKind::Else,
                        "false" => TokenKind::False,
                        "for" => TokenKind::For,
                        "fun" => TokenKind::Fun,
                        "if" => TokenKind::If,
                        "nil" => TokenKind::Nil,
                        "or" => TokenKind::Or,
                        "print" => TokenKind::Print,
                        "return" => TokenKind::Return,
                        "super" => TokenKind::Super,
                        "this" => TokenKind::This,
                        "true" => TokenKind::True,
                        "var" => TokenKind::Var,
                        "while" => TokenKind::While,
                        _ => TokenKind::Identifier,
                    };

                    self.lox_remaining = &chars_remaining[end_of_identifier..];

                    return Some(Ok(Token {
                        kind: ident,
                        character: identifier_literal,
                    }));
                }
            }
        }
    }
}
