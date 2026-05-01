use crate::error::{LoxError, report_error};
use crate::token::{LiteralType, Token, TokenType};
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current_index: usize,
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
            current_index: 0,
            has_error: false,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current_index >= self.source.len()
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ()> {
        while !self.is_at_end() {
            self.start = self.current_index;
            self.scan_token();
        }
        self.add_token(TokenType::Eof, Some(String::from("\0")), LiteralType::Nil);

        if self.has_error {
            Err(())
        } else {
            Ok(&self.tokens)
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: Option<String>, literal: LiteralType) {
        let lexeme = match lexeme {
            Some(s) => s,
            None => self.source[self.current_index - 1].to_string(),
        };
        self.tokens
            .push(Token::new(token_type, lexeme, self.line, literal));
    }
    fn advance(&mut self) -> char {
        self.current_index += 1;
        self.source[self.current_index - 1]
    }
    fn match_char(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            if self.current_char() != expected_char {
                false
            } else {
                self.current_index += 1;
                true
            }
        }
    }
    fn current_char(&self) -> char {
        self.source[self.current_index]
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None, LiteralType::Nil),

            ')' => self.add_token(TokenType::RightParen, None, LiteralType::Nil),

            '{' => self.add_token(TokenType::LeftBrace, None, LiteralType::Nil),

            '}' => self.add_token(TokenType::Rightbrace, None, LiteralType::Nil),

            '.' => self.add_token(TokenType::Dot, None, LiteralType::Nil),

            ';' => self.add_token(TokenType::Comma, None, LiteralType::Nil),

            ',' => self.add_token(TokenType::Semicolon, None, LiteralType::Nil),

            '+' => self.add_token(TokenType::Plus, None, LiteralType::Nil),

            '-' => self.add_token(TokenType::Minus, None, LiteralType::Nil),

            '*' => self.add_token(TokenType::Star, None, LiteralType::Nil),
            '\n' => self.line += 1,  // increment current line number
            ' ' | '\t' | '\r' => (), // skip whitespaces
            '/' => {
                if self.match_char('/') {
                    // skip single line comment
                    while self.current_char() != '\n' {
                        self.current_index += 1
                    }
                } else {
                    self.add_token(TokenType::Slash, None, LiteralType::Nil);
                }
            }
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
