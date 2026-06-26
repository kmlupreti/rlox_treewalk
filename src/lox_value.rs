use crate::callable::Callable;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Callable(Callable),
    Null,
}
impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Number(n) => write!(f, "{n}"),
            LoxValue::String(s) => write!(f, "\"{s}\""),
            LoxValue::Boolean(b) => write!(f, "{b}"),
            LoxValue::Callable(c) => write!(f, "{:?}", c),
            LoxValue::Null => write!(f, "nil"),
        }
    }
}
impl LoxValue {
    pub fn is_true(&self) -> bool {
        match self {
            Self::Boolean(b) => *b,
            Self::Null => false,
            _ => true,
        }
    }
}
