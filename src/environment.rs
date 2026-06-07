use crate::lox_value::LoxValue;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: String) -> Option<&LoxValue> {
        self.values.get(&name)
    }
}
