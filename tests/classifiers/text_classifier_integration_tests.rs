#[cfg(test)]
mod tests {

    use classification_server_www::{
        classifiers::{
            classifier::Classifier, classifier_query::ClassifierQuery,
            text_classifier::TextClassifier,
        },
        core::{input_type::InputType, input_vector::InputVector, label::Label},
        models::{
            hashmap_model::HashmapModel, hashmap_model_store::HashmapModelStore, model::Model,
            model_store::ModelStore,
        },
    };

    #[async_std::test]
    async fn test_classifier() {
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
                        InputType::Text("value2".to_string()),
                    ],
                ],
            })
            .await
            .unwrap();
        let mut model_store: Box<dyn ModelStore<String, String>> =
            Box::new(HashmapModelStore::<String>::new());
        let _new_model = model_store.add("testing", model).await;

        let query = ClassifierQuery {
            text: "name1",
            model: "testing",
        };

        let classifier = TextClassifier { store: model_store };
        let response = classifier.classify(&query).await.unwrap();
        println!("response: {:?}", response);
    }
}
