use classification_server_www::models::model::Model;
use classification_server_www::{
    core::{feature_description::FeatureDescription, label::Label},
    models::perceptron_neural_model::PerceptronNeuralModel,
    neural::{categorical_input_layer::CategoricalInputLayer, perceptron_layer::PerceptronLayer},
};

#[async_std::test]
async fn test_categorical_perceptron_neural_model() {
    let inputs = CategoricalInputLayer {
        categories: vec![
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
        ],
    };

    let layers = vec![PerceptronLayer::<f64>::new_with_random(
        2,
        "sigmoid".to_string(),
    )];
    let outputs = vec![Label {
        name: String::from("gender"),
        value: 0.0,
    }];

    let mut model = PerceptronNeuralModel::<String, f64>::new(&inputs, &layers, &outputs);
    let name = model.get_name();

    // model.train()

    assert_eq!("perceptron_neural_model", name);
}
