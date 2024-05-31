#[cfg(test)]
mod tests {
    use classification_server_www::math::normalizer_function_factory::NormalizerFunctionFactory;
    #[test]
    fn mean_standard_deviation_normalizer_function_test() {
        let normalizer = NormalizerFunctionFactory::create::<f64>("default").unwrap();
        let normalized_inputs =
            normalizer.apply(&vec![-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5, 2.0]);
        assert_eq!(
            normalized_inputs,
            vec![
                -1.5491933384829668,
                -1.161895003862225,
                -0.7745966692414834,
                -0.3872983346207417,
                0.0,
                0.3872983346207417,
                0.7745966692414834,
                1.161895003862225,
                1.5491933384829668
            ]
        );
    }

    #[test]
    fn mean_standard_deviation_normalizer_function_again_test() {
        let normalizer = NormalizerFunctionFactory::create::<f64>("default").unwrap();
        let normalized_inputs = normalizer.apply(&vec![100.0, 200.0, 300.0, 500.0]);
        assert_eq!(
            normalized_inputs,
            vec![
                -1.1832159566199232,
                -0.50709255283711,
                0.16903085094570333,
                1.52127765851133
            ]
        );
    }
}
