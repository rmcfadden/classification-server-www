use std::ops::AddAssign;

use num::Float;

use super::{
    activation_function::ActivationFunction,
    identity_activation_function::IdentityActivationFunction,
    sigmoid_activation_function::SigmoidActivationFunction,
    tanh_activation_function::TanhActivationFunction,
};

pub struct ActivationFunctionFactory;
impl ActivationFunctionFactory {
    pub fn create<T: Float + AddAssign>(
        name: &str,
    ) -> Result<Box<dyn ActivationFunction<T>>, &'static str> {
        match name.to_lowercase().as_str() {
            "default" | "sigmoid" => Ok(Box::new(SigmoidActivationFunction {})),
            "tanh" => Ok(Box::new(TanhActivationFunction {})),
            "identity" => Ok(Box::new(IdentityActivationFunction {})),
            _ => Err("cannot find activation function {name}"),
        }
    }
}
