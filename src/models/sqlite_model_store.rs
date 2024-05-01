use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use super::{ model::Model, model_store::ModelStore };

pub struct SqliteModelStore<V: ToString> {
    pub test: V,
}

const DB_URL: &str = "sqlite://";

#[async_trait(?Send)]
impl<'a, V: ToString + 'a> ModelStore<'a, String, V> for SqliteModelStore<V> {
    async fn add(
        &mut self,
        name: &'a str,
        model: &'a Box<dyn Model<String, V> + 'a>
    ) -> Option<&'a Box<dyn Model<String, V> + 'a>> {


        let model_text = json::stringify("");
        let db = SqlitePool::connect(DB_URL).await.unwrap();
        let result = sqlx::query("insert into models (name, text) VALUES (?,?)")
            .bind(name)
            .bind(model_text)
            .execute(&db)
            .await
            .unwrap();

        None
    }

    async fn get(&self, name: &'a str) -> Option<&&'a Box<dyn Model<String, V> + 'a>> {
        None
    }

    async fn setup(&self) -> Result<(), &'static str> {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL).await.unwrap();
        } else {
            println!("Database already exists");
        }

        let db = SqlitePool::connect(DB_URL).await.unwrap();

        let sql_text = include_str!("../resources/sql/sqlite/store_migrate_1.sql");
        println!("Applying SQL: {:?}", sql_text);
        let result = sqlx::query(sql_text).execute(&db).await.unwrap();
        println!("Create user table result: {:?}", result);
        Ok(())
    }
}
