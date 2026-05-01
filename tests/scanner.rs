use lox::{
    scanner::Scanner,
    token::{LiteralType, Token, TokenType},
};

#[test]
fn should_skip_comments() {
    let source = "// hello this is test
(
// another test
*
";
    let mut scanner = Scanner::new(String::from(source));
    let mut tokens = scanner.scan_tokens().unwrap().into_iter();
    assert_eq!(
        tokens.next().unwrap(),
        &Token::new(TokenType::LeftParen, String::from("("), 2, LiteralType::Nil)
    );
    assert_eq!(
        tokens.next().unwrap(),
        &Token::new(TokenType::Star, String::from("*"), 4, LiteralType::Nil)
    );
    assert_eq!(
        tokens.next().unwrap(),
        &Token::new(TokenType::Eof, String::from("\0"), 5, LiteralType::Nil)
    )
}

#[test]
fn should_recognize_single_char_tokens() {
    let source = "(){}.;,+-*";
    let source_chars = source.chars();
    let mut scanner = Scanner::new(String::from(source));
    let mut tokens = scanner.scan_tokens().unwrap().into_iter();
    for c in source_chars {
        assert_eq!(
            tokens.next().unwrap(),
            &Token::new(
                single_char_to_token_type(c),
                String::from(c),
                1,
                LiteralType::Nil
            )
        );
    }
}

fn single_char_to_token_type(c: char) -> TokenType {
    match c {
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::Rightbrace,
        '.' => TokenType::Dot,
        ';' => TokenType::Comma,
        ',' => TokenType::Semicolon,
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Star,
        '\0' => TokenType::Eof,
        _ => TokenType::Nil,
    }
}
