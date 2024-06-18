use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::Display;

use crate::classifiers::classifier_response::ClassifierResponse;
use crate::core::input_type::InputType;
use crate::core::input_vector::InputVector;
use crate::core::label::Label;
use crate::core::label_prediction::LabelPrediction;
use crate::core::serialize::Serialize;
use crate::models::model::Model;

use super::training_result::TrainingResult;

pub struct HashmapModel<V: ToString + Into<V> + Display + Clone> {
    map: HashMap<String, V>,
}

#[async_trait(?Send)]
impl<'a, V: ToString + Into<V> + From<String> + From<InputType> + Display + Clone> Model<String, V>
    for HashmapModel<V>
{
    fn get_name(&self) -> String {
        "hashmap_model".to_string()
    }

    async fn train(&mut self, inputs: &InputVector) -> Result<TrainingResult, &'static str> {
        self.map = inputs
            .items
            .iter()
            .map(|v| (v[0].clone().into(), v[1].clone().into()))
            .collect();
        Ok(TrainingResult {
            id: "".to_string(),
            elapsed: 0,
        })
    }

    async fn predict(
        &self,
        feature: String,
    ) -> Result<ClassifierResponse<String, V>, &'static str> {
        let item = self.map.get(&feature);
        let predictions = match item {
            Some(l) => vec![LabelPrediction {
                label: Label {
                    name: feature,
                    value: l.to_owned(),
                },
                percent: 100.0,
            }],
            None => vec![],
        };
        Ok(ClassifierResponse { predictions })
    }
}

impl<V: ToString + Display + Clone> Default for HashmapModel<V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<'a, V: ToString + Display + Clone> HashmapModel<V> {
    pub fn new(map: HashMap<String, V>) -> Self {
        Self { map }
    }
}

impl<V: ToString + Into<V> + From<String> + Display + Clone> Serialize<String> for HashmapModel<V> {
    fn serialize(&self) -> String {
        let map: HashMap<String, String> = self
            .map
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        serde_json::to_string(&map).unwrap()
    }

    fn deserialize(&mut self, input: String) {
        let _map: HashMap<String, String> = serde_json::from_str(&input).unwrap();
        self.map = _map
            .iter()
            .map(|(key, value)| (key.to_string(), V::from(value.to_string())))
            .collect();
    }
}
