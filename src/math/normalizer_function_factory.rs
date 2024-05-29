use std::{
    fmt::Display,
    ops::{AddAssign, DivAssign},
};

use super::{
    mean_standard_deviation_normalizer_function::MeanStandardDeviationNormalizerFunction,
    normalizer_function::NormalizerFunction,
};
use num::Float;

pub struct NormalizerFunctionFactory;
impl NormalizerFunctionFactory {
    pub fn create<T: Float + AddAssign + DivAssign + Display>(
        name: &str,
    ) -> Result<Box<dyn NormalizerFunction<T>>, &'static str> {
        match name.to_lowercase().as_str() {
            "default" | "mean_standard_deviation" => {
                Ok(Box::new(MeanStandardDeviationNormalizerFunction))
            }
            _ => Err("cannot find normalizer function {name}"),
        }
    }
}
