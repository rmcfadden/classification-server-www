#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use classification_server_www::models::hashmap_model_store::HashmapModelStore;
    use classification_server_www::models::model_store::ModelStore;
    use classification_server_www::models::hashmap_model::HashmapModel;
    use classification_server_www::models::model::Model;
    
    #[async_std::test]
    async fn test_model_store(){ 
        let model: Box<dyn Model<String, String>> = Box::new(HashmapModel::<String, String>{..Default::default()});
        let mut store = HashmapModelStore::<String>{ map: HashMap::new()};
        let name = "test_model";
        store.add(name, &model).await;
        let added_model = store.get(name).await.unwrap();
        assert_eq!(added_model.get_name(), model.get_name());
    }
}