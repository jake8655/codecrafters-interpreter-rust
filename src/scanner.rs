use std::{cmp::Ordering, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Eof,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Equal,
    EqualEqual,
    Invalid { char: char, line: usize },
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Eof => write!(f, "EOF  null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Token::Invalid { char, line } => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, char)
            }
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (_, Token::Invalid { .. }) => Ordering::Greater,
            (Token::Invalid { .. }, _) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

impl Token {
    fn tokenize_char(c: char, next_char: Option<char>, line: usize) -> (Token, bool) {
        match c {
            '(' => (Token::LeftParen, false),
            ')' => (Token::RightParen, false),
            '{' => (Token::LeftBrace, false),
            '}' => (Token::RightBrace, false),
            ',' => (Token::Comma, false),
            '.' => (Token::Dot, false),
            '-' => (Token::Minus, false),
            '+' => (Token::Plus, false),
            ';' => (Token::Semicolon, false),
            '*' => (Token::Star, false),
            '=' => {
                if let Some('=') = next_char {
                    (Token::EqualEqual, true)
                } else {
                    (Token::Equal, false)
                }
            }
            _ => (Token::Invalid { char: c, line }, false),
        }
    }

    pub fn scan_file(file_contents: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        for (line_number, line) in file_contents.lines().enumerate() {
            let bytes = line.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                let next_char = bytes.get(i + 1).map(|b| *b as char);

                let (token, skip) =
                    Token::tokenize_char(bytes[i] as char, next_char, line_number + 1);

                tokens.push(token);
                if skip {
                    i += 1;
                }
                i += 1;
            }
        }
        tokens.push(Token::Eof);

        tokens
    }
}
