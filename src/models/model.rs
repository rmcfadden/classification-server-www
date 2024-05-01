use async_trait::async_trait;
use crate::{classifiers::classifier_response::ClassifierResponse, core::label::Label};

#[async_trait(?Send)]
pub trait Model<L: ToString, V: ToString> {
    fn get_name(&self) -> String;
    async fn train(&mut self, labels: Vec<Label<L,V>>) -> Result<(), &'static str>;
    async fn predict(&self, value: L) -> Result<ClassifierResponse<L,V>, &'static str>;
}