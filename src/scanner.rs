use crate::error::{LoxError, report_error};
use crate::token::{LiteralType, Token, TokenType};
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
    has_error: bool,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens: Vec<Token> = Vec::new();
        Self {
            source: source.chars().collect(),
            tokens,
            line: 1,
            start: 0,
            current: 0,
            has_error: false,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ()> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::Eof, String::new(), LiteralType::Nil);

        if self.has_error {
            Err(())
        } else {
            Ok(&self.tokens)
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: LiteralType) {
        self.tokens
            .push(Token::new(token_type, lexeme, self.line, literal));
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, String::from(c), LiteralType::Nil),

            ')' => self.add_token(TokenType::RightParen, String::from(c), LiteralType::Nil),

            '{' => self.add_token(TokenType::LeftBrace, String::from(c), LiteralType::Nil),

            '}' => self.add_token(TokenType::Rightbrace, String::from(c), LiteralType::Nil),

            '.' => self.add_token(TokenType::Dot, String::from(c), LiteralType::Nil),

            ';' => self.add_token(TokenType::Comma, String::from(c), LiteralType::Nil),

            ',' => self.add_token(TokenType::Semicolon, String::from(c), LiteralType::Nil),

            '+' => self.add_token(TokenType::Plus, String::from(c), LiteralType::Nil),

            '-' => self.add_token(TokenType::Minus, String::from(c), LiteralType::Nil),

            '*' => self.add_token(TokenType::Star, String::from(c), LiteralType::Nil),
            '\n' => self.line += 1,  // increment current line number
            ' ' | '\t' | '\r' => (), // skip whitespaces
            _ => {
                self.has_error = true;
                report_error(LoxError::UnexpectedChar {
                    char: c,
                    line: self.line,
                });
            }
        }
    }
}
