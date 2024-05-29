use num::Float;

pub trait NormalizerFunction<T: Float> {
    fn apply(&self, inputs: &Vec<T>) -> Vec<T> { inputs.to_vec()}
}
