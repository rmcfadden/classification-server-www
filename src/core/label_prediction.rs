use crate::core::label::Label;

#[derive(Debug, Clone)]
pub struct LabelPrediction<L: ToString, V: ToString> {
    pub label: Label<L, V>,
    pub percent: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_label_prediction_get_label_value() {
        let prediction = LabelPrediction {
            label: Label {
                name: String::from("name1AA"),
                value: String::from("value1BB"),
            },
            percent: 23.11231,
        };
        let label = prediction.label;
        assert_eq!(label.name, String::from("name1AA"));
        assert_eq!(label.value, String::from("value1BB"));
        assert_eq!(prediction.percent, 23.11231);
    }
}
