use num::Zero;

use super::matrix::Matrix;

pub struct PerceptronLayer<'a,T> where T: Zero + ToString + Copy {
    weights: &'a Matrix<T>,
    biases: &'a Matrix<T>,
    activation: String
} 

impl<'a,T> PerceptronLayer<'a,T> where T: Zero + ToString + Copy {
    pub fn new (weights: &'a Matrix<T>,biases: &'a Matrix<T>) -> Self {
        Self { 
            weights: weights,
            biases: biases,
            activation: "tanh".to_string()
        }
    }

    pub fn forward(self, rhs: Matrix<T>) -> Matrix::<T>  {
        Matrix::<T>::new(1, self.weights.len())
    }

}