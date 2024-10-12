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
        file_content: &'a str,
        idx: usize,
    }

    impl<'a> Scanner<'a> {
        pub fn new(file_content: &'a str) -> Self {
            Self {
                file_content,
                idx: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct Token {
        kind: TokenKind,
        character: char,
    }

    impl fmt::Display for Token {
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
    }

    #[derive(Debug)]
    pub struct SingleTokenError {
        character: char,
    }

    impl fmt::Display for SingleTokenError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "[line 1] Error: Unexpected character: {}",
                self.character
            )
        }
    }

    impl<'a> Iterator for Scanner<'a> {
        type Item = Result<Token, SingleTokenError>;

        fn next(&mut self) -> Option<Self::Item> {
            let c = self.file_content.chars().nth(self.idx)?;
            self.idx += 1;

            match c {
                '(' => Some(Ok(Token {
                    kind: TokenKind::LeftParen,
                    character: c,
                })),
                ')' => Some(Ok(Token {
                    kind: TokenKind::RightParen,
                    character: c,
                })),
                '{' => Some(Ok(Token {
                    kind: TokenKind::LeftBrace,
                    character: c,
                })),
                '}' => Some(Ok(Token {
                    kind: TokenKind::RightBrace,
                    character: c,
                })),
                '*' => Some(Ok(Token {
                    kind: TokenKind::Star,
                    character: c,
                })),
                '.' => Some(Ok(Token {
                    kind: TokenKind::Dot,
                    character: c,
                })),
                ',' => Some(Ok(Token {
                    kind: TokenKind::Comma,
                    character: c,
                })),
                '+' => Some(Ok(Token {
                    kind: TokenKind::Plus,
                    character: c,
                })),
                '-' => Some(Ok(Token {
                    kind: TokenKind::Minus,
                    character: c,
                })),
                ';' => Some(Ok(Token {
                    kind: TokenKind::Semicolon,
                    character: c,
                })),
                c => Some(Err(SingleTokenError { character: c })),
            }
        }
    }
}
