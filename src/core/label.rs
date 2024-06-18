use std::string::ToString;

#[derive(Debug, Clone)]
pub struct Label<L: ToString, V: ToString> {
    pub name: L,
    pub value: V,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_label_get_label_value() {
        let label = Label {
            name: String::from("name1"),
            value: String::from("value1"),
        };
        assert_eq!(label.name, String::from("name1"));
        assert_eq!(label.value, String::from("value1"));
    }
}
