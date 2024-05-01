use async_trait::async_trait;

use crate::models::model::Model;

#[async_trait(?Send)]
pub trait ModelStore<'a, L: ToString + 'a, V: ToString + 'a> {
    async fn add(
        &mut self,
        name: &'a str,
        model: &'a Box<dyn Model<L, V> + 'a>
    ) -> Option<&'a Box<dyn Model<L, V> + 'a>>;
    async fn get(&self, name: &'a str) -> Option<&&'a Box<dyn Model<L, V> + 'a>>;
    async fn setup(&self) -> Result<(), &'static str>;
}
