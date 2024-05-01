use async_trait::async_trait;
use std::collections::HashMap;
use std::hash::Hash;

use crate::classifiers::classifier_response::ClassifierResponse;
use crate::core::label_prediction::LabelPrediction;
use crate::models::model::Model;
use crate::core::label::Label;

#[derive(Default)]
pub struct HashmapModel<L: ToString, V: ToString> {
    pub map: HashMap<L, Box<V>>
}

#[async_trait(?Send)]
impl<'a,L: ToString + Eq + Hash + Clone, V: ToString + Clone> Model<L,V> for HashmapModel<L,V> {
    fn get_name(&self) -> String { "HashmapModel".to_string()}
    
    async fn train(&mut self, labels: Vec<Label<L,V>>) -> Result<(), &'static str> {
        self.map = labels.iter()
            .map(|l| (l.name.clone(), Box::new(l.value.clone())))
            .collect();
        Ok(())
    }

    async fn predict(&self, feature: L) -> Result<ClassifierResponse<L,V>, &'static str> {
        let item = self.map.get(&feature);
        let predictions = match item  {
            Some(l) => vec![ LabelPrediction { label: Label { name: feature.to_owned(), value: *l.to_owned() }, percent: 100.0 }],
            None  => vec![]
        };
        Ok( ClassifierResponse { predictions })
    }
}