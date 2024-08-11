use std::{cmp::Ordering, fmt};

#[derive(PartialEq, Eq, Clone)]
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
            Token::Eof => write!(f, "EOF"),
            Token::LeftParen => write!(f, "LEFT_PAREN"),
            Token::RightParen => write!(f, "RIGHT_PAREN"),
            Token::LeftBrace => write!(f, "LEFT_BRACE"),
            Token::RightBrace => write!(f, "RIGHT_BRACE"),
            Token::Comma => write!(f, "COMMA"),
            Token::Dot => write!(f, "DOT"),
            Token::Minus => write!(f, "MINUS"),
            Token::Plus => write!(f, "PLUS"),
            Token::Semicolon => write!(f, "SEMICOLON"),
            Token::Star => write!(f, "STAR"),
            Token::Equal => write!(f, "EQUAL"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL"),
            Token::Bang => write!(f, "BANG"),
            Token::BangEqual => write!(f, "BANG_EQUAL"),
            Token::Less => write!(f, "LESS"),
            Token::LessEqual => write!(f, "LESS_EQUAL"),
            Token::Greater => write!(f, "GREATER"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL"),
            Token::Slash => write!(f, "SLASH"),
            Token::String(_) => write!(f, "STRING"),
            Token::Number(_) => write!(f, "NUMBER"),
            Token::Identifier(_) => write!(f, "IDENTIFIER"),
            Token::And => write!(f, "AND"),
            Token::Class => write!(f, "CLASS"),
            Token::Else => write!(f, "ELSE"),
            Token::False => write!(f, "FALSE"),
            Token::Fun => write!(f, "FUN"),
            Token::For => write!(f, "FOR"),
            Token::If => write!(f, "IF"),
            Token::Nil => write!(f, "NIL"),
            Token::Or => write!(f, "OR"),
            Token::Print => write!(f, "PRINT"),
            Token::Return => write!(f, "RETURN"),
            Token::Super => write!(f, "SUPER"),
            Token::This => write!(f, "THIS"),
            Token::True => write!(f, "TRUE"),
            Token::Var => write!(f, "VAR"),
            Token::While => write!(f, "WHILE"),
            Token::Invalid { .. } => write!(f, "INVALID"),
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

pub struct Chunk {
    pub token_type: Token,
    pub lexeme: String,
    pub literal: Option<String>,
}

impl Chunk {
    fn new(token_type: Token, lexeme: String, literal: Option<String>) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.clone().unwrap_or("null".to_string())
        )
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

            let (token, skip) = Token::tokenize_char(bytes[i] as char, next_char, line_number + 1);
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

pub trait ToChunks {
    fn to_chunks(self) -> Vec<Chunk>
    where
        Self: Sized + IntoIterator<Item = Token>,
    {
        let mut chunks = Vec::new();

        for token in self {
            let mut chunk = Chunk::new(token.clone(), "".to_string(), None);

            match token {
                Token::Eof => {}
                Token::LeftParen => {
                    chunk.lexeme = "(".to_string();
                }
                Token::RightParen => {
                    chunk.lexeme = ")".to_string();
                }
                Token::LeftBrace => {
                    chunk.lexeme = "{".to_string();
                }
                Token::RightBrace => {
                    chunk.lexeme = "}".to_string();
                }
                Token::Comma => {
                    chunk.lexeme = ",".to_string();
                }
                Token::Dot => {
                    chunk.lexeme = ".".to_string();
                }
                Token::Minus => {
                    chunk.lexeme = "-".to_string();
                }
                Token::Plus => {
                    chunk.lexeme = "+".to_string();
                }
                Token::Semicolon => {
                    chunk.lexeme = ";".to_string();
                }
                Token::Star => {
                    chunk.lexeme = "*".to_string();
                }
                Token::Equal => {
                    chunk.lexeme = "=".to_string();
                }
                Token::EqualEqual => {
                    chunk.lexeme = "==".to_string();
                }
                Token::Bang => {
                    chunk.lexeme = "!".to_string();
                }
                Token::BangEqual => {
                    chunk.lexeme = "!=".to_string();
                }
                Token::Less => {
                    chunk.lexeme = "<".to_string();
                }
                Token::LessEqual => {
                    chunk.lexeme = "<=".to_string();
                }
                Token::Greater => {
                    chunk.lexeme = ">".to_string();
                }
                Token::GreaterEqual => {
                    chunk.lexeme = ">=".to_string();
                }
                Token::Slash => {
                    chunk.lexeme = "/".to_string();
                }
                Token::String(s) => {
                    chunk.lexeme = format!("\"{}\"", s);
                    chunk.literal = Some(s);
                }
                Token::Number(n) => {
                    let is_float = n.parse::<f64>().unwrap().fract().abs() > 0.0;
                    if is_float {
                        chunk.literal = Some(format!("{}", n));
                    } else {
                        chunk.literal = Some(format!("{:.1}", n.parse::<f64>().unwrap()));
                    }
                    chunk.lexeme = n;
                }
                Token::Identifier(s) => {
                    chunk.lexeme = s;
                }
                Token::And => {
                    chunk.lexeme = "and".to_string();
                }
                Token::Class => {
                    chunk.lexeme = "class".to_string();
                }
                Token::Else => {
                    chunk.lexeme = "else".to_string();
                }
                Token::False => {
                    chunk.lexeme = "false".to_string();
                }
                Token::Fun => {
                    chunk.lexeme = "fun".to_string();
                }
                Token::For => {
                    chunk.lexeme = "for".to_string();
                }
                Token::If => {
                    chunk.lexeme = "if".to_string();
                }
                Token::Nil => {
                    chunk.lexeme = "nil".to_string();
                }
                Token::Or => {
                    chunk.lexeme = "or".to_string();
                }
                Token::Print => {
                    chunk.lexeme = "print".to_string();
                }
                Token::Return => {
                    chunk.lexeme = "return".to_string();
                }
                Token::Super => {
                    chunk.lexeme = "super".to_string();
                }
                Token::This => {
                    chunk.lexeme = "this".to_string();
                }
                Token::True => {
                    chunk.lexeme = "true".to_string();
                }
                Token::Var => {
                    chunk.lexeme = "var".to_string();
                }
                Token::While => {
                    chunk.lexeme = "while".to_string();
                }
                Token::Invalid { .. } => {}
            }
            chunks.push(chunk);
        }

        chunks
    }
}

impl ToChunks for Vec<Token> {}
