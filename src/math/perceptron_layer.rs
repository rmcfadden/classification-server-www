use num::Zero;

use super::matrix::Matrix;

pub struct PerceptronLayer<T> 
where T: Zero + ToString + Copy {
    weights: Matrix<T>,
    biases: Matrix<T>,
    activation: String
} 

impl<T> PerceptronLayer<T> 
where T: Zero + ToString + Copy {
    pub fn new (l: usize) -> Self {
        Self { 
            weights: Matrix::<T>::new(1,l),
            biases: Matrix::<T>::new(1,l),
            activation: "tanh".to_string()
        }
    }
}