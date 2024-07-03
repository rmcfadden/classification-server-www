use async_trait::async_trait;
use num::Float;
use std::error::Error;
use std::hash::Hash;
use std::hash::{DefaultHasher, Hasher};
use std::ops::{AddAssign, DivAssign};
use std::time::Instant;

use crate::core::input_type::InputType;
use crate::core::input_vector::InputVector;
use crate::math::normalizer_function_factory::NormalizerFunctionFactory;
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
    V: ToString + Into<V> + AddAssign + Float + Clone + TryFrom<InputType> + AddAssign + DivAssign,
> {
    pub inputs: &'a dyn InputLayer,
    pub layers: &'a Vec<PerceptronLayer<V>>,
    pub outputs: &'a Vec<Label<L, V>>,
}

#[async_trait(?Send)]
impl<
        'a,
        L: ToString + Eq + Hash + Clone + Into<L>,
        V: ToString + Into<V> + AddAssign + Float + Clone + TryFrom<InputType> + AddAssign + DivAssign,
    > Model<L, V> for PerceptronNeuralModel<'a, L, V>
{
    fn get_name(&self) -> String {
        "perceptron_neural_model".to_string()
    }

    async fn train(
        &mut self,
        inputs: &InputVector,
        targets: &Vec<Label<L, V>>,
    ) -> Result<TrainingResult, Box<dyn Error>> {
        if inputs.items.len() == 0 {
            return Err("inputs length cannot be 0".into());
        }
        if targets.len() == 0 {
            return Err("targets length cannot be 0".into());
        }

        let before = Instant::now();

        // preprocessing
        let mut processed_items: Vec<Vec<V>> = vec![];
        for items in inputs.items.iter() {
            let mut processed_inputs: Vec<V> = vec![];
            for item in items.iter() {
                let is_text = match *item {
                    InputType::Text(_) => true,
                    _ => false,
                };
                if is_text {
                    let item_text: String = (*item).clone().into();
                    let mut hasher = DefaultHasher::new();
                    item_text.hash(&mut hasher);
                    let hashed_item = hasher.finish() as f64;
                    processed_inputs.push(try_from_v::<V>(InputType::Float64(hashed_item))?);
                } else {
                    processed_inputs.push(try_from_v::<V>((*item).clone())?);
                }
            }
            processed_items.push(processed_inputs);
        }

        let normalizer = NormalizerFunctionFactory::create::<V>("default")?;
        for i in 0..processed_items[0].len() {
            let mut processed_inputs: Vec<V> = vec![];
            for j in 0..processed_items.len() {
                processed_inputs.push(processed_items[j][i]);
            }
            let normalized_inputs = normalizer.apply(&processed_inputs)?;
            for j in 0..processed_items.len() {
                processed_items[j][i] = normalized_inputs[j];
            }
        }

        // foward propagation
        let output_layer =
            PerceptronLayer::<V>::new_with_random(self.outputs.len(), "sigmoid".to_string())?;

        for items in processed_items {
            let mut result_items = items;
            for layer in self.layers {
                result_items = layer.forward(&result_items)?;
            }

            let output_results = output_layer.forward(&result_items)?;
            // Apply loss function
        }

        Ok(TrainingResult {
            id: "".to_string(),
            elapsed: before.elapsed().as_micros(),
        })
    }

    async fn predict(&self, feature: L) -> Result<ClassifierResponse<L, V>, Box<dyn Error>> {
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
        V: ToString + Into<V> + AddAssign + Float + Clone + TryFrom<InputType> + AddAssign + DivAssign,
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
        V: ToString + Into<V> + AddAssign + Float + Clone + TryFrom<InputType> + AddAssign + DivAssign,
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

fn try_from_v<
    V: ToString + Into<V> + AddAssign + Float + Clone + TryFrom<InputType> + AddAssign + DivAssign,
>(
    item: InputType,
) -> Result<V, Box<dyn Error>> {
    match V::try_from(item) {
        Ok(r) => Ok(r),
        Err(_) => Err("Error generating random float".into()),
    }
}
