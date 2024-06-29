use num::{Float, Zero};
use rand::Rng;
use std::{
    error::Error,
    ops::{AddAssign, Mul},
};

use crate::math::activation_function_factory::ActivationFunctionFactory;

pub struct PerceptronLayer<T>
where
    T: Zero + ToString + Copy + Mul<Output = T> + AddAssign + Float + From<f64>,
{
    weights: Vec<T>,
    biases: Vec<T>,
    activation: String,
}

impl<'a, T> PerceptronLayer<T>
where
    T: Zero + ToString + Copy + Mul<Output = T> + AddAssign + Float + From<f64>,
{
    pub fn new(weights: Vec<T>, biases: Vec<T>, activation: String) -> Self {
        Self {
            weights,
            biases,
            activation,
        }
    }

    pub fn new_with_random(weight_count: usize, activation: String) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weights: (1..weight_count)
                .into_iter()
                .map(|_| rng.gen::<f64>().into())
                .collect(),
            biases: (1..weight_count)
                .into_iter()
                .map(|_| rng.gen::<f64>().into())
                .collect(),
            activation,
        }
    }

    pub fn forward(self, inputs: Vec<T>) -> Result<Vec<T>, Box<dyn Error>> {
        let w = self.weights.len();
        let il = inputs.len();
        let mut items = vec![T::zero(); w];
        let activation_func = ActivationFunctionFactory::create::<T>(self.activation.as_str())?;
        for i in 0..w {
            let mut product = T::zero();
            let weight = self.weights[i];
            let bias = self.biases[i];
            for j in 0..il {
                product += inputs[j] * weight; // T need to implement Mul
            }
            items[i] = activation_func.apply((product + bias).into()).into();
        }
        Ok(items)
    }
}
