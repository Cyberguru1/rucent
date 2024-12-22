use crate::client::decode_publish;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PublishResult {
}

#[cfg(test)]
mod tests {
    use super::*;
}
#[test]
fn test_decode_publish_valid_json() {
    let json_data = r#"{"channel": "test_channel", "offset": 42}"#;
    let result = decode_publish(json_data.as_bytes()).unwrap();
    assert_eq!(result.channel, "test_channel");
    assert_eq!(result.offset, 42);
}
#[test]
fn test_decode_publish_empty_slice() {
    let empty_slice: &[u8] = &[];
    let result = decode_publish(empty_slice);
    assert!(result.is_err());
}

#[test]
fn test_decode_publish_malformed_json() {
    let malformed_json = b"{\"channel\": \"test_channel\", \"offset\": 42";
    let result = decode_publish(malformed_json);
    assert!(result.is_err());
}
