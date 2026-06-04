use crate::error::LoxError;

#[derive(Debug, PartialEq)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
impl LoxValue {
    pub fn parse_num(&self) -> Result<f64, LoxError> {
        match self {
            LoxValue::Number(n) => Ok(*n),
            LoxValue::String(s) => match s.parse::<f64>() {
                Ok(n) => Ok(n),
                Err(_) => Err(LoxError::EvalError {
                    msg: "failed to parse string {s} to number",
                }),
            },
            _ => Err(LoxError::EvalError {
                msg: "failed to parse the value into number",
            }),
        }
    }
}
