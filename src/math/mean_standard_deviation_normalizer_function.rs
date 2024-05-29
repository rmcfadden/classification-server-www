use std::{
    fmt::Display,
    ops::{AddAssign, DivAssign},
};

use num::Float;

use super::normalizer_function::NormalizerFunction;

pub struct MeanStandardDeviationNormalizerFunction;

impl<T: Float + AddAssign + DivAssign + Display> NormalizerFunction<T>
    for MeanStandardDeviationNormalizerFunction
{
    fn apply(&self, inputs: &Vec<T>) -> Vec<T> {
        let mut sum = T::zero();
        let len = inputs.len();
        for i in 0..len {
            sum += inputs[i];
        }
        let mean = sum / T::from(len).unwrap();
        let mut variance = T::zero();
        for i in 0..len {
            let diff = mean - inputs[i];
            variance += diff * diff;
        }
        variance /= T::from(len).unwrap();

        let standard_deviation = T::sqrt(variance);

        println!("sum: {sum}");
        println!("variance: {variance}");
        println!("mean: {mean}");
        println!("standard_deviation: {standard_deviation}");

        inputs
            .iter()
            .map(|i| (*i - mean) / standard_deviation)
            .collect()
    }
}
