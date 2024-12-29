#![allow(unused_imports)]
use lazy_static::lazy_static;
use rucent::client::{decode_publish, Client, Config};
use rucent::options::{with_disconnect, with_skip_history, Disconnect};
use std::env;
use std::rc::Rc;
use tokio::runtime::Runtime;

#[cfg(test)]
mod tests {

    use super::*;

    lazy_static! {
        static ref ADDR: String =
            env::var("API_URL").unwrap_or("http://127.0.0.1:8000/api".to_string());
        static ref API_KEY: String =
            env::var("API_KEY").unwrap_or("default_api_key_hex".to_string());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_with_api_key() {
        match env::var("API_KEY") {
            Ok(_) => println!("API_URL environment variable set"),
            Err(_) => panic!("API_URL environment variable not set"),
        }

        match env::var("API_URL") {
            Ok(_) => println!("API_URL environment variable set"),
            Err(_) => panic!("API_URL environment variable not set"),
        }
    }

    #[test]
    fn test_decode_publish_valid_json() {
        let json_data = r#"
        {
            "epoch": "1789378957", 
            "offset": 42
        }"#;

        let result = decode_publish(json_data.as_bytes()).unwrap();
        assert_eq!(result.offset, Some(42));
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

    #[test]
    fn test_client_new() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };
        let client = Client::new(config);
        assert_eq!(client.endpoint, Some(ADDR.to_string()));
        assert_eq!(client.api_key, Some(API_KEY.to_string()));
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_publish() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let rt = Runtime::new().unwrap();

        let client = Client::new(config);
        let result = rt.block_on(client.publish(
            "test_channel".to_string(),
            data,
            &[with_skip_history(true)],
        ));
        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_broadcast() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let rt = Runtime::new().unwrap();

        let client = Client::new(config);
        let result = rt.block_on(client.broadcast(
            vec!["test_channel".to_string(), "test_channel2".to_string()],
            data,
            &[],
        ));

        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_subscribe() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result =
            rt.block_on(client.subscribe("test_channel".to_string(), "test_user".to_string(), &[]));

        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_unsubscribe() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.unsubscribe(
            "test_channel".to_string(),
            "test_user".to_string(),
            &[],
        ));

        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_disconnect() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        println!("{:?}", config.addr);

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.disconnect(
            "test_user".to_string(),
            &[with_disconnect(Disconnect::default())],
        ));

        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_presence() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.presence("test_channel".to_string()));
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_presence_stats() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.presence_stats("test_channel".to_string()));
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_history() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.history("test_channel".to_string(), &[]));
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_history_remove() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.history_remove("test_channel".to_string()));
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_channels() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.channels(&[]));
        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_info() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);
        let result = rt.block_on(client.info());
        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_multiple_commands() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);

        let result = rt.block_on(client.info());
        assert!(!result.is_err());

        let result = rt.block_on(client.info());
        assert!(!result.is_err());

        let result = rt.block_on(client.channels(&[]));
        assert!(!result.is_err());

        let result = rt.block_on(client.disconnect(
            "test_user".to_string(),
            &[with_disconnect(Disconnect::default())],
        ));
        assert!(!result.is_err());
    }

    #[test]
    #[cfg(feature = "with_local_server")]
    fn test_client_multiple_request_in_pipe() {
        let config = Config {
            addr: Some(ADDR.to_string()),
            get_addr: None,
            key: Some(API_KEY.to_string()),
            http_client: None,
        };

        let rt = Runtime::new().unwrap();
        let client = Client::new(config);

        let pipe = client.pipe();
        let channel = Rc::new("chan3".to_string());

        let count = 10;

        for _ in 0..count {
            let _ = pipe.add_publish(channel.to_string(), r#"{"input": "test1"}"#, &[]);
        }

        let replies = match rt.block_on(client.send_pipe(&pipe)) {
            Ok(reply) => reply,
            Err(err) => {
                println!("an error occurred while sending pipe {err}");
                Vec::new()
            }
        };

        let reply_len = replies.len();

        for reply in replies {
            if let Some(err) = reply.error {
                println!("An error occured with {err}");
            }
        }

        assert_eq!(reply_len, count);
    }
}
