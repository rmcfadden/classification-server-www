#[cfg(test)]
mod tests {
    use classification_server_www::math::activation_function_factory::ActivationFunctionFactory;
    #[test]
    fn tanh_activation_function_test() {
        let activation = ActivationFunctionFactory::create::<f64>("tanh").unwrap();
        let zero_result = activation.apply(0.0);
        let negative_two_result = activation.apply(-2.0);
        let negative_fiveish_result = activation.apply(-5.1231);
        let positive_oneish_result = activation.apply(1.123);
        assert_eq!(zero_result, 0.0);
        assert_eq!(negative_two_result, -0.964027580075817);
        assert_eq!(negative_fiveish_result, -0.9999290182893742);
        assert_eq!(positive_oneish_result, 0.8086098888201517);
    }
}
