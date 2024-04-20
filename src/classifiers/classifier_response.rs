use crate::core::label_prediction::LabelPrediction;

#[derive(Debug)]
pub struct ClassifierResponse <L: ToString, V: ToString> {
    pub predictions: Vec<LabelPrediction<L,V>>,
}
