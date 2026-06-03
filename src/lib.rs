use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
    path::Path,
};

use crate::parser::Parser;

pub mod error;
pub mod expresssion;
pub mod lox_value;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod token_type;

pub fn run_file<P>(path: P) -> Result<(), ()>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path).expect("error opening source file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("error reading source file");
    run(file_content)
}
pub fn run_prompt() -> Result<(), ()> {
    let mut stdin = io::stdin().lock();
    loop {
        print!("> ");
        io::stdout().flush().expect("error flushing stdout");
        let mut line = String::new();
        stdin
            .read_line(&mut line)
            .expect("error reading line from stdout");
        if line.is_empty() {
            break;
        }
        run(line)?
    }
    Ok(())
}
fn run(source: String) -> Result<(), ()> {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens.clone());
    let expr = parser.parse()?;
    println!("{:?}", expr.evaluate());
    Ok(())
}
