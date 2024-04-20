#[cfg(test)]
mod tests {
    use classification_server_www::{core::label::Label, models::{
        hashmap_model::HashmapModel, model::Model
    }};

    #[async_std::test]
    async fn test_hashmap_model(){ 
        let mut model = HashmapModel::<String, String>{..Default::default()};
        let _trainResult = model.train(vec![
            Label {name: "name1".to_string(), value:"value".to_string()}
        ] ).await;


        let result =  model.predict(&"name1".to_string()).await;


        println!("RESULT: {:?}", result);
        assert_eq!(1,1);
    } 
}