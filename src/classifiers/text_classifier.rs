use async_trait::async_trait;
use crate::classifiers::classifier::Classifier;
use crate::classifiers::classifier_query::ClassifierQuery;
use crate::classifiers::classifier_response::ClassifierResponse;
use crate::models::model_store::ModelStore;

pub struct TextClassifier<'a> {
    pub store: Box<dyn ModelStore<'a, String,String> + 'a>
}


#[async_trait(?Send)]
impl<'a> Classifier<'a, String, String> for TextClassifier<'a>  { 
    async fn classify(&self, query: &ClassifierQuery<'a>) -> Result<ClassifierResponse<String, String> , String>   {
        let text = query.text;
        let model_name = query.model;
        let model = match self.store.get(model_name).await {
            Some(m) =>  m,
            None=> return Err(format!("cannot find model name {model_name} in the store")),
        };
        let result = model.predict(text.to_string()).await?;
        Ok(ClassifierResponse { predictions: result.predictions})
    }
}