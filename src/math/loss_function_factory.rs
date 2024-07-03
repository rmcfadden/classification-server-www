use std::{
    error::Error,
    ops::{AddAssign, MulAssign},
};

use crate::core::base_error::BaseError;

use super::{
    binary_cross_entropy_loss_function::BinaryCrossEntropyLossFunction, loss_function::LossFunction,
};
use num::Float;

pub struct LossFunctionFactory;
impl LossFunctionFactory {
    pub fn create<T: Float + AddAssign + MulAssign>(
        name: &str,
    ) -> Result<Box<dyn LossFunction<T>>, Box<dyn Error>> {
        match name.to_lowercase().as_str() {
            "default" | "binary_cross_entropy" => Ok(Box::new(BinaryCrossEntropyLossFunction {})),
            _ => Err(Box::new(BaseError {
                message: format!("cannot find normalizer function {name}",),
            })),
        }
    }
}
