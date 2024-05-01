#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use classification_server_www::{classifiers::{
        classifier::Classifier, 
        classifier_query::ClassifierQuery, text_classifier::TextClassifier
    }, core::label::Label, models::{hashmap_model::HashmapModel, hashmap_model_store::HashmapModelStore, model::Model, model_store::ModelStore}};

    #[async_std::test]
    async fn test_classifier(){ 

        let mut model: Box<dyn Model<String, String>> = Box::new(HashmapModel::<String, String>{..Default::default()});

        // TOOD: return value for trainging
        let _train_result = model.train(vec![
            Label {name: "name1".to_string(), value:"value".to_string()},
            Label {name: "name2".to_string(), value:"value2".to_string()},
        ] ).await.unwrap();

        let mut model_store: Box< dyn ModelStore< String, String>> = Box::new(HashmapModelStore::<String> { map: HashMap::new()});
        model_store.add("testing", &model).await;
        
        let query = ClassifierQuery{
            text: "name1",
            model: "testing"
        };

        let classifier = TextClassifier {store: model_store};
        let response = classifier.classify(&query).await.unwrap();
        println!("response: {:?}", response); 
    } 
}