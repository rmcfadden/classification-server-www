use num::Float;
use std::error::Error;

pub trait LossFunction<T: Float> {
    fn apply(&self, _outputs: &Vec<T>, _expecteds: &Vec<T>) -> Result<T, Box<dyn Error>> {
        Ok(T::zero())
    }
}
