use crate::core::feature_description::FeatureDescription;

use super::input_layer::InputLayer;

pub struct CategoricalInputLayer<'a> {
    pub categories: &'a Vec<FeatureDescription>,
}
impl<'a> InputLayer for CategoricalInputLayer<'a> {}
