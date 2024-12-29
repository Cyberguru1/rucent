# Rucent

Rucent is a Rust library designed to facilitate communication with Centrifugo's HTTP API. It provides seamless interaction with Centrifugo's server for managing real-time messaging and client interactions. With Rucent, you can issue API commands such as publishing messages, managing subscriptions, and retrieving history data.

## Features

- Publish messages to specific channels.
- Manage subscriptions and disconnections.
- Retrieve historical data from Centrifugo.
- Batch multiple commands in a single request for efficiency.

## Requirements

Before using Rucent, ensure you have:

- Rust installed on your system (version 1.65 or later).
- A running instance of Centrifugo.

## Installation

Add the Rucent library to your `Cargo.toml` dependencies:

```toml
[dependencies]
rucent = "0.1.1"
```

Enable optional features as needed:

```toml
[features]
with_local_server = []
```

## Usage

### Example: Sending a Publish Command

```rust
use rucent::client::{Client, Config};

#[tokio::main]
async fn main() {
    let config = Config {
        addr: Some("http://127.0.0.1:8000/api".to_string()),
        key: Some("your_api_key".to_string()),
        http_client: None,
    };

    let client = Client::new(config);

    let channel = "test_channel";
    let payload = serde_json::json!({ "input": "Hello, Rucent!" });

    match client.publish(channel.to_string(), payload).await {
        Ok(response) => println!("Publish successful: {:?}", response),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```

### Example: Broadcast to Multiple Channels
```rust
--snip--
    let mut channels = Vec::new();

    for i in 0..10 {
        channels.push(format!("test_channel_{}", i));
    }


    let broadcast_result = client
        .broadcast(channels, r#"{"date": "2024-12-28"}"#, &[])
        .await;

    log::info!("Broadcasted to {} channels successfully", broadcast_result.unwrap().responses.len());
```

### Example: Batching Multiple Commands

```rust
#[tokio::main]
async fn main() {
    let config = Config {
        addr: Some("http://127.0.0.1:8000/api".to_string()),
        key: Some("your_api_key".to_string()),
        http_client: None,
    };

    let client = Client::new(config);
    let pipe = client.pipe();

    let channel = Rc::new("chan3".to_string());

    let count = 10;

    for _ in 0..count {
        let _ = pipe.add_publish(channel.to_string(), r#"{"input": "test1"}"#, &[]);
    }

    let replies = match client.send_pipe(&pipe).await {
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

    println!("Sent {reply_len} commands in one http request");
}
```

## Running Tests

### With Local Environment

To run tests using a local Centrifugo instance, provide the required environment variables:

```bash
API_URL=http://127.0.0.1:8000/api API_KEY=fa7ce149-b279-4870-af59-ad7ce78ef11a cargo test --test client --features with_local_server -- --nocapture
```

### Without Local Environment

To run tests without a local Centrifugo instance:

```bash
cargo test --test client -- --nocapture
```

## Contributing

We welcome contributions! Feel free to open issues, submit pull requests, or suggest new features.

### Steps to Contribute

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-branch-name`.
3. Make your changes and commit them: `git commit -m "Description of changes"`.
4. Push to your fork: `git push origin feature-branch-name`.
5. Open a pull request against the main repository.

## License

Rucent is licensed under either of the following licenses, at your option:

- MIT License
- Apache License 2.0

See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) for details.

