use std::error::Error;

use async_trait::async_trait;

use crate::models::model::Model;

#[async_trait(?Send)]
pub trait ModelStore<'a, L: ToString + 'a, V: ToString + 'a> {
    async fn add(
        &mut self,
        name: &'a str,
        model: Box<dyn Model<L, V> + 'a>,
    ) -> Result<Option<Box<dyn Model<String, V> + 'a>>, Box<dyn Error>>;
    async fn get(
        &self,
        name: &'a str,
    ) -> Result<Option<Box<dyn Model<String, V> + 'a>>, Box<dyn Error>>;
    async fn setup(&self) -> Result<(), Box<dyn Error>>;
}
