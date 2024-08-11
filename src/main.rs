mod parser;
mod scanner;

use std::path::PathBuf;
use std::{fs, process};

use clap::{Parser, ValueEnum};
use scanner::ToChunks;

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
    /// Parse the file and print the AST
    #[clap(name = "parse", alias = "p")]
    Parse,
}

enum ExitCode {
    Success,
    Failure(i32),
}

impl From<i32> for ExitCode {
    fn from(code: i32) -> Self {
        match code {
            0 => ExitCode::Success,
            _ => ExitCode::Failure(code),
        }
    }
}

impl ExitCode {
    fn exit(self) -> ! {
        match self {
            ExitCode::Success => process::exit(0),
            ExitCode::Failure(code) => process::exit(code),
        }
    }
}

fn main() {
    let args = Cli::parse();
    let Ok(file_contents) = fs::read_to_string(&args.file_path) else {
        return eprintln!("Failed to read file {}", args.file_path.display());
    };

    match args.command {
        Command::Tokenize => {
            let mut tokens = scanner::scan_file(&file_contents);
            tokens.sort();

            let exit_code = if tokens
                .iter()
                .any(|t| matches!(t, scanner::Token::Invalid { .. }))
            {
                ExitCode::Failure(65)
            } else {
                ExitCode::Success
            };

            let chunks = tokens.to_chunks();

            display_chunks(&chunks);

            exit_code.exit();
        }
        Command::Parse => {
            let ast = parser::parse(&file_contents);

            for statement in ast {
                println!("{}", statement);
            }
        }
    }
}

fn display_chunks(chunks: &Vec<scanner::Chunk>) {
    for chunk in chunks {
        match &chunk.token_type {
            scanner::Token::Invalid { err, line } => {
                eprintln!("[line {}] Error: {}", line, err);
            }
            _ => {
                println!("{}", chunk);
            }
        }
    }
}
