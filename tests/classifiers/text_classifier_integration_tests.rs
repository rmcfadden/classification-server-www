#[cfg(test)]
mod tests {
    use classification_server_www::classifiers::{
        classifier::Classifier, 
        classifier_query::ClassifierQuery, text_classifier::TextClassifier
    };

    #[test]
    fn test_classifier(){ 
        let query = ClassifierQuery{
            text: String::from("asefd")
        };
        let classifier = TextClassifier {};
        let response = classifier.classify(query);

        println!("RESULT: {:?}", response);
        assert_eq!(1,1);
    } 
}