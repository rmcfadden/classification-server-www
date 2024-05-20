use core::fmt;
use std::{fmt::Display, ops::{Add, Index, IndexMut, Mul}};
use num::{Zero};

pub struct Matrix<T=f32> 
    where T: Zero + ToString {
    items: Vec<Vec<T>>
} 

impl<T> Matrix<T> 
    where T: Zero + ToString {
    pub fn new (m: usize, n: usize) -> Self {
        Self { 
            items: (0..m).map(|_x| (0..n).map(|_x| T::zero() ).collect()).collect()     
        }
    }

    pub fn dot(m1: Matrix<T>, m2: Matrix<T>) -> Matrix<T> {
        if m1.items.len() == 0 && m2.items.len() == 0 {
            return Matrix::new(0,0);
        }
        return Matrix::new(0,0);
    }
}

impl<T> Index<usize> for Matrix<T>
where T: Zero + ToString {
    type Output = Vec<T>;
    fn index(&self, index: usize) -> &Vec<T> {
        &self.items[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> 
where T: Zero + ToString {
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.items[index]
    }
}

impl<T> Display for Matrix<T> 
where T: Zero + ToString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n{}\n]", self.items.iter().map(|v| 
                "  [".to_owned() + v.iter().map(|v1| v1.to_string()).collect::<Vec<String>>().join(",").as_str() + "]")
            .collect::<Vec<String>>()
            .join(",\n"))
    }
}

impl<T: Add<Output = T>> Add for Matrix<T> 
where T: Zero + ToString {
    type Output = Self;
    fn add(self, _rhs: Matrix<T>) -> Self::Output {        
        Self {  items: Vec::<Vec<T>>::new() }
    }
}

/* 
impl<T: Mul<Output = T>> Mul for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {  items: vec![vec![0], 0] }
    }
}*/