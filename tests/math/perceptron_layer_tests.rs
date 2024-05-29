#[cfg(test)]
mod tests {
    use classification_server_www::math::{matrix::Matrix, perceptron_layer::PerceptronLayer};

#[test]
 fn test_perceptron_forward(){

    let weights = Matrix::<f32>::from(vec![
            vec![1.0,2.0,31.2]
        ]);

    let biases = Matrix::<f32>::from(vec![
        vec![1.0,2.0,31.2]
    ]);

    let mut layer1 = PerceptronLayer::<f32>::new(&weights, &biases);
 }   
}