use crate::utils::JsonParser;
use serde_json::{Error, Value};

pub trait FromJson<T> {
    fn from_parser(json: JsonParser) -> T {
        unimplemented!("Method not implemented")
    }
    fn from_json_string(json: &str) -> Result<T, Error> {
        unimplemented!("Method not implemented")
    }
    fn from_json_object(json: Value) -> Result<T, Error> {
        unimplemented!("Method not implemented")
    }
}
