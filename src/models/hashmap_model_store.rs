use std::collections::HashMap;
use async_trait::async_trait;

use super::{model::Model, model_store::ModelStore};

pub struct HashmapModelStore<'a,V: ToString> {
    pub map: HashMap<&'a str, &'a Box<dyn Model<String,V> + 'a>>,
}

#[async_trait(?Send)]
impl<'a, V: ToString> ModelStore<'a,String, V> for HashmapModelStore<'a,V> {
    async fn add(&mut self, name: &'a str, model: &'a Box<dyn Model<String,V> + 'a>) -> Option<&'a Box<dyn Model<String,V> + 'a>> {
        self.map.insert(name, model)
    }

    async fn get(&self, name: &'a str) -> Option<&&'a Box<dyn Model<String, V> + 'a>> {
        self.map.get(&name)
    }

    async fn setup(&self) -> Result<(),  &'static str> {
        Ok(())
    }
}