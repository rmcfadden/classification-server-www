use num::Float;

use super::{activation_function::ActivationFunction, identity_activation_function::IdentityActivationFunction, sigmoid_activation_function::SigmoidActivationFunction};

pub struct ActivationFunctionFactory;
impl ActivationFunctionFactory {
    pub fn create<T: Float>(name: &str) -> Result<Box<dyn ActivationFunction<T>>, &'static str> {
        match name.to_lowercase().as_str()  {
            "default" | "sigmoid" => Ok(Box::new(SigmoidActivationFunction {})),
            "identity" => Ok(Box::new(IdentityActivationFunction {})),
            _ => Err("cannot find activation function {name}")
        }
    }
}