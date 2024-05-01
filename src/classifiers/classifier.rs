use async_trait::async_trait;
use crate::classifiers::classifier_query::ClassifierQuery;
use crate::classifiers::classifier_response::ClassifierResponse;

#[async_trait(?Send)]
pub trait Classifier <'a,L: ToString, V: ToString> {
    async fn classify(&self, query: &ClassifierQuery<'a>) -> Result<ClassifierResponse<L,V>,String>;
}