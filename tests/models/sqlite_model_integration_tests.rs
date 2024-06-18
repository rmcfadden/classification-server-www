#[cfg(test)]
mod tests {
    use classification_server_www::core::input_type::InputType;
    use classification_server_www::core::input_vector::InputVector;
    use classification_server_www::models::hashmap_model::HashmapModel;
    use classification_server_www::models::model::Model;
    use classification_server_www::models::model_store::ModelStore;
    use classification_server_www::models::sqlite_model_store::SqliteModelStore;
    use uuid::Uuid;

    #[async_std::test]
    async fn test_sqlite_store() {
        let mut store = SqliteModelStore::<String> {
            url: "sqlite://models.sql",
            _marker: std::marker::PhantomData,
        };
        store.setup().await.unwrap();

        let mut model: Box<dyn Model<String, String>> = Box::new(HashmapModel::<String>::default());

        let _train_result = model
            .train(&InputVector {
                items: vec![
                    vec![
                        InputType::Text("name1".to_string()),
                        InputType::Text("value".to_string()),
                    ],
                    vec![
                        InputType::Text("name2".to_string()),
                        InputType::Text("asdfasdf".to_string()),
                    ],
                ],
            })
            .await
            .unwrap();

        let name = Uuid::new_v4().to_string();
        let _new_model = store.add(&name, model).await.unwrap();

        let saved_model = store
            .get(&name)
            .await
            .unwrap()
            .expect("saved model should be defined");

        let result = saved_model.predict("name2".to_string()).await.unwrap();
        let predications = result.predictions;
        assert_eq!(1, predications.len());
        assert_eq!("name2", predications[0].label.name);
        assert_eq!("asdfasdf", predications[0].label.value);
        assert_eq!(100.0, predications[0].percent);
    }
}
