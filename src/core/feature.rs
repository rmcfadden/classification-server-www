use std::string::ToString;
pub struct Feature <T: ToString> {
    pub value: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_feature_get() {
        let feature = Feature 
        {
            value: 12312.123
        };
        assert_eq!(feature.value, 12312.123);
    }
}