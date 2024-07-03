use std::{
    error::Error,
    ops::{AddAssign, DivAssign},
};

use num::Float;

use super::normalizer_function::NormalizerFunction;

pub struct SoftmaxNormalizerFunction;

impl<T: Float + AddAssign + DivAssign> NormalizerFunction<T> for SoftmaxNormalizerFunction {
    fn apply(&self, inputs: &Vec<T>) -> Result<Vec<T>, Box<dyn Error>> {
        let mut sum = T::zero();
        let len = inputs.len();
        for i in 0..len {
            sum += inputs[i].exp();
        }
        Ok(inputs.iter().map(|i| i.exp() / sum).collect())
    }
}
