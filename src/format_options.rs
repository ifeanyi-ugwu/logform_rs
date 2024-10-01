use serde_json::{Map, Value};

#[derive(Clone, Debug, Default)]
pub struct FormatOptions(Map<String, Value>);

impl FormatOptions {
    pub fn new() -> Self {
        Self(Map::new())
    }

    pub fn insert<T: Into<Value>>(mut self, key: &str, value: T) -> Self {
        self.0.insert(key.to_string(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(Value::as_bool)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(Value::as_str).map(String::from)
    }

    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(Value::as_f64)
    }

    pub fn get_object(&self, key: &str) -> Option<&Map<String, Value>> {
        self.get(key).and_then(Value::as_object)
    }

    pub fn get_array(&self, key: &str) -> Option<&Vec<Value>> {
        self.get(key).and_then(Value::as_array)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn merge(&mut self, other: FormatOptions) {
        self.0.extend(other.0);
    }
}

impl From<Map<String, Value>> for FormatOptions {
    fn from(map: Map<String, Value>) -> Self {
        FormatOptions(map)
    }
}

impl From<Value> for FormatOptions {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(map) => FormatOptions(map),
            _ => FormatOptions::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_options() {
        let options = FormatOptions::new()
            .insert("verbose", true)
            .insert("log_level", "info")
            .insert("timeout", 30.0)
            .insert("tags", vec!["important", "urgent"])
            .insert(
                "metadata",
                serde_json::json!({
                    "user": "admin",
                    "process_id": 1234
                }),
            );

        if options.get_bool("verbose").unwrap_or(false) {
            println!("Verbose logging enabled");
        }

        let log_level = options
            .get_string("log_level")
            .unwrap_or_else(|| "warning".to_string());
        let timeout = options.get_number("timeout").unwrap_or(60.0);

        if let Some(tags) = options.get_array("tags") {
            for tag in tags {
                println!("Tag: {}", tag);
            }
        }

        if let Some(metadata) = options.get_object("metadata") {
            if let Some(user) = metadata.get("user").and_then(Value::as_str) {
                println!("User: {}", user);
            }
        }
    }
}
