use crate::scanner;

pub fn parse(file_contents: &str) -> Vec<String> {
    let tokens = scanner::scan_file(file_contents);
    let mut ast = Vec::new();

    for token in tokens {
        match token {
            scanner::Token::True | scanner::Token::False | scanner::Token::Nil => {
                let token_str = token.to_string();
                let lower_idx = token_str.find(' ').unwrap();
                let higher_idx = token_str.rfind(' ').unwrap();

                let value = token_str[lower_idx + 1..higher_idx].to_string();
                ast.push(value);
            }
            scanner::Token::Eof => {}
            _ => {
                todo!()
            }
        }
    }

    ast
}
