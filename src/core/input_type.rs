use core::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
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

impl InputType {
    pub fn try_parse(input_type: String, input: String) -> Result<InputType, String> {
        match input_type.as_str() {
            "text" => Ok(InputType::Text(input)),
            "f64" => {
                let parsed = match input.parse::<f64>() {
                    Ok(f) => f,
                    Err(_) => return Err(format!("Could not parse {input} into f64")),
                };
                Ok(InputType::Float64(parsed))
            }
            "i32" => {
                let parsed = match input.parse::<i32>() {
                    Ok(i) => i,
                    Err(_) => return Err(format!("Could not parse {input} into i32")),
                };
                Ok(InputType::Int32(parsed))
            }

            _ => Err("".to_string()),
        }
    }
}
