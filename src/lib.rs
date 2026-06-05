use std::io::{self, BufRead, BufReader, Write};
use std::{fs::File, path::Path, process::exit};

use crate::parser::Parser;

pub mod error;
pub mod expresssion;
pub mod lox_value;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod token_type;

pub fn run_file<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? > 0 {
        let mut scanner = scanner::Scanner::new(&buffer);
        let tokens = match scanner.scan_tokens() {
            Ok(t) => t,
            Err(_) => exit(65),
        };
        let mut parser = Parser::new(tokens.clone());
        let expr = match parser.parse() {
            Ok(e) => e,
            Err(_) => exit(65),
        };
        match expr.interpret() {
            Ok(_) => (),
            Err(_) => exit(70),
        }
        buffer.clear();
    }
    Ok(())
}
pub fn run_prompt() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        let mut scanner = scanner::Scanner::new(&line);
        let tokens = match scanner.scan_tokens() {
            Ok(t) => t,
            Err(_) => exit(65),
        };
        let mut parser = Parser::new(tokens.clone());
        let expr = match parser.parse() {
            Ok(e) => e,
            Err(_) => exit(65),
        };
        match expr.interpret() {
            Ok(_) => (),
            Err(_) => continue,
        }
    }
    Ok(())
}
