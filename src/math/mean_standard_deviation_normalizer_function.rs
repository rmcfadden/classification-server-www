use std::{
    error::Error,
    ops::{AddAssign, DivAssign},
};

use num::Float;

use super::normalizer_function::NormalizerFunction;

pub struct MeanStandardDeviationNormalizerFunction;

impl<T: Float + AddAssign + DivAssign> NormalizerFunction<T>
    for MeanStandardDeviationNormalizerFunction
{
    fn apply(&self, inputs: &Vec<T>) -> Result<Vec<T>, Box<dyn Error>> {
        let mut sum = T::zero();
        let len = inputs.len();
        for i in 0..len {
            sum += inputs[i];
        }
        let mean = sum / T::from(len as f32).unwrap();
        let mut variance = T::zero();
        for i in 0..len {
            let diff = mean - inputs[i];
            variance += diff * diff;
        }
        variance /= T::from(len as f32).unwrap();
        let standard_deviation = T::sqrt(variance);
        Ok(inputs
            .iter()
            .map(|i| (*i - mean) / standard_deviation)
            .collect())
    }
}
