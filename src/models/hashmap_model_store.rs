use std::{collections::HashMap, error::Error};
use async_trait::async_trait;

use super::{model::Model, model_factory::ModelFactory, model_store::ModelStore};

pub struct HashmapModelStore<'a,V: ToString + 'a> {
    pub map: HashMap<&'a str, Box<dyn Model<String,V>  + 'a>>
}

#[async_trait(?Send)]
impl<'a, V: ToString +  Clone + From<String> + 'a> ModelStore<'a,String, V> for HashmapModelStore<'a,V> {
    async fn add(&mut self, name: &'a str, model: Box<dyn Model<String,V> + 'a>) -> Result<Option<Box<dyn Model<String, V> + 'a>>,  &'static str> {
        let new_model = self.map.insert(name, model);
        Ok(new_model)
    }

    async fn get(&self, name: &'a str) -> Result<Option<Box<dyn Model<String, V> + 'a>>,  &'static str> {
        let new_model = self.map.get(&name).unwrap();
        let text_model = new_model.serialize();
        let mut new_model = ModelFactory::create::<V>(new_model.get_name().as_str()).unwrap();
        new_model.deserialize(text_model);
        Ok(Some(new_model))
    }

    async fn setup(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}