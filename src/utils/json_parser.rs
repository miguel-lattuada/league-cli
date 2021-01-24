use serde_json::{Value};

// Move parser to diff crate

// Using default values to prevent overuse of Option | Result
const DEFAULT_STRING_VALUE: &str = "";
const DEFAULT_INTEGER_VALUE: u64 = 0;

pub struct JsonParser {
    json_data: Value,
}

impl JsonParser {
    pub fn new(json_data: Value) -> Self {
        JsonParser { json_data }
    }

    pub fn safe_read_str(&self, key: &str) -> String {
        let value: Option<&Value> = self.json_data.get(key);

        match value {
            Some(value) => {
                let inner_value: Option<&str> = value.as_str();
                match inner_value {
                    Some(value) => value.to_owned(),
                    None => DEFAULT_STRING_VALUE.to_owned(),
                }
            }
            None => DEFAULT_STRING_VALUE.to_owned(),
        }
    }

    pub fn safe_read_int(&self, key: &str) -> u64 {
        let value: Option<&Value> = self.json_data.get(key);

        match value {
            Some(value) => {
                let inner_value: Option<u64> = value.as_u64();
                match inner_value {
                    Some(value) => value,
                    None => DEFAULT_INTEGER_VALUE,
                }
            }
            None => DEFAULT_INTEGER_VALUE,
        }
    }

    pub fn safe_read_array(&self, key: &str) -> Option<&Vec<Value>> {
        let value: Option<&Value> = self.json_data.get(key);

        match value {
            Some(value) => {
                let inner_value: Option<&Vec<Value>> = value.as_array();
                match inner_value {
                    Some(value) => Some(value),
                    None => None,
                }
            }
            None => None,
        }
    }
}