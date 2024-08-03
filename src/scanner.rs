use std::{cmp::Ordering, fmt};

#[derive(PartialEq, Eq)]
pub enum Token {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals
    String(String),
    Number(String),
    Identifier(String),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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

    Invalid { err: String, line: usize },
    Eof,
}

impl TryFrom<&str> for Token {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "and" => Ok(Token::And),
            "class" => Ok(Token::Class),
            "else" => Ok(Token::Else),
            "false" => Ok(Token::False),
            "fun" => Ok(Token::Fun),
            "for" => Ok(Token::For),
            "if" => Ok(Token::If),
            "nil" => Ok(Token::Nil),
            "or" => Ok(Token::Or),
            "print" => Ok(Token::Print),
            "return" => Ok(Token::Return),
            "super" => Ok(Token::Super),
            "this" => Ok(Token::This),
            "true" => Ok(Token::True),
            "var" => Ok(Token::Var),
            "while" => Ok(Token::While),
            _ => Err(format!("Unknown keyword: {}", value)),
        }
    }
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
            Token::String(s) => write!(f, "STRING \"{}\" {}", s, s),
            Token::Number(n) => {
                let is_float = n.parse::<f64>().unwrap().fract().abs() > 0.0;
                if is_float {
                    write!(f, "NUMBER {} {}", n, n)
                } else {
                    write!(f, "NUMBER {} {1:.1}", n, n.parse::<f64>().unwrap())
                }
            }
            Token::Identifier(s) => write!(f, "IDENTIFIER {} null", s),
            Token::And => write!(f, "AND and null"),
            Token::Class => write!(f, "CLASS class null"),
            Token::Else => write!(f, "ELSE else null"),
            Token::False => write!(f, "FALSE false null"),
            Token::Fun => write!(f, "FUN fun null"),
            Token::For => write!(f, "FOR for null"),
            Token::If => write!(f, "IF if null"),
            Token::Nil => write!(f, "NIL nil null"),
            Token::Or => write!(f, "OR or null"),
            Token::Print => write!(f, "PRINT print null"),
            Token::Return => write!(f, "RETURN return null"),
            Token::Super => write!(f, "SUPER super null"),
            Token::This => write!(f, "THIS this null"),
            Token::True => write!(f, "TRUE true null"),
            Token::Var => write!(f, "VAR var null"),
            Token::While => write!(f, "WHILE while null"),
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
                tokens.push(Token::Number(number.clone()));
                return (false, true);
            } else if bytes[*i] == b'.' && !is_float {
                is_float = true;
            }

            number.push(bytes[*i] as char);
            *i += 1;
        }

        if number.ends_with('.') {
            number.pop();
            tokens.push(Token::Number(number.clone()));
            *i -= 1;
            return (false, true);
        }

        tokens.push(Token::Number(number.clone()));
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

        if let Ok(keyword) = Token::try_from(identifier.as_str()) {
            tokens.push(keyword);
            return;
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
