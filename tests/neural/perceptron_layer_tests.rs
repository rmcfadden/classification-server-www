#[cfg(test)]
mod tests {
    use classification_server_www::neural::perceptron_layer::PerceptronLayer;
    #[test]
    fn test_perceptron_forward() {
        let weights = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let biases = vec![1.0, 0.5, 0.0, -1.0, 0.5];
        let inputs = vec![-1.0, 0.1, 2.0];
        let layer1 = PerceptronLayer::<f64>::new(weights, biases, "default".to_string());
        let outputs = layer1.forward(inputs);
        assert_eq!(
            outputs,
            [
                0.23147521650098232,
                0.35434369377420455,
                0.5,
                0.52497918747894,
                0.9370266439430035
            ]
        );
    }
}
