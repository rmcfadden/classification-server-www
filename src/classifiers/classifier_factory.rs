use crate::classifiers::classifier::Classifier;
use crate::classifiers::text_classifier::TextClassifier;

pub struct ClassifierFactory {}

impl ClassifierFactory {
    // TODO: how do we defined data types for classifier in factory
    pub fn create(name: &str) -> Result<Box<dyn Classifier<String,String>>, &'static str> {
        match name  {
            "default" => Ok(Box::new(TextClassifier{})),
            _ => Err("cannot find classifer {name}")
        }
    }
}