mod scanner;

use std::path::PathBuf;
use std::{fs, process};

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
/// Lox interpreter
#[command(version, long_about = None)]
struct Cli {
    /// The mode to run the program in
    #[arg(value_enum)]
    command: Command,

    /// The file to parse
    #[arg()]
    file_path: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    /// Print the tokens of the file
    #[clap(name = "tokenize", alias = "t")]
    Tokenize,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Tokenize => {
            let Ok(file_contents) = fs::read_to_string(&args.file_path) else {
                return eprintln!("Failed to read file {}", args.file_path.display());
            };

            let mut tokens = scanner::Token::scan_file(&file_contents);
            tokens.sort();

            display_tokens(&tokens);

            let exit_code = if tokens
                .iter()
                .any(|t| matches!(t, scanner::Token::Invalid { .. }))
            {
                65
            } else {
                0
            };

            process::exit(exit_code);
        }
    }
}

fn display_tokens(tokens: &Vec<scanner::Token>) {
    for token in tokens {
        match *token {
            scanner::Token::Invalid { char, line } => {
                eprintln!("{}", scanner::Token::Invalid { char, line });
            }
            _ => {
                println!("{}", token);
            }
        }
    }
}
