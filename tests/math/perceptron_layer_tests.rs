#[cfg(test)]
mod tests {
    use classification_server_www::math::perceptron_layer::PerceptronLayer;
    #[test]
    fn test_perceptron_forward() {
        let weights = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let biases = vec![1.0, 0.5, 0.0, -1.0, 0.5];
        let inputs = vec![-1.0, 0.1, 2.0];
        let layer1 = PerceptronLayer::<f32>::new(&weights, &biases, "default".to_string());
        let outputs = layer1.forward(inputs);
        assert_eq!(
            outputs,
            vec![0.231475219, 0.354343712, 0.5, 0.524979174, 0.937026619]
        );
    }
}
