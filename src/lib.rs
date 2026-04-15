use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
    path::Path,
    process::exit,
};

use crate::scanner::Scanner;
pub mod scanner;
pub mod token;
pub fn run_file<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut file = File::open(path).expect("error opening source file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("error reading source file");
    run(file_content);
}
pub fn run_prompt() {
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
        run(line);
    }
}
fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("lexeme: {}", token.to_string())
    }
}
fn report_error(line: usize, kind: String, msg: String) {
    eprintln!("[line: {}, Error {} : {}]", line, kind, msg);
    exit(65);
}
