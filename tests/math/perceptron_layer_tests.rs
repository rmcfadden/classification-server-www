#[cfg(test)]
mod tests {
    use classification_server_www::math::perceptron_layer::PerceptronLayer;
    #[test]
    fn test_perceptron_forward() {
        let weights = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let biases = vec![1.0, 0.5, 0.0, -1.0, 0.5];
        let inputs = vec![-1.0, 0.1, 2.0];

        let layer1 = PerceptronLayer::<f32>::new(&weights, &biases);
        let outputs = layer1.forward(inputs);
        println!("{:?}", outputs)
    }
}
