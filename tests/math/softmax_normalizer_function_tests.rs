#[cfg(test)]
mod tests {
    use classification_server_www::math::normalizer_function_factory::NormalizerFunctionFactory;
    #[test]
    fn mean_standard_deviation_normalizer_function_test() {
        let normalizer = NormalizerFunctionFactory::create::<f64>("softmax").unwrap();
        let normalizeds = normalizer.apply(&vec![1.0, 2.0, 3.0]).unwrap();
        let expecteds = vec![0.09003057317038046, 0.24472847105479767, 0.6652409557748219];
        assert_eq!(normalizeds, expecteds);
        assert_eq!(1 as f64, expecteds.iter().sum::<f64>());
    }

    #[test]
    fn mean_standard_deviation_normalizer_function_again_test() {
        let normalizer = NormalizerFunctionFactory::create::<f64>("softmax").unwrap();
        let normalizeds = normalizer.apply(&vec![3.776, 6.434, 10.0]).unwrap();
        let expecteds = vec![
            0.001923129450267104,
            0.02743868231659842,
            0.9706381882331344,
        ];
        assert_eq!(normalizeds, expecteds);
        assert_eq!(0.9999999999999999 as f64, expecteds.iter().sum::<f64>());
    }
}
