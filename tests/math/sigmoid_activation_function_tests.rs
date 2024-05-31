#[cfg(test)]
mod tests {
    use classification_server_www::math::activation_function_factory::ActivationFunctionFactory;
    #[test]
    fn sigmoid_activation_function_test() {
        let activation = ActivationFunctionFactory::create::<f64>("sigmoid").unwrap();
        let zero_result = activation.apply(0.0);
        let negative_two_result = activation.apply(-2.0);
        let negative_fiveish_result = activation.apply(-5.1231);
        let positive_oneish_result = activation.apply(1.123);
        assert_eq!(zero_result, 0.5);
        assert_eq!(negative_two_result, 0.11920292202211755);
        assert_eq!(negative_fiveish_result, 0.005922243987178412);
        assert_eq!(positive_oneish_result, 0.7545447615401272);
    }
}
