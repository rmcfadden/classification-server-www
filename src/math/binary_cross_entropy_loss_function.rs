use num::Float;
use std::error::Error;
use std::ops::{AddAssign, MulAssign};

use super::loss_function::LossFunction;
use crate::core::base_error::BaseError;

pub struct BinaryCrossEntropyLossFunction;
impl<T: Float + AddAssign + MulAssign> LossFunction<T> for BinaryCrossEntropyLossFunction {
    fn apply(&self, outputs: &Vec<T>, targets: &Vec<T>) -> Result<T, Box<dyn Error>> {
        if outputs.len() != targets.len() {
            return Err(Box::new(BaseError {
                message: format!("ouput lengths must equal targets length"),
            }));
        }
        let mut lost = T::zero();
        for i in 0..outputs.len() {
            lost += outputs[i] * targets[i].log2();
        }
        Ok(T::from(-1.0).unwrap() * lost)
    }
}
