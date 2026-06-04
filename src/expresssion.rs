use std::fmt::Display;

use crate::{
    error::{
        LoxError::{self, EvalError},
        report_error,
    },
    lox_value::LoxValue,
    token::Token,
    token_type::TokenType,
};

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
        }
    }
}

impl Expr {
    pub fn interpret(&self) {
        match self.evaluate() {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(e) => report_error(e),
        }
    }

    pub fn evaluate(&self) -> Result<LoxValue, LoxError> {
        match self {
            Self::Literal { value } => match value.token_type {
                TokenType::String => Ok(LoxValue::String(
                    (value.lexeme[1..value.lexeme.len() - 1]).to_string(),
                )),
                TokenType::Number => Ok(LoxValue::Number(value.lexeme.parse().unwrap())),
                TokenType::False => Ok(LoxValue::Boolean(false)),
                TokenType::True => Ok(LoxValue::Boolean(true)),
                _ => Err(LoxError::EvalError {
                    msg: "Illegal literal value '{value.lexeme}' found",
                }),
            },
            Self::Unary { operator, right } => {
                let right = right.evaluate()?;
                match operator.token_type {
                    TokenType::Minus => {
                        let n = right.parse_num()?;
                        Ok(LoxValue::Number(-n))
                    }
                    TokenType::Bang => match right {
                        LoxValue::Boolean(b) => Ok(LoxValue::Boolean(!b)),
                        _ => Ok(LoxValue::Boolean(true)),
                    },
                    _ => Err(EvalError {
                        msg: "Illegal unary operator '{operator.lexeme' found",
                    }),
                }
            }
            Self::Grouping { expr } => expr.evaluate(),
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                match operator.token_type {
                    TokenType::Minus => {
                        let n1 = left.parse_num()?;
                        let n2 = right.parse_num()?;
                        Ok(LoxValue::Number(n1 - n2))
                    }
                    TokenType::Star => {
                        let n1 = left.parse_num()?;
                        let n2 = right.parse_num()?;
                        Ok(LoxValue::Number(n1 * n2))
                    }
                    TokenType::Slash => {
                        let n1 = left.parse_num()?;
                        let n2 = right.parse_num()?;
                        Ok(LoxValue::Number(n1 / n2))
                    }
                    TokenType::Plus => {
                        if let LoxValue::Number(n1) = left
                            && let LoxValue::Number(n2) = right
                        {
                            Ok(LoxValue::Number(n1 + n2))
                        } else if let LoxValue::String(s1) = left
                            && let LoxValue::String(s2) = right
                        {
                            Ok(LoxValue::String(format!("{}{}", s1, s2)))
                        } else {
                            Err(LoxError::EvalError {
                                msg: "failed to add/concat as operands should be both number or string",
                            })
                        }
                    }
                    _ => Err(LoxError::EvalError {
                        msg: "Illegal binary operator {operator.lexeme}",
                    }),
                }
            }
        }
    }
}
