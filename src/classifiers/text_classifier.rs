use crate::classifiers::classifier::Classifier;
use crate::classifiers::classifier_query::ClassifierQuery;
use crate::classifiers::classifier_response::ClassifierResponse;

use crate::models::hashmap_model::HashmapModel;

pub struct TextClassifier {}

impl Classifier<String, String> for TextClassifier  {
    fn classify(&self, query: ClassifierQuery) -> Result<ClassifierResponse<String, String>,()> {

        let text = query.text;

        Ok(ClassifierResponse { predictions: Vec::new()})
        /* 
use std::collections::HashMap;
use http::StatusCode;

        Ok( HttpResponse {
            status: StatusCode::OK,
            body: String::from(""),
            headers: HashMap::new(),
        })*/
    }
}