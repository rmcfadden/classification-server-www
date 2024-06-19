#[cfg(test)]
mod tests {
    use classification_server_www::{
        core::{input_type::InputType, input_vector::InputVector, label::Label},
        models::{hashmap_model::HashmapModel, model::Model},
    };

    #[async_std::test]
    async fn test_hashmap_model() {
        let mut model = HashmapModel::<String>::default();

        let targets = vec![
            Label {
                name: "name1".to_string(),
                value: "".to_string(),
            },
            Label {
                name: "name2".to_string(),
                value: "".to_string(),
            },
        ];

        let _train_result = model
            .train(
                &InputVector {
                    items: vec![
                        vec![InputType::Text("value".to_string())],
                        vec![InputType::Text("asdfasdf".to_string())],
                    ],
                },
                &targets,
            )
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
