pub enum LoxError {
    UnexpectedChar { char: char, line: usize },
    ParseError,
}
pub fn report_error(e: LoxError) {
    match e {
        LoxError::UnexpectedChar { char, line } => {
            eprintln!("[line: {}] Unexpected character {:?} found", line, char);
        }
        _ => {
            eprintln!("Unknown error occured")
        }
    }
}
