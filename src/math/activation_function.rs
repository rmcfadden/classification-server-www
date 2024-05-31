use num::Float;

pub trait ActivationFunction<T: Float> {
    fn apply(&self, input: T) -> T {
        T::from(input).unwrap()
    }
}
