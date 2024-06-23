mod tests {
    use classification_server_www::core::{
        feature_description::FeatureDescription, input_type::InputType, input_vector::InputVector,
    };

    #[test]
    fn test_input_vector_create() {
        let categories = vec![
            FeatureDescription {
                name: "age".to_string(),
                data_type: "f64".to_string(),
            },
            FeatureDescription {
                name: "height".to_string(),
                data_type: "f64".to_string(),
            },
            FeatureDescription {
                name: "nationality".to_string(),
                data_type: "text".to_string(),
            },
        ];

        let inputs = vec![
            vec![
                "16.5".to_string(),
                "140.101".to_string(),
                "fiji".to_string(),
            ],
            vec!["24.5".to_string(), "165.55".to_string(), "usa".to_string()],
        ];

        let data_types = categories.iter().map(|c| c.data_type.clone()).collect();
        let vector = InputVector::create_categorical::<String>(&data_types, &inputs).unwrap();
        assert_eq!(InputType::Float64(16.5), vector.items[0][0]);
        assert_eq!(InputType::Float64(140.101), vector.items[0][1]);
        assert_eq!(InputType::Text("fiji".to_string()), vector.items[0][2]);
        assert_eq!(InputType::Float64(24.5), vector.items[1][0]);
        assert_eq!(InputType::Float64(165.55), vector.items[1][1]);
        assert_eq!(InputType::Text("usa".to_string()), vector.items[1][2]);
    }

    #[test]
    fn test_input_vector_create_fail_bad_data() {
        let categories = vec![
            FeatureDescription {
                name: "age".to_string(),
                data_type: "f64".to_string(),
            },
            FeatureDescription {
                name: "height".to_string(),
                data_type: "f64".to_string(),
            },
        ];
        let inputs = vec![vec!["TESTS".to_string(), "140.101".to_string()]];
        let data_types = categories.iter().map(|c| c.data_type.clone()).collect();

        let vector_result = match InputVector::create_categorical(&data_types, &inputs) {
            Err(e) => e,
            Ok(_) => "success".into(),
        };
        assert_eq!(
            "Could not parse TESTS into f64".to_string(),
            vector_result.to_string()
        );
    }

    #[test]
    fn test_input_vector_create_fail_bad_data2() {
        let categories = vec![
            FeatureDescription {
                name: "age".to_string(),
                data_type: "i32".to_string(),
            },
            FeatureDescription {
                name: "height".to_string(),
                data_type: "f64".to_string(),
            },
        ];
        let data_types = categories.iter().map(|c| c.data_type.clone()).collect();
        let inputs = vec![vec!["12.1".to_string(), "140.101".to_string()]];
        let vector_result = match InputVector::create_categorical(&data_types, &inputs) {
            Err(e) => e,
            Ok(_) => "success".into(),
        };
        assert_eq!(
            "Could not parse 12.1 into i32".to_string(),
            vector_result.to_string()
        );
    }

    #[test]
    fn test_input_vector_create_fail_categories_length() {
        let categories = vec![FeatureDescription {
            name: "age".to_string(),
            data_type: "f64".to_string(),
        }];
        let data_types = categories.iter().map(|c| c.data_type.clone()).collect();
        let inputs = vec![vec!["TESTS".to_string(), "140.101".to_string()]];
        let vector_result = match InputVector::create_categorical(&data_types, &inputs) {
            Err(e) => e,
            Ok(_) => "success".into(),
        };
        assert_eq!(
            "inputs lengths does not match categories lengths",
            vector_result.to_string()
        );
    }
}
