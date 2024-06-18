use core::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum InputType {
    Int32(i32),
    Int64(i32),
    Float64(f64),
    Float32(f32),
    Text(String),
    Bool(bool),
}

impl Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputType::Text(c) => write!(f, "{}", c),
            _ => unreachable!(),
        }
    }
}

impl From<InputType> for String {
    fn from(value: InputType) -> Self {
        value.to_string()
    }
}
