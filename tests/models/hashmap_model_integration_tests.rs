#[cfg(test)]
mod tests {
    use classification_server_www::{
        core::{input_type::InputType, input_vector::InputVector, label::Label},
        models::{hashmap_model::HashmapModel, model::Model},
    };

    #[async_std::test]
    async fn test_hashmap_model() {
        let mut model = HashmapModel::<String>::default();

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
        let result = model.predict("name1".to_string()).await.unwrap();
        let predications = result.predictions;
        assert_eq!(1, predications.len());
        assert_eq!("name1", predications[0].label.name);
        assert_eq!("value", predications[0].label.value);
        assert_eq!(100.0, predications[0].percent);
    }
}
