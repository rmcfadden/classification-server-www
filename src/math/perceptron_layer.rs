use std::ops::{AddAssign, Mul};

use num::{Float, Zero};

use super::{activation_function_factory::ActivationFunctionFactory, matrix::Matrix};

pub struct PerceptronLayer<'a,T> where T: Zero + ToString + Copy + Mul<Output = T> + AddAssign + Float  {
    weights: &'a Matrix<T>,
    biases: &'a Matrix<T>,
    activation: String
} 

impl<'a,T> PerceptronLayer<'a,T> where T: Zero + ToString + Copy + Mul<Output = T> + AddAssign + Float{
    pub fn new (weights: &'a Matrix<T>,biases: &'a Matrix<T>) -> Self {
        Self { 
            weights,
            biases,
            activation: "tanh".to_string()
        }
    }

    pub fn forward(self, inputs: Matrix<T>) -> Matrix::<T>  {
        let w = self.weights[0].len();
        let il = inputs.len();
        let mut items= Vec::<T>::with_capacity(w);
        let activation_func = ActivationFunctionFactory::create::<T>(self.activation.as_str()).unwrap();
        for i in 0..w {
            let mut product = T::zero();
            let weight =  self.weights[0][i];
            let bias =  self.biases[0][i];
            for j in 0..il {
                product += inputs[0][j] * weight;  // T need to implement Mul
            } 
            items[i] =  activation_func.apply((product + bias).into()).into();
        }
        Matrix::<T>::from(vec![items]) 
    }
}