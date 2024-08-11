use crate::scanner::{self, ToChunks};

pub fn parse(file_contents: &str) -> Vec<String> {
    let tokens = scanner::scan_file(file_contents);
    let chunks = tokens.to_chunks();
    let mut ast = Vec::new();

    for chunk in chunks {
        match &chunk.token_type {
            scanner::Token::True | scanner::Token::False | scanner::Token::Nil => {
                ast.push(chunk.lexeme);
            }
            scanner::Token::Number(_) => {
                ast.push(chunk.literal.unwrap());
            }
            scanner::Token::Eof => {}
            _ => {
                todo!()
            }
        }
    }

    ast
}
