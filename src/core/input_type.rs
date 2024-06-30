use core::fmt;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float64(f64),
    Float32(f32),
    Text(String),
    Bool(bool),
}

impl Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputType::Int8(i) => write!(f, "{}", i),
            InputType::UInt8(i) => write!(f, "{}", i),
            InputType::Int16(i) => write!(f, "{}", i),
            InputType::Int32(i) => write!(f, "{}", i),
            InputType::Int64(i) => write!(f, "{}", i),
            InputType::Float64(i) => write!(f, "{}", i),
            InputType::Float32(i) => write!(f, "{}", i),
            InputType::Text(i) => write!(f, "{}", i),
            InputType::Bool(i) => write!(f, "{}", i),
        }
    }
}

impl From<InputType> for String {
    fn from(value: InputType) -> Self {
        value.to_string()
    }
}

impl TryFrom<InputType> for f64 {
    type Error = String;
    fn try_from(value: InputType) -> Result<Self, Self::Error> {
        let parsed = match value {
            InputType::Int8(i) => i.into(),
            InputType::UInt8(i) => i.into(),
            InputType::Int16(i) => i.into(),
            InputType::Int32(i) => i.into(),
            InputType::Int64(i) => i as f64,
            InputType::Float64(i) => i as f64,
            InputType::Float32(i) => i.into(),
            InputType::Text(i) => match i.parse::<f64>() {
                Ok(f) => f,
                Err(e) => return Err(e.to_string()),
            },
            InputType::Bool(i) => i.into(),
        };
        Ok(parsed)
    }
}

impl TryFrom<InputType> for f32 {
    type Error = String;
    fn try_from(value: InputType) -> Result<Self, Self::Error> {
        let parsed: f32 = match value {
            InputType::Int8(i) => i.into(),
            InputType::UInt8(i) => i.into(),
            InputType::Int16(i) => i.into(),
            InputType::Int32(i) => i as f32,
            InputType::Int64(i) => (i as i64) as f32,
            InputType::Float64(i) => (i as f64) as f32,
            InputType::Float32(i) => i.into(),
            InputType::Text(i) => match i.parse::<f32>() {
                Ok(f) => f,
                Err(e) => return Err(e.to_string()),
            },
            InputType::Bool(i) => i.into(),
        };
        Ok(parsed)
    }
}

impl InputType {
    pub fn try_parse(input_type: &str, input: &str) -> Result<InputType, Box<dyn Error>> {
        match input_type {
            "text" => Ok(InputType::Text(input.to_string())),
            "f64" => {
                let parsed = match input.parse::<f64>() {
                    Ok(f) => f,
                    Err(_) => return Err(format!("Could not parse {input} into f64").into()),
                };
                Ok(InputType::Float64(parsed))
            }
            "f32" => {
                let parsed = match input.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(format!("Could not parse {input} into f32").into()),
                };
                Ok(InputType::Float32(parsed))
            }

            "i8" => {
                let parsed = match input.parse::<i8>() {
                    Ok(i) => i,
                    Err(_) => return Err(format!("Could not parse {input} into i8").into()),
                };
                Ok(InputType::Int8(parsed))
            }
            "u8" => {
                let parsed = match input.parse::<u8>() {
                    Ok(i) => i,
                    Err(_) => return Err(format!("Could not parse {input} into u8").into()),
                };
                Ok(InputType::UInt8(parsed))
            }

            "i16" => {
                let parsed = match input.parse::<i16>() {
                    Ok(i) => i,
                    Err(_) => return Err(format!("Could not parse {input} into i16").into()),
                };
                Ok(InputType::Int16(parsed))
            }
            "i32" => {
                let parsed = match input.parse::<i32>() {
                    Ok(i) => i,
                    Err(_) => return Err(format!("Could not parse {input} into i32").into()),
                };
                Ok(InputType::Int32(parsed))
            }
            _ => Err(format!("{input_type} does not exist").into()),
        }
    }
}
