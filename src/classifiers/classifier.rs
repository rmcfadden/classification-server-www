use std::error::Error;
use std::fmt::Display;

use crate::classifiers::classifier_query::ClassifierQuery;
use crate::classifiers::classifier_response::ClassifierResponse;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Classifier<'a, L: ToString, V: ToString + Display> {
    async fn classify(
        &self,
        query: &ClassifierQuery<'a>,
    ) -> Result<ClassifierResponse<L, V>, Box<dyn Error>>;
}
