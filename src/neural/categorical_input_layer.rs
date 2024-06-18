use crate::core::feature_description::FeatureDescription;

use super::input_layer::InputLayer;

pub struct CategoricalInputLayer {
    pub categories: Vec<FeatureDescription>,
}
impl InputLayer for CategoricalInputLayer {}
