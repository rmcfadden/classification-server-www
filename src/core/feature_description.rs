pub struct FeatureDescription<L: ToString> {
    pub name: L,
    pub data_type: L,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_feature_description_get() {
        let feature = FeatureDescription {
            name: "apple".to_string(),
            data_type: "float".to_string(),
        };
        assert_eq!(feature.name, "apple");
        assert_eq!(feature.data_type, "float");
    }
}
