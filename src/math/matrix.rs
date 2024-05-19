use std::ops::{Add, Mul};

pub struct Matrix<T> {
    items: Vec<Vec<T>>
} 

impl<T> Matrix 
    where
        T: Mul<f64, Output = T> + Add<T, Output = T> + Add<f64, Output = T> + Copy,
        f64: Mul<f64, Output = T> + Mul<f64, Output = f64> {
    pub fn new (m: i32, n: i32) -> Self {
        Self { items: vec![vec![m], n] }
    }

    pub fn dot(&m1: Matrix<T>, &m2: Matrix<T>) -> Matrix<T> {
        if m1.items.count() == 0 && m2.items.count() == 0 {
            return Matrix::new(0,0);
        }
        return Matrix::new(0,0);
    }
}

impl<T> Add<T> for Matrix<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {  items: vec![vec![0], 0] }
    }
}

impl<T> Mul<T> for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {  items: vec![vec![0], 0] }
    }
}