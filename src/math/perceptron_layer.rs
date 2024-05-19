use super::matrix::Matrix;

pub struct PerceptronLayer<T> {
    weights: Matrix<T>,
    biases: Matrix<T>,
    activation: String
} 

impl<T> PerceptronLayer {
    pub fn new (l: i32) -> Self {
        Self { 
            weights: Matrix::<T>::new(),
            biases: Matrix::<T>::new(),
            activation: "tanh".to_string()
        }
    }
}