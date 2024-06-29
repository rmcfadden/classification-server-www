use std::{
    fmt::Display,
    io::Error,
    ops::{AddAssign, DivAssign},
};

use super::{
    mean_standard_deviation_normalizer_function::MeanStandardDeviationNormalizerFunction,
    normalizer_function::NormalizerFunction,
};
use num::Float;

pub struct NormalizerFunctionFactory;
impl NormalizerFunctionFactory {
    pub fn create<T: Float + AddAssign + DivAssign + Display + From<u32>>(
        name: &str,
    ) -> Result<Box<dyn NormalizerFunction<T>>, Error> {
        match name.to_lowercase().as_str() {
            "default" | "mean_standard_deviation" => {
                Ok(Box::new(MeanStandardDeviationNormalizerFunction))
            }
            _ => Err(Error::other("cannot find normalizer function {name}")),
        }
    }
}
