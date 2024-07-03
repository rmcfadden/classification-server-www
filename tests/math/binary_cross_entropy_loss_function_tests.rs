#[cfg(test)]
mod tests {
    use classification_server_www::math::loss_function_factory::LossFunctionFactory;

    #[test]
    fn binary_cross_entropy_loss_function_test_error() {
        let normalizer = LossFunctionFactory::create::<f64>("binary_cross_entropy").unwrap();

        let ouputs = vec![0.6652409557748219, 123.0];
        let targets = vec![0.6652409557748219];

        let result = normalizer.apply(&ouputs, &targets);
        assert!(result.is_err());
    }

    #[test]
    fn binary_cross_entropy_loss_function_test_simple() {
        let normalizer = LossFunctionFactory::create::<f64>("binary_cross_entropy").unwrap();

        let ouputs = vec![0.10, 0.40, 0.50];
        let targets = vec![0.80, 0.15, 0.05];

        let loss = normalizer.apply(&ouputs, &targets).unwrap();
        assert_eq!(loss, 0.0);
        //assert_eq!(1 as f64, expecteds.iter().sum::<f64>());
    }

    #[test]
    fn binary_cross_entropy_loss_function_test() {
        let normalizer = LossFunctionFactory::create::<f64>("binary_cross_entropy").unwrap();

        let ouputs = vec![0.09003057317038046, 0.24472847105479767, 0.6652409557748219];
        let targets = vec![0.24472847105479767, 0.09003057317038046, 0.6652409557748219];

        //let loss = normalizer.apply(&ouputs, &targets).unwrap();
        //assert_eq!(normalizeds, expecteds);
        //assert_eq!(1 as f64, expecteds.iter().sum::<f64>());
    }
}
