use super::input_layer::InputLayer;

pub struct NumericalInputLayer {
    pub height: usize,
    pub data_type: String,
}
impl InputLayer for NumericalInputLayer {}
