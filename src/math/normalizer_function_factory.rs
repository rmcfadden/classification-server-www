use std::{
    error::Error,
    ops::{AddAssign, DivAssign},
};

use crate::core::base_error::BaseError;

use super::{
    mean_standard_deviation_normalizer_function::MeanStandardDeviationNormalizerFunction,
    normalizer_function::NormalizerFunction,
    softmax_normalizer_function::SoftmaxNormalizerFunction,
};
use num::Float;

pub struct NormalizerFunctionFactory;
impl NormalizerFunctionFactory {
    pub fn create<T: Float + AddAssign + DivAssign>(
        name: &str,
    ) -> Result<Box<dyn NormalizerFunction<T>>, Box<dyn Error>> {
        match name.to_lowercase().as_str() {
            "default" | "mean_standard_deviation" => {
                Ok(Box::new(MeanStandardDeviationNormalizerFunction))
            }
            "softmax" => Ok(Box::new(SoftmaxNormalizerFunction)),
            _ => Err(Box::new(BaseError {
                message: format!("cannot find normalizer function {name}"),
            })),
        }
    }
}
