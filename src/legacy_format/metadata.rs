use serde_json::json;

use crate::LogInfo;

use super::{Format, FormatOptions};
use std::collections::HashMap;

pub fn metadata() -> Format {
    Format::new(|mut info: LogInfo, opts: FormatOptions| {
        // Default metadata key is "metadata"
        let key = opts
            .clone()
            .and_then(|o| o.get("key").cloned())
            .map(|v| v.to_string())
            .unwrap_or_else(|| "metadata".to_string());

        // Collect keys to be excluded from the metadata
        let fill_except: Vec<String> = opts
            .clone()
            .and_then(|o| o.get("fillExcept").cloned())
            .map(|v| v.split(',').map(|x| x.trim().to_string()).collect())
            .unwrap_or_else(|| vec![]);

        // Collect keys to be included in the metadata
        let fill_with: Vec<String> = opts
            .and_then(|o| o.get("fillWith").cloned())
            .map(|v| v.split(',').map(|x| x.trim().to_string()).collect())
            .unwrap_or_else(|| vec![]);

        // Create the metadata object
        let mut metadata = HashMap::new();

        if !fill_with.is_empty() {
            // Add only the keys specified in fillWith
            for key in fill_with {
                if fill_except.contains(&key) {
                    continue; // Skip keys in fillExcept
                }
                if let Some(value) = info.meta.remove(&key) {
                    metadata.insert(key, value);
                }
            }
        } else {
            // Add all keys except those in fillExcept
            for (key, value) in info.meta.clone().into_iter() {
                if !fill_except.contains(&key) {
                    metadata.insert(key.clone(), value);
                    info.meta.remove(&key); // Remove the key from info.meta
                }
            }
        }
        // Insert the metadata in the specified key
        info.meta.insert(key, json!(metadata));

        Some(info)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_metadata_with_fill_with() {
        let metadata_format = metadata();

        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("key1".to_string(), "value1".into());
        info.meta.insert("key2".to_string(), "value2".into());

        let mut opts = HashMap::new();
        opts.insert("key".to_string(), "metadata".to_string());
        opts.insert("fillWith".to_string(), "key1".to_string());

        let result = metadata_format.transform(info, Some(opts)).unwrap();
        let metadata = result.meta.get("metadata").unwrap();

        // Ensure only `key1` was added to metadata
        assert_eq!(
            metadata.get("key1"),
            Some(&Value::String("value1".to_string()))
        );
        assert!(metadata.get("key2").is_none());

        // Ensure `key1` is removed from `info.meta`
        assert!(result.meta.get("key1").is_none());

        // Ensure `key2` remains in `info.meta`
        assert_eq!(
            result.meta.get("key2"),
            Some(&Value::String("value2".to_string()))
        );
    }

    #[test]
    fn test_metadata_with_fill_except() {
        let metadata_format = metadata();

        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("key1".to_string(), "value1".into());
        info.meta.insert("key2".to_string(), "value2".into());
        info.meta.insert("key3".to_string(), "value3".into());

        let mut opts = HashMap::new();
        opts.insert("key".to_string(), "metadata".to_string());
        opts.insert("fillExcept".to_string(), "key1,key3".to_string());

        let result = metadata_format.transform(info, Some(opts)).unwrap();
        let metadata = result.meta.get("metadata").unwrap();

        // Ensure `key2` is in metadata
        assert_eq!(
            metadata.get("key2"),
            Some(&Value::String("value2".to_string()))
        );

        // Ensure `key1` and `key3` are not in metadata
        assert!(metadata.get("key1").is_none());
        assert!(metadata.get("key3").is_none());

        // Ensure `key1` and `key3` remain in `info.meta`
        assert_eq!(
            result.meta.get("key1"),
            Some(&Value::String("value1".to_string()))
        );
        assert_eq!(
            result.meta.get("key3"),
            Some(&Value::String("value3".to_string()))
        );
    }

    #[test]
    fn test_metadata_with_fill_with_and_fill_except() {
        let metadata_format = metadata();

        let mut info = LogInfo::new("info", "Test message");
        info.meta.insert("key1".to_string(), "value1".into());
        info.meta.insert("key2".to_string(), "value2".into());
        info.meta.insert("key3".to_string(), "value3".into());

        let mut opts = HashMap::new();
        opts.insert("key".to_string(), "metadata".to_string());
        opts.insert("fillWith".to_string(), "key1,key2,key3".to_string());
        opts.insert("fillExcept".to_string(), "key2".to_string());

        let result = metadata_format.transform(info, Some(opts)).unwrap();
        let metadata = result.meta.get("metadata").unwrap();

        // Ensure only `key1` and `key3` are in metadata
        assert_eq!(
            metadata.get("key1"),
            Some(&Value::String("value1".to_string()))
        );
        assert_eq!(
            metadata.get("key3"),
            Some(&Value::String("value3".to_string()))
        );
        assert!(metadata.get("key2").is_none());

        // Ensure `key2` remains in `info.meta`
        assert_eq!(
            result.meta.get("key2"),
            Some(&Value::String("value2".to_string()))
        );

        // Ensure `key1` and `key3` are removed from `info.meta`
        assert!(result.meta.get("key1").is_none());
        assert!(result.meta.get("key3").is_none());
    }
}
