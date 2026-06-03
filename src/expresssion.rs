use crate::token::Token;

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

impl Expr {
    pub fn print_ast(&self) -> String {
        match &self {
            Self::Literal { value } => value.lexeme.clone(),
            Self::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.print_ast())
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.print_ast(),
                    right.print_ast()
                )
            }
            Self::Grouping { expr } => {
                format!("(group {})", expr.print_ast())
            }
        }
    }
}
