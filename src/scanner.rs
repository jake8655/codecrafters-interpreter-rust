use std::fmt;

pub enum Token {
    Eof,
    LeftParen,
    RightParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Eof => write!(f, "EOF  null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            _ => Token::Eof,
        }
    }
}

impl Token {
    pub fn scan_file(file_contents: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        for c in file_contents.chars() {
            tokens.push(c.into());
        }
        tokens
    }
}
