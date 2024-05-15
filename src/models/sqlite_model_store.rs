use std::error::Error;

use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::{core::serialize::Serialize, data_access::model::Model as ModelDataAccess};

use super::{ model::Model, model_store::ModelStore };

pub struct SqliteModelStore<V: ToString> {
    pub test: V,
}

const DB_URL: &str = "sqlite://models.sql";

#[async_trait(?Send)]
impl<'a, V: ToString + 'a> ModelStore<'a, String, V> for SqliteModelStore<V> {
    async fn add(
        &mut self,
        name: &'a str,
        model: &'a Box<dyn Model<String, V> + 'a>
    ) -> Result<Option<&'a Box<dyn Model<String, V> + 'a>>,  &'static str> {

        let model_text = model.serialize();
        
        let db = SqlitePool::connect(DB_URL).await.unwrap();
        let result = sqlx::query("insert into models (name, text) VALUES (?,?)")
            .bind(name)
            .bind(model_text)
            .execute(&db)
            .await
            .unwrap();

        Ok(Some(model))
    }

    async fn get(&self, name: &'a str) -> Result<Option<&&Box<dyn Model<String, V> + 'a>>,  &'static str> {
        let db = SqlitePool::connect(DB_URL).await.unwrap();

        let model = sqlx::query_as::<_, ModelDataAccess>( 
            "select id, name, text from models where name = ?"
        )
        .bind(name)
        .fetch_one(&db)
        .await.unwrap();

        print!("{0}", model.name);


        Err("asdf")
    }

    async fn setup(&self) -> Result<(), Box<dyn Error>> {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL).await?;
        } else {            
            println!("Database already exists");
        }

        let db = SqlitePool::connect(DB_URL).await?;
        let sql_text = include_str!("../resources/sql/sqlite/store_migrate_1.sql");
        println!("applying SQL store_migrate_1: {:?}", sql_text);
        let result = sqlx::query(sql_text).execute(&db).await?;
        println!("Create model table result: {:?}", result);
        Ok(())
    }
}
