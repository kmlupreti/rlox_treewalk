use crate::token::{LiteralType, Token, TokenType};
pub struct Scanner {
    tokens: Vec<Token>,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            tokens: vec![Token::new(
                TokenType::String,
                source,
                LiteralType::String,
                11,
            )],
        }
    }
    pub fn scan_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
