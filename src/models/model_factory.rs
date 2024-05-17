use crate::models::model::Model;
use crate::models::hashmap_model::HashmapModel;

pub struct ModelFactory {}

impl ModelFactory {
    pub fn create<'a, V: ToString + Clone + From<String> + 'a>(name: &str) -> Result<Box<dyn Model<String,V> + 'a>, &'static str> {
        match name.to_lowercase().as_str()  {
            "default" | "hashmap_model" => Ok(Box::new(HashmapModel::<String,V>{..Default::default()} )),
            _ => Err("cannot find model {name}")
        }
    }
}