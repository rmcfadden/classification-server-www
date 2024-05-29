use num::Float;

use super::activation_function::ActivationFunction;
pub struct IdentityActivationFunction;
impl<T: Float> ActivationFunction<T> for IdentityActivationFunction {
    fn apply(&self, input: T) -> T { input }
}