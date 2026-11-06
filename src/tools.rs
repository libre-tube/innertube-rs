use crate::InnertubeError;
use serde_json::Value;

/// Gets data from JSON with specified path
pub fn json(path: &'static str, json: &Value) -> Result<Value, InnertubeError> {
    path.split(".")
        .try_fold(json, |value, key| {
            if key.chars().all(|ch| ch.is_digit(10)) {
                value.get(key.parse::<usize>().unwrap())
            } else {
                value.get(key)
            }
            .ok_or(())
        })
        .map_err(|_| InnertubeError::NotFoundInJSON(vec![path.to_string()]))
        .and_then(|v| Ok(v.to_owned()))
}

/// Connects two `Value`
pub fn append(from: &mut Value, to: &mut Value) {
    to.as_object_mut()
        .unwrap()
        .append(from.as_object_mut().unwrap());
}
