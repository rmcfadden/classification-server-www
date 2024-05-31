#[cfg(test)]
mod tests {
    use classification_server_www::math::activation_function_factory::ActivationFunctionFactory;
    #[test]
    fn identity_activation_function_tests() {
        let activation = ActivationFunctionFactory::create::<f64>("identity").unwrap();
        let zero_result = activation.apply(0.0);
        let negative_two_result = activation.apply(-2.0);
        let positive_oneish_result = activation.apply(1.123);
        assert_eq!(zero_result, 0.0);
        assert_eq!(negative_two_result, -2.0);
        assert_eq!(positive_oneish_result, 1.123);
    }
}
