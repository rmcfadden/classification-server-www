use std::{error::Error, fmt::Display};

use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::{
    core::input_type::InputType, data_access::model::Model as ModelDataAccess,
    models::model_factory::ModelFactory,
};

use super::{model::Model, model_store::ModelStore};

pub struct SqliteModelStore<'a, V> {
    pub url: &'a str,
    pub _marker: std::marker::PhantomData<V>,
}

#[async_trait(?Send)]
impl<'a, V: ToString + Clone + From<String> + Display + From<InputType> + 'a>
    ModelStore<'a, String, V> for SqliteModelStore<'a, V>
{
    async fn add(
        &mut self,
        name: &'a str,
        model: Box<dyn Model<String, V> + 'a>,
    ) -> Result<Option<Box<dyn Model<String, V> + 'a>>, Box<dyn Error>> {
        let model_text = model.serialize();
        let db = SqlitePool::connect(&self.url).await?;
        let _result = sqlx::query("insert into models (name, model_type, text) VALUES (?,?,?)")
            .bind(name)
            .bind(model.get_name())
            .bind(model_text)
            .execute(&db)
            .await?;
        Ok(Some(model))
    }

    async fn get(
        &self,
        name: &'a str,
    ) -> Result<Option<Box<dyn Model<String, V> + 'a>>, Box<dyn Error>> {
        let db = SqlitePool::connect(&self.url).await?;

        let model = sqlx::query_as::<_, ModelDataAccess>(
            "select id,name,model_type,text from models where name = ?",
        )
        .bind(name)
        .fetch_one(&db)
        .await?;

        let mut new_model = ModelFactory::create::<V>(model.model_type.as_str())?;
        new_model.deserialize(model.text);
        Ok(Some(new_model))
    }

    async fn setup(&self) -> Result<(), Box<dyn Error>> {
        if !Sqlite::database_exists(&self.url).await.unwrap_or(false) {
            println!("Creating database {}", &self.url);
            Sqlite::create_database(&self.url).await?;
        } else {
            println!("Database already exists");
        }

        let db = SqlitePool::connect(&self.url).await?;
        let sql_text = include_str!("../resources/sql/sqlite/store_migrate_1.sql");
        println!("applying SQL store_migrate_1: {:?}", sql_text);
        let result = sqlx::query(sql_text).execute(&db).await?;
        println!("Create model table result: {:?}", result);
        Ok(())
    }
}
