use super::input_type::InputType;
pub struct InputVector {
    pub items: Vec<Vec<InputType>>,
}

impl InputVector {
    pub fn create_categorical<S: ToString>(
        data_types: &Vec<String>,
        inputs: &Vec<Vec<S>>,
    ) -> Result<InputVector, String> {
        let mut new_items: Vec<Vec<InputType>> = vec![];
        for input_items in inputs {
            if input_items.len() != data_types.len() {
                return Err("inputs lengths does not match categories lengths".to_string());
            }
            let mut new_inputs: Vec<InputType> = vec![];
            for (i, input) in input_items.iter().enumerate() {
                let input_type = InputType::try_parse(data_types[i].clone(), input.to_string())?;
                new_inputs.push(input_type);
            }
            new_items.push(new_inputs);
        }
        Ok(InputVector { items: new_items })
    }
}
