use std::fmt::Display;

use crate::{
    classifiers::classifier_response::ClassifierResponse,
    core::{input_vector::InputVector, label::Label, serialize::Serialize},
};
use async_trait::async_trait;

use super::training_result::TrainingResult;

#[async_trait(?Send)]
pub trait Model<L: ToString, V: ToString + Display>: Serialize<String> {
    fn get_name(&self) -> String;
    async fn train(
        &mut self,
        inputs: &InputVector,
        targets: &Vec<Label<L, V>>,
    ) -> Result<TrainingResult, &'static str>;
    async fn predict(&self, value: L) -> Result<ClassifierResponse<L, V>, &'static str>;
}
