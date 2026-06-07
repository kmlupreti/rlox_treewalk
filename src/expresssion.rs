use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub enum Expr {
    Literal {
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Literal { value } => {
                let value = value.lexeme.clone();
                write!(f, "{value}")
            }
            Self::Unary { operator, right } => {
                let operator = operator.lexeme.clone();
                write!(f, "({operator} {right})")
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let operator = operator.lexeme.clone();
                write!(f, "({operator} {left} {right})")
            }
            Self::Grouping { expr } => {
                write!(f, "(group {expr})")
            }
            Self::Variable { name } => write!(f, "{name}"),
        }
    }
}
