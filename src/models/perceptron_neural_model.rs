use async_trait::async_trait;
use num::Float;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::AddAssign;

use crate::core::input_vector::InputVector;
use crate::models::model::Model;
use crate::neural::input_layer::InputLayer;
use crate::neural::perceptron_layer::PerceptronLayer;
use crate::{
    classifiers::classifier_response::ClassifierResponse,
    core::{label::Label, serialize::Serialize},
};

use super::training_result::TrainingResult;

pub struct PerceptronNeuralModel<
    'a,
    L: ToString + Eq + Hash + Clone + Into<L>,
    V: ToString + Into<V> + AddAssign + Float + Clone + From<f64>,
> {
    pub inputs: &'a dyn InputLayer,
    pub layers: &'a Vec<PerceptronLayer<V>>,
    pub outputs: &'a Vec<Label<L, V>>,
}

#[async_trait(?Send)]
impl<
        'a,
        L: ToString + Eq + Hash + Clone + Into<L>,
        V: ToString + Into<V> + AddAssign + Float + Clone + From<f64> + Display,
    > Model<L, V> for PerceptronNeuralModel<'a, L, V>
{
    fn get_name(&self) -> String {
        "perceptron_neural_model".to_string()
    }

    async fn train(
        &mut self,
        inputs: &InputVector,
        targets: &Vec<Label<L, V>>,
    ) -> Result<TrainingResult, &'static str> {
        //self.map = labels.iter()
        //    .map(|l| (l.name.clone(), Box::new(l.value.clone())))
        //    .collect();
        Ok(TrainingResult {
            id: "".to_string(),
            elapsed: 0,
        })
    }

    async fn predict(&self, feature: L) -> Result<ClassifierResponse<L, V>, &'static str> {
        //let item = self.map.get(&feature);
        //let predictions = match item  {
        //   Some(l) => vec![ LabelPrediction { label: Label { name: feature.to_owned(), value: *l.to_owned() }, percent: 100.0 }],
        //   None  => vec![]
        //};
        Ok(ClassifierResponse {
            predictions: vec![],
        })
    }
}

impl<
        'a,
        L: ToString + Eq + Hash + Clone + Into<L>,
        V: ToString + Into<V> + AddAssign + Float + Clone + From<f64>,
    > PerceptronNeuralModel<'a, L, V>
{
    pub fn new(
        inputs: &'a dyn InputLayer,
        layers: &'a Vec<PerceptronLayer<V>>,
        outputs: &'a Vec<Label<L, V>>,
    ) -> Self {
        Self {
            inputs,
            layers,
            outputs,
        }
    }
}

#[async_trait(?Send)]
impl<
        'a,
        L: ToString + Eq + Hash + Clone + Into<L>,
        V: ToString + Into<V> + AddAssign + Float + Clone + From<f64>,
    > Serialize<String> for PerceptronNeuralModel<'a, L, V>
{
    fn serialize(&self) -> String {
        return "".to_string();
        /*
        let map: HashMap<String,String> = self.map
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
        serde_json::to_string(&map).unwrap()
        */
    }

    fn deserialize(&mut self, _input: String) {
        /*
            let _map: HashMap<String,String> = serde_json::from_str(&input).unwrap();
            self.map = _map
                .iter()
                .map(|(key, value)| (L::from(key.to_string()), Box::new(V::from(value.to_string()))))
            .collect();
        */
    }
}
