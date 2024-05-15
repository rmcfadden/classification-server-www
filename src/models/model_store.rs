use std::error::Error;

use async_trait::async_trait;

use crate::models::model::Model;

#[async_trait(?Send)]
pub trait ModelStore<'a, L: ToString + 'a, V: ToString + 'a> {
    async fn add(
        &mut self,
        name: &'a str,
        model: &'a Box<dyn Model<L, V> + 'a>
    ) -> Result<Option<&'a Box<dyn Model<String, V> + 'a>>,  &'static str>;
    async fn get(&self, name: &'a str) -> Result<Option<&&Box<dyn Model<String, V> + 'a>>,  &'static str>;
    async fn setup(&self) -> Result<(), Box<dyn Error>>;
}
