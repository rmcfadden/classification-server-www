use crate::models::model::Model;
use crate::models::hashmap_model::HashmapModel;

pub struct ModelFactory {}

impl ModelFactory {
    pub fn create(name: &str) -> Result<Box<dyn Model<String,String>>, &'static str> {
        match name  {
            "default" => Ok(Box::new(HashmapModel::<String,String>{..Default::default()})),
            _ => Err("cannot find model {name}")
        }
    }
}