use std::ops::{AddAssign, Mul};

use num::{Float, Zero};

use super::activation_function_factory::ActivationFunctionFactory;

pub struct PerceptronLayer<'a, T>
where
    T: Zero + ToString + Copy + Mul<Output = T> + AddAssign + Float,
{
    weights: &'a Vec<T>,
    biases: &'a Vec<T>,
    activation: String,
}

impl<'a, T> PerceptronLayer<'a, T>
where
    T: Zero + ToString + Copy + Mul<Output = T> + AddAssign + Float,
{
    pub fn new(weights: &'a Vec<T>, biases: &'a Vec<T>, activation: String) -> Self {
        Self {
            weights,
            biases,
            activation,
        }
    }

    pub fn forward(self, inputs: Vec<T>) -> Vec<T> {
        let w = self.weights.len();
        let il = inputs.len();
        let mut items = vec![T::zero(); w];
        let activation_func =
            ActivationFunctionFactory::create::<T>(self.activation.as_str()).unwrap();
        for i in 0..w {
            let mut product = T::zero();
            let weight = self.weights[i];
            let bias = self.biases[i];
            for j in 0..il {
                product += inputs[j] * weight; // T need to implement Mul
            }
            items[i] = activation_func.apply((product + bias).into()).into();
        }
        items
    }
}
