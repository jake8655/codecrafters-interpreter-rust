use std::{cmp::Ordering, fmt};

#[derive(PartialEq, Eq)]
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
    String(String),
    Number { value: MyFloat },
    Identifier(String),
    Invalid { err: String, line: usize },
}

pub struct MyFloat(f64);

impl PartialEq for MyFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MyFloat {}

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
            Token::String(s) => write!(f, "STRING \"{}\" {}", s, s),
            Token::Number { value } => {
                let is_float = value.0.to_string().contains('.');
                if is_float {
                    write!(f, "NUMBER {} {}", value.0, value.0)
                } else {
                    write!(f, "NUMBER {} {1:.1}", value.0, value.0)
                }
            }
            Token::Identifier(s) => write!(f, "IDENTIFIER {} null", s),
            Token::Invalid { err, line } => {
                write!(f, "[line {}] Error: {}", line, err)
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
            _ => (
                Some(Token::Invalid {
                    err: format!("Unexpected character: {}", c),
                    line,
                }),
                false,
            ),
        }
    }

    fn parse_string_literal(
        i: &mut usize,
        bytes: &[u8],
        tokens: &mut Vec<Token>,
        line_number: usize,
    ) -> (bool, bool) {
        let mut string = String::new();
        *i += 1;
        while *i < bytes.len() {
            if bytes[*i] == b'"' {
                break;
            }

            if *i + 1 == bytes.len() {
                tokens.push(Token::Invalid {
                    err: "Unterminated string.".to_string(),
                    line: line_number + 1,
                });
                *i += 1;
                return (false, true);
            }
            string.push(bytes[*i] as char);
            *i += 1;
        }
        tokens.push(Token::String(string));
        *i += 1;
        (false, true)
    }

    fn parse_number_literal(i: &mut usize, bytes: &[u8], tokens: &mut Vec<Token>) -> (bool, bool) {
        let mut number = String::new();
        number.push(bytes[*i] as char);
        *i += 1;
        let mut is_float = false;

        while *i < bytes.len() && ((bytes[*i] as char).is_numeric() || bytes[*i] == b'.') {
            if bytes[*i] == b'.' && is_float {
                tokens.push(Token::Number {
                    value: MyFloat(number.parse::<f64>().unwrap()),
                });
                return (false, true);
            } else if bytes[*i] == b'.' && !is_float {
                is_float = true;
            }

            number.push(bytes[*i] as char);
            *i += 1;
        }

        if number.ends_with('.') {
            tokens.push(Token::Number {
                value: MyFloat(number.parse::<f64>().unwrap()),
            });
            *i -= 1;
            return (false, true);
        }

        tokens.push(Token::Number {
            value: MyFloat(number.parse::<f64>().unwrap()),
        });
        (false, true)
    }

    fn parse_identifier(i: &mut usize, bytes: &[u8], tokens: &mut Vec<Token>) {
        let mut identifier = String::new();
        identifier.push(bytes[*i] as char);
        *i += 1;
        while *i < bytes.len() && (bytes[*i] == b'_' || (bytes[*i] as char).is_alphanumeric()) {
            identifier.push(bytes[*i] as char);
            *i += 1;
        }
        tokens.push(Token::Identifier(identifier));
    }

    pub fn scan_file(file_contents: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        'line: for (line_number, line) in file_contents.lines().enumerate() {
            let bytes = line.as_bytes();
            let mut i = 0;
            'char: while i < bytes.len() {
                if bytes[i] == b'"' {
                    let (skip_line, skip_char) =
                        Token::parse_string_literal(&mut i, bytes, &mut tokens, line_number);
                    if skip_line {
                        continue 'line;
                    }
                    if skip_char {
                        continue 'char;
                    }
                }

                if (bytes[i] as char).is_numeric() {
                    let (skip_line, skip_char) =
                        Token::parse_number_literal(&mut i, bytes, &mut tokens);
                    if skip_line {
                        continue 'line;
                    }
                    if skip_char {
                        continue 'char;
                    }
                }

                if bytes[i] == b'_' || (bytes[i] as char).is_alphabetic() {
                    Token::parse_identifier(&mut i, bytes, &mut tokens);
                    continue 'char;
                }

                let next_char = bytes.get(i + 1).map(|b| *b as char);

                let (token, skip) =
                    Token::tokenize_char(bytes[i] as char, next_char, line_number + 1);
                if skip {
                    i += 1;
                }

                if let Some(token) = token {
                    tokens.push(token);
                } else if skip {
                    continue 'line;
                }

                i += 1;
            }
        }
        tokens.push(Token::Eof);

        tokens
    }
}
