use std::fmt::Display;

use crate::core::input_type::InputType;
use crate::models::hashmap_model::HashmapModel;
use crate::models::model::Model;

pub struct ModelFactory {}

impl ModelFactory {
    pub fn create<'a, V: ToString + Clone + From<String> + From<InputType> + Display + 'a>(
        name: &str,
    ) -> Result<Box<dyn Model<String, V> + 'a>, &'static str> {
        match name.to_lowercase().as_str() {
            "default" | "hashmap_model" => Ok(Box::new(HashmapModel::<V>::default())),
            _ => Err("cannot find model {name}"),
        }
    }
}
