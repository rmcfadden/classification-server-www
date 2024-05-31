use std::ops::AddAssign;

use super::activation_function::ActivationFunction;
use num::Float;
pub struct SigmoidActivationFunction;
impl<T: Float + AddAssign> ActivationFunction<T> for SigmoidActivationFunction {
    fn apply(&self, input: T) -> T {
        T::from(1.0).unwrap() / (T::from(1.0).unwrap() + Float::exp(T::from(-1.0).unwrap() * input))
    }
}
