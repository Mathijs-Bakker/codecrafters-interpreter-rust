use std::fmt;

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub character: &'a str,
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
            TokenKind::String => {
                write!(f, "STRING {character} {}", character.trim_matches('"'))
            }
            TokenKind::Number(n) => {
                if n == n.trunc() {
                    write!(f, "NUMBER {character} {n}.0")
                } else {
                    write!(f, "NUMBER {character} {n}")
                }
            }
            TokenKind::Identifier => write!(f, "IDENTIFIER {character} null"),
            TokenKind::And => write!(f, "AND {character} null"),
            TokenKind::Class => write!(f, "CLASS {character} null"),
            TokenKind::Else => write!(f, "ELSE {character} null"),
            TokenKind::False => write!(f, "FALSE {character} null"),
            TokenKind::For => write!(f, "FOR {character} null"),
            TokenKind::Fun => write!(f, "FUN {character} null"),
            TokenKind::If => write!(f, "IF {character} null"),
            TokenKind::Nil => write!(f, "NIL {character} null"),
            TokenKind::Or => write!(f, "OR {character} null"),
            TokenKind::Print => write!(f, "PRINT {character} null"),
            TokenKind::Return => write!(f, "RETURN {character} null"),
            TokenKind::Super => write!(f, "SUPER {character} null"),
            TokenKind::This => write!(f, "THIS {character} null"),
            TokenKind::True => write!(f, "TRUE {character} null"),
            TokenKind::Var => write!(f, "VAR {character} null"),
            TokenKind::While => write!(f, "WHILE {character} null"),
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
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
    String,
    Number(f32),
    Identifier,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}
