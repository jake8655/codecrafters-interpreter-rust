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
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
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
            Token::Bang => write!(f, "BANG ! null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::Less => write!(f, "LESS < null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::Slash => write!(f, "SLASH / null"),
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
    fn tokenize_char(c: char, next_char: Option<char>, line: usize) -> (Option<Token>, bool) {
        match c {
            '(' => (Some(Token::LeftParen), false),
            ')' => (Some(Token::RightParen), false),
            '{' => (Some(Token::LeftBrace), false),
            '}' => (Some(Token::RightBrace), false),
            ',' => (Some(Token::Comma), false),
            '.' => (Some(Token::Dot), false),
            '-' => (Some(Token::Minus), false),
            '+' => (Some(Token::Plus), false),
            ';' => (Some(Token::Semicolon), false),
            '*' => (Some(Token::Star), false),
            '=' => {
                if let Some('=') = next_char {
                    (Some(Token::EqualEqual), true)
                } else {
                    (Some(Token::Equal), false)
                }
            }
            '!' => {
                if let Some('=') = next_char {
                    (Some(Token::BangEqual), true)
                } else {
                    (Some(Token::Bang), false)
                }
            }
            '<' => {
                if let Some('=') = next_char {
                    (Some(Token::LessEqual), true)
                } else {
                    (Some(Token::Less), false)
                }
            }
            '>' => {
                if let Some('=') = next_char {
                    (Some(Token::GreaterEqual), true)
                } else {
                    (Some(Token::Greater), false)
                }
            }
            '/' => {
                if let Some('/') = next_char {
                    (None, true)
                } else {
                    (Some(Token::Slash), false)
                }
            }
            ' ' | '\t' => (None, false),
            _ => (Some(Token::Invalid { char: c, line }), false),
        }
    }

    pub fn scan_file(file_contents: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        'outer: for (line_number, line) in file_contents.lines().enumerate() {
            let bytes = line.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                let next_char = bytes.get(i + 1).map(|b| *b as char);

                let (token, skip) =
                    Token::tokenize_char(bytes[i] as char, next_char, line_number + 1);

                if let Some(token) = token {
                    tokens.push(token);
                } else if skip {
                    continue 'outer;
                }

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
