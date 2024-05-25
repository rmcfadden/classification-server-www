use core::fmt;
use std::{fmt::Display, ops::{Add, AddAssign, Index, IndexMut, Mul}};
use num::Zero;


// TODO: Only 2d now
pub struct Matrix<T=f32> 
    where T: Zero + ToString + Copy {
    items: Vec<Vec<T>>
} 

impl<T> Matrix<T> 
    where T: Zero + ToString + Copy {
    pub fn new (m: usize, n: usize) -> Self {
        Self { 
            items: (0..m).map(|_x| (0..n).map(|_x| T::zero() ).collect()).collect()     
        }
    }

    pub fn from (items: Vec<Vec<T>>) -> Self {
        Self { 
            items: items  
        }
    }

    pub fn len(&self) -> usize { self.items.len() }

    pub fn dot(m1: Matrix<T>, m2: Matrix<T>) -> T {
        if m1.items.len() == 0 && m2.items.len() == 0 {
            return T::zero();
        }
        return T::zero()
    }
}

impl<T> Index<usize> for Matrix<T>
where T: Zero + ToString + Copy {
    type Output = Vec<T>;
    fn index(&self, index: usize) -> &Vec<T> {
        &self.items[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> 
where T: Zero + ToString + Copy{
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.items[index]
    }
}

impl<T> Display for Matrix<T> 
where T: Zero + ToString + Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n{}\n]", self.items.iter().map(|v| 
                "  [".to_owned() + v.iter().map(|v1| v1.to_string()).collect::<Vec<String>>().join(",").as_str() + "]")
            .collect::<Vec<String>>()
            .join(",\n"))
    }
}

impl<T: Add<Output = T>> Add for Matrix<T> 
where T: Zero + ToString + Copy {
    type Output = Self;
    fn add(self, rhs: Matrix<T>) -> Self::Output {     
        if self.items.len() == 0 && rhs.items.len() == 0 {
            return self;
        }
        if self.items.len() != rhs.items.len() || self.items[0].len() !=  rhs.items.len() {
            panic!("Two matrices must have the same size");
        }        
        let m = self.items.len();
        let n = self.items[0].len();
        let mut new_matrix = Matrix::<T>::new(m,n);
        for i in 0..m {
            for j in 0..n {
                new_matrix[i][j] = self[i][j] + rhs[i][j];
            }
        }
        new_matrix
    }
}

// TODO....
impl<T: Mul<Output = T>> Mul for Matrix<T> 
where T: Zero + ToString + Copy + AddAssign {
    type Output = Self;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {     
        if self.items.len() == 0 && rhs.items.len() == 0 {
            return self;
        }
        if self.items[0].len() != rhs.items.len() {
            panic!("Left side matrix columns must equal right side rows");
        }         
        let m1 = self.items.len();
        let n1 = self.items[0].len();
        let n2 = rhs.items[0].len();
        let mut new_matrix = Matrix::<T>::new(m1,n2);
        for i in 0..m1 {
            for j in 0..n2 {
                let mut sum: T = T::zero();
                for k in 0..n1 {
                    sum += self[i][k] * rhs[k][j]
                }
                new_matrix[i][j] = sum;
            }
        }
        new_matrix
    }
}
