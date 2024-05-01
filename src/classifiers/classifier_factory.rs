use crate::classifiers::classifier::Classifier;
use crate::classifiers::text_classifier::TextClassifier;
use crate::models::model_store::ModelStore;

pub struct ClassifierFactory {}

impl<'a> ClassifierFactory  {
    // TODO: how do we defined data types for classifier in factory
    pub fn create(name: &'a str, store: Box< dyn ModelStore<'a, String,String> + 'a>) -> Result<Box< dyn Classifier<'a, String,String> + 'a>, String> {
        match name  {
            "default" => Ok(Box::new(TextClassifier {store})),
            _ => Err(format!("cannot find classifer {name}"))
        }
    }
}   