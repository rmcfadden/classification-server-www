use async_trait::async_trait;
use std::collections::HashMap;
use std::hash::Hash;

use crate::classifiers::classifier_response::ClassifierResponse;
use crate::core::label_prediction::LabelPrediction;
use crate::core::serialize::Serialize;
use crate::models::model::Model;
use crate::core::label::Label;

pub struct HashmapModel<L: ToString + Into<L>, V: ToString + Into<V>> {
    pub map: HashMap<L, Box<V>>
}

#[async_trait(?Send)]
impl<'a,L: ToString + Eq + Hash + Clone + Into<L> + From<String>, V: ToString + Into<V> + From<String> + Clone> Model<L,V> for HashmapModel<L,V> {
    fn get_name(&self) -> String { "hashmap_model".to_string()}
    
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

impl<L: ToString,V: ToString> Default for HashmapModel<L,V> {
    fn default() -> Self {
        Self { map: Default::default() }
    }
}

impl<L: ToString + Into<L> + From<String> + Eq + Hash,V: ToString + Into<V> + From<String>> Serialize<String> for HashmapModel<L,V> {
    fn serialize(&self) -> String {
        let map: HashMap<String,String> = self.map
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();        
        serde_json::to_string(&map).unwrap()
    }
    
    fn deserialize(&mut self, input: String ) {
        let _map: HashMap<String,String> = serde_json::from_str(&input).unwrap();
        self.map = _map
            .iter()
            .map(|(key, value)| (L::from(key.to_string()), Box::new(V::from(value.to_string()))))
        .collect(); 
    }
}
