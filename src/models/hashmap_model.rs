use async_trait::async_trait;
use std::collections::HashMap;
use std::hash::Hash;

use crate::classifiers::classifier_response::ClassifierResponse;
use crate::models::model::Model;
use crate::core::label::Label;


#[derive(Default)]
pub struct HashmapModel<L: ToString, V: ToString> {
    pub map: HashMap<L, V>,
}

#[async_trait(?Send)]
impl<L: ToString + Eq + Hash + Clone, V: ToString + Clone> Model<L,V> for HashmapModel<L,V> {
    async fn train(&mut self, labels: Vec<Label<L,V>>) -> Result<(), ()> {
        self.map = labels.iter()
            .map(|l| (l.name.clone(), l.value.clone()))
            .collect();

        Ok(())
    }
    async fn predict(&self, value: &L) -> Result<ClassifierResponse<L,V>, ()> {

        let item = self.map.get(value);

        Ok( ClassifierResponse {predictions: vec![] })
    }
}