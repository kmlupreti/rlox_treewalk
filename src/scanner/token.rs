use crate::scanner::TokenType;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}
