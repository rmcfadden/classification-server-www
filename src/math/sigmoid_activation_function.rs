use num::Float;
use super::activation_function::ActivationFunction;
pub struct SigmoidActivationFunction;
impl<T: Float> ActivationFunction<T> for SigmoidActivationFunction {
    fn apply(&self, input: T) -> T { input }
}