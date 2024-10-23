use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let mut scanner_error = false;

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let tokens = lexical_analyzer::Scanner::new(&file_contents);

            for token in tokens {
                let token = match token {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{e}");
                        scanner_error = true;
                        continue;
                    }
                };
                println!("{token}");
            }
            println!("EOF  null");

            if scanner_error {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

mod lexical_analyzer {
    use std::fmt;

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

    #[derive(Debug)]
    pub struct Token<'a> {
        kind: TokenKind,
        character: &'a str,
    }

    impl fmt::Display for Token<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let character = self.character;
            match self.kind {
                TokenKind::LeftParen => write!(f, "LEFT_PAREN {character} null"),
                TokenKind::RightParen => write!(f, "RIGHT_PAREN {character} null"),
                TokenKind::LeftBrace => write!(f, "LEFT_BRACE {character} null"),
                TokenKind::RightBrace => write!(f, "RIGHT_BRACE {character} null"),
                TokenKind::Star => write!(f, "STAR {character} null"),
                TokenKind::Dot => write!(f, "DOT {character} null"),
                TokenKind::Comma => write!(f, "COMMA {character} null"),
                TokenKind::Plus => write!(f, "PLUS {character} null"),
                TokenKind::Minus => write!(f, "MINUS {character} null"),
                TokenKind::Semicolon => write!(f, "SEMICOLON {character} null"),
                TokenKind::Equal => write!(f, "EQUAL {character} null"),
                TokenKind::EqualEqual => write!(f, "EQUAL_EQUAL {character} null"),
                TokenKind::Bang => write!(f, "BANG {character} null"),
                TokenKind::BangEqual => write!(f, "BANG_EQUAL {character} null"),
                TokenKind::Greater => write!(f, "GREATER {character} null"),
                TokenKind::GreaterEqual => write!(f, "GREATER_EQUAL {character} null"),
                TokenKind::Less => write!(f, "LESS {character} null"),
                TokenKind::LessEqual => write!(f, "LESS_EQUAL {character} null"),
                TokenKind::Slash => write!(f, "SLASH {character} null"),
            }
        }
    }

    #[derive(Debug)]
    enum TokenKind {
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Star,
        Dot,
        Comma,
        Plus,
        Minus,
        Semicolon,
        Equal,
        EqualEqual,
        Bang,
        BangEqual,
        LessEqual,
        Less,
        GreaterEqual,
        Greater,
        Slash,
    }

    #[derive(Debug)]
    pub struct SingleTokenError {
        token: char,

        source_code: String,
        source_code_idx: usize,
    }

    impl SingleTokenError {
        pub fn line(&self) -> usize {
            let line_start = 1;

            let code_span_to_error = &self.source_code[..self.source_code_idx + line_start];
            code_span_to_error.lines().count()
        }
    }

    impl fmt::Display for SingleTokenError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "[line {}] Error: Unexpected character: {}",
                self.line(),
                self.token,
            )
        }
    }

    impl<'a> Iterator for Scanner<'a> {
        type Item = Result<Token<'a>, SingleTokenError>;

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
                    '>' => LongLexemes::OperatorOrSingleChar(
                        TokenKind::GreaterEqual,
                        TokenKind::Greater,
                    ),

                    '=' => {
                        LongLexemes::OperatorOrSingleChar(TokenKind::EqualEqual, TokenKind::Equal)
                    }
                    '/' => LongLexemes::Slash,
                    c if c.is_whitespace() => continue,
                    c => {
                        return Some(Err(SingleTokenError {
                            token: c,
                            source_code: self.source_code.to_string(),
                            source_code_idx: self.source_code.len() - chars_remaining.len(),
                        }))
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
                }
            }
        }
    }
}
