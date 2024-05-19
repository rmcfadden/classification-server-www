#[cfg(test)]
mod tests {
    use classification_server_www::models::model_factory::ModelFactory;
    #[test]
    fn test_model_factor(){ 
        let model = ModelFactory::create::<String>("default")
            .unwrap();
        assert_eq!("hashmap_model", model.get_name());
    } 
}
