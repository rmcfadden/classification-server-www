use std::{collections::HashMap, error::Error};
use async_trait::async_trait;

use super::{model::Model, model_store::ModelStore};

pub struct HashmapModelStore<'a,V: ToString> {
    pub map: HashMap<&'a str, &'a Box<dyn Model<String,V> + 'a>>,
}

#[async_trait(?Send)]
impl<'a, V: ToString> ModelStore<'a,String, V> for HashmapModelStore<'a,V> {
    async fn add(&mut self, name: &'a str, model: &'a Box<dyn Model<String,V> + 'a>) -> Result<Option<&'a Box<dyn Model<String, V> + 'a>>,  &'static str> {
        let new_model = self.map.insert(name, &model);
        Ok(new_model)
    }

    async fn get(&self, name: &'a str) -> Result<Option<&&Box<dyn Model<String, V> + 'a>>,  &'static str> {
        let new_model = self.map.get(&name);
        Ok(new_model)
    }

    async fn setup(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}