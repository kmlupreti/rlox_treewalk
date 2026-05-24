pub enum LoxError {
    UnexpectedChar { char: char, line: usize },
    ParseError,
    UnterminatedString { line: usize },
}
pub fn report_error(e: LoxError) {
    match e {
        LoxError::UnexpectedChar { char, line } => {
            eprintln!("[line: {}] Unexpected character {:?} found", line, char);
        }
        LoxError::UnterminatedString { line } => {
            eprintln!("[line: {}] Unterminated string", line);
        }
        _ => {
            eprintln!("Unknown error occured")
        }
    }
}
