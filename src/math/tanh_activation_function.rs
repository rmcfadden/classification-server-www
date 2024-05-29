use num::Float;
use super::activation_function::ActivationFunction;
pub struct TanhActivationFunction;
impl<T: Float> ActivationFunction<T> for TanhActivationFunction {
    fn apply(&self, input: T) -> T { Float::tanh(input) }
}