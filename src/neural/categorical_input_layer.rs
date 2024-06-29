use super::input_layer::InputLayer;
use crate::core::feature_description::FeatureDescription;

pub struct CategoricalInputLayer<'a, L: ToString> {
    pub categories: &'a Vec<FeatureDescription<L>>,
}
impl<'a, L: ToString> InputLayer for CategoricalInputLayer<'a, L> {}
