#[cfg(test)]
mod tests {
    use classification_server_www::models::hashmap_model::HashmapModel;
    use classification_server_www::models::model::Model;
    use classification_server_www::models::sqlite_model_store::SqliteModelStore;
    use classification_server_www::models::model_store::ModelStore;
    use uuid::Uuid;

    #[async_std::test]
    async fn test_sqlite_store() {
        let mut store = SqliteModelStore::<String> { test: "asdf".to_string() };
        store.setup().await.unwrap();

        let model: Box<dyn Model<String,String>>= Box::new(HashmapModel::<String, String>{..Default::default()} );
        let name = Uuid::new_v4().to_string();
        let _new_model = store.add(&name, &model).await.unwrap();

        store.get(&name).await.unwrap();


    }
}
