use std::ops::AddAssign;

use super::activation_function::ActivationFunction;
use num::Float;
pub struct TanhActivationFunction;
impl<T: Float + AddAssign> ActivationFunction<T> for TanhActivationFunction {
    fn apply(&self, input: T) -> T {
        let e_x = Float::exp(input);
        let e_negative_x = Float::exp(T::from(-1.0).unwrap() * input);
        return (e_x - e_negative_x) / (e_x + e_negative_x);
    }
}
