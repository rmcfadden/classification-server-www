use classification_server_www::core::input_vector::InputVector;
use classification_server_www::models::model::Model;
use classification_server_www::neural::numerical_input_layer::NumericalInputLayer;
use classification_server_www::{
    core::{feature_description::FeatureDescription, label::Label},
    models::perceptron_neural_model::PerceptronNeuralModel,
    neural::{categorical_input_layer::CategoricalInputLayer, perceptron_layer::PerceptronLayer},
};

#[async_std::test]
async fn test_mixed_categorical_perceptron_neural_model() {
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

    let inputs = CategoricalInputLayer {
        categories: &categories,
    };

    let layers = vec![PerceptronLayer::<f64>::new_with_random(
        2,
        "sigmoid".to_string(),
    )];
    let outputs = vec![Label {
        name: "gender",
        value: 0.0,
    }];

    let mut model = PerceptronNeuralModel::<&str, f64>::new(&inputs, &layers, &outputs);
    let name = model.get_name();
    let data_types = categories.iter().map(|c| c.data_type.clone()).collect();

    let input_vector = InputVector::create_categorical::<&str>(
        &data_types,
        &vec![
            vec!["44.1", "185.5", "usa"],
            vec!["26.1", "165.5", "fiji"],
            vec!["26.1", "165.5", "australia"],
            vec!["76.1", "122.5", "australia"],
        ],
    )
    .unwrap();

    let targets = vec![
        Label {
            name: "gender",
            value: 1.0,
        },
        Label {
            name: "gender",
            value: 0.0,
        },
        Label {
            name: "gender",
            value: 0.0,
        },
        Label {
            name: "gender",
            value: 1.0,
        },
    ];

    let _train_result = model.train(&input_vector, &targets).await.unwrap();

    assert_eq!("perceptron_neural_model", name);
}

#[async_std::test]
async fn test_mixed_numerical_perceptron_neural_model() {
    let inputs = NumericalInputLayer {
        height: 3,
        data_type: "i8".to_string(),
    };

    let layers = vec![PerceptronLayer::<f64>::new_with_random(
        10,
        "sigmoid".to_string(),
    )];

    let outputs = vec![
        Label {
            name: String::from("cat"),
            value: 0.0,
        },
        Label {
            name: String::from("dog"),
            value: 0.0,
        },
        Label {
            name: String::from("bird"),
            value: 0.0,
        },
    ];

    let mut model = PerceptronNeuralModel::<String, f64>::new(&inputs, &layers, &outputs);

    let input_vector = InputVector::create_numerical::<u8>(
        "u8".to_string(),
        &vec![vec![255, 0, 0], vec![0, 255, 0], vec![0, 0, 255]],
    )
    .unwrap();

    let targets = vec![
        Label {
            name: "gender".to_string(),
            value: 1.0,
        },
        Label {
            name: "gender".to_string(),
            value: 0.0,
        },
        Label {
            name: "gender".to_string(),
            value: 0.0,
        },
    ];

    let _train_result = model.train(&input_vector, &targets).await.unwrap();
}
