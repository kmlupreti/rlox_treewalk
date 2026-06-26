use crate::{statement::Stmt, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
