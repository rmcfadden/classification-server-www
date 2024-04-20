use async_trait::async_trait;
use crate::{classifiers::classifier_response::ClassifierResponse, core::label::Label};

#[async_trait(?Send)]
pub trait Model<L: ToString, V: ToString> {
    async fn train(&mut self, labels: Vec<Label<L,V>>) -> Result<(), ()>;
    async fn predict(&self, value: &L) -> Result<ClassifierResponse<L,V>, ()>;
}

