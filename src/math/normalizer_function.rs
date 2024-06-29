use std::error::Error;

use num::Float;

pub trait NormalizerFunction<T: Float> {
    fn apply(&self, inputs: &Vec<T>) -> Result<Vec<T>, Box<dyn Error>> {
        Ok(inputs.to_vec())
    }
}
