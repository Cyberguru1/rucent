use crate::options::{
    ChannelsOption, DisconnectOption, HistoryOption, PublishOption, SubscribeOption,
    UnsubscribeOption,
};
use crate::protocol::{
    BroadcastResult, ChannelsResult, HistoryResult, InfoResult, PresenceResult,
    PresenceStatsResult, PublishResult, Reply,
};
use reqwest::Client as ReqClient;
use serde_json;
use std::error::Error;
use std::fmt;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::pipe::{Command, Pipe};

const ERR_MALFORMED_RESPONSE_STRING: &str = "malformed response returned from server";
const ERR_PIPE_EMPTY_STRING: &str = "no commands in pipe";

#[derive(Debug)]
struct ErrMalformedResponse {}

#[derive(Debug)]
struct ErrPipeEmpty {}

impl fmt::Display for ErrPipeEmpty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ERR_PIPE_EMPTY_STRING)
    }
}

impl fmt::Display for ErrMalformedResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ERR_MALFORMED_RESPONSE_STRING)
    }
}

// Implement the `Error` trait for
impl Error for ErrPipeEmpty {}
// Implement the `Error` trait for `ErrMalformedResponse`
impl Error for ErrMalformedResponse {}

// ErrStatusCode can be returned in case request to server resulted in wrong status code.
#[derive(Debug)]
pub struct ErrStatusCode {
    pub code: u16,
    pub body: String,
}

// Implement the `std::fmt::Display` trait for `ErrStatusCode`
impl fmt::Display for ErrStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wrong status code: {}, body {}", self.code, &self.body)
    }
}

// Implement the `Error` trait for `ErrStatusCode`
impl Error for ErrStatusCode {}

pub type ErrRes = Box<dyn Error + Send + Sync>;

/// # Config
#[derive(Default, Clone)]
pub struct Config {
    /// addr is centrifugo api endpoint
    pub addr: Option<String>,
    /// GetAddr when set will be used before every API call to extract
    /// Centrifugo API endpoint. In this case Addr field of Config will be
    /// ignored. Nil value means using static Config.addr field.
    pub get_addr: Option<Arc<dyn Fn() -> Result<String, ErrRes>>>,
    /// Centrifugo api key
    pub key: Option<String>,
    /// http_client is a custom http client to be used
    /// default is used if nil
    pub http_client: Option<ReqClient>,
}

/// # Client
/// Client is API client for project registered in server.
pub struct Client {
    pub endpoint: Option<String>,
    pub get_endpoint: Option<Arc<dyn Fn() -> Result<String, ErrRes>>>,
    pub api_key: Option<String>,
    pub http_client: ReqClient,
}

/// DefaultHTTPClent
pub fn default_http_client() -> ReqClient {
    ReqClient::builder()
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap()
}

impl Client {
    /// Create a new client instance.
    pub fn new(config: Config) -> Self {
        let http_client = config.http_client.unwrap_or_else(default_http_client);
        Client {
            endpoint: config.addr,
            get_endpoint: config.get_addr,
            api_key: config.key,
            http_client,
        }
    }

    /// set_http_client allows to set custom http client to use for requests.
    pub fn set_http_client(&mut self, http_client: ReqClient) {
        self.http_client = http_client;
    }

    /// pipe allows to create new pipe to send several commands in one HTTP request.
    pub fn pipe(&self) -> Pipe {
        Pipe {
            commands: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Publish allows to publish data to channel.
    pub async fn publish(
        &self,
        channel: String,
        data: &str,
        opts: &[PublishOption],
    ) -> Result<PublishResult, Box<dyn Error>> {
        let pipe = self.pipe();
        pipe.add_publish(channel, data, opts)?;

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_publish(&serde_json::to_vec(&resp.result).unwrap())
    }

    /// Broadcast allows to broadcast the same data into many channels..
    pub async fn broadcast(
        &self,
        channels: Vec<String>,
        data: &str,
        opts: &[PublishOption],
    ) -> Result<BroadcastResult, Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_broadcast(channels, data, opts);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_broadcast(&serde_json::to_vec(&resp.result).unwrap())
    }

    /// Subscribe allow subscribing user to a channel (using server-side subscriptions).
    pub async fn subscribe(
        &self,
        channel: String,
        user: String,
        opts: &[SubscribeOption],
    ) -> Result<(), Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_subscribe(channel, user, opts);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    /// Unsubscribe allows to unsubscribe user from channel.
    pub async fn unsubscribe(
        &self,
        channel: String,
        user: String,
        opts: &[UnsubscribeOption],
    ) -> Result<(), Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_unsubscribe(channel, user, opts);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    /// Disconnect allows to close all connections of user to server.
    pub async fn disconnect(
        &self,
        user: String,
        opts: &[DisconnectOption],
    ) -> Result<(), Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_disconnect(user, opts);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    /// Presence returns channel presence information.
    pub async fn presence(&self, channel: String) -> Result<PresenceResult, Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_presence(channel);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_presence(&serde_json::to_vec(&resp.result).unwrap())
    }

    /// PresenceStats returns short channel presence information (only counters).
    pub async fn presence_stats(
        &self,
        channel: String,
    ) -> Result<PresenceStatsResult, Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_presence_stats(channel);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_presence_stats(&serde_json::to_vec(&resp.result).unwrap())
    }

    /// History returns channel history.
    pub async fn history(
        &self,
        channel: String,
        opts: &[HistoryOption],
    ) -> Result<HistoryResult, Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_history(channel, opts);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_history(&serde_json::to_vec(&resp.result).unwrap())
    }

    /// HistoryRemove removes channel history.
    pub async fn history_remove(&self, channel: String) -> Result<(), Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_history_remove(channel);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    /// Channels returns information about active channels (with one or more subscribers) on server.
    pub async fn channels(
        &self,
        opts: &[ChannelsOption],
    ) -> Result<ChannelsResult, Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_channels(opts);

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_channels(&serde_json::to_vec(&resp.result).unwrap())
    }

    /// Info returns information about server nodes.
    pub async fn info(&self) -> Result<InfoResult, Box<dyn Error>> {
        let pipe = self.pipe();
        let _ = pipe.add_info();

        let response = self.send_pipe(&pipe).await;

        let result = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_info(&serde_json::to_vec(&resp.result).unwrap())
    }

    pub async fn send_pipe(&self, pipe: &Pipe) -> Result<Vec<Reply>, Box<dyn Error + Send + Sync>> {
        let mut commands = pipe.commands.lock().map_err(|_| "Lock poisoned")?;
        if commands.is_empty() {
            return Err(Box::new(ErrPipeEmpty {}));
        }

        let response = self.send(commands.deref_mut().to_vec()).await;

        let result: Vec<Reply> = match response {
            Ok(response) => response,
            Err(err) => return Err(err),
        };

        if result.len() != commands.len() {
            return Err(Box::new(ErrMalformedResponse {}));
        }

        Ok(result)
    }

    pub async fn send(
        &self,
        commands: Vec<Command>,
    ) -> Result<Vec<Reply>, Box<dyn Error + Sync + Send>> {
        // Serialize commands to json string

        let mut lines = Vec::with_capacity(commands.len());

        for cmd in &commands {
            lines.push(serde_json::to_string(cmd)?);
        }

        let lines = lines.join("\n");

        let endpoint = if let Some(get_endpoint) = &self.get_endpoint {
            get_endpoint()?
        } else {
            self.endpoint.clone().unwrap()
        };

        // Create the HTTP request
        let request_builder = self
            .http_client
            .post(&endpoint)
            .header("Content-Type", "application/json");

        let request_builder = if let Some(api_key) = &self.api_key {
            request_builder.header("Authorization", format!("apikey {}", api_key))
        } else {
            request_builder
        };

        // Send request

        let response = request_builder.body(lines).send().await?;
        // Handle non-200 status code
        if !response.status().is_success() {
            let status = response.status();
            let resp_body = response.text().await?;
            return Err(Box::new(ErrStatusCode {
                code: status.as_u16(),
                body: resp_body,
            }));
        }

        // Deserialize replies from the response body.
        let bytes = response.bytes().await?;

        // Split the JSON by newline and deserialize to Reply structs
        let replies = String::from_utf8(bytes.to_vec())?
            .lines()
            .map(|line| serde_json::from_str::<Reply>(line))
            .collect::<Result<Vec<Reply>, _>>()?;

        Ok(replies)
    }
}

pub fn decode_publish(result: &[u8]) -> Result<PublishResult, Box<dyn Error>> {
    let r: PublishResult = serde_json::from_slice(result)?;
    Ok(r)
}

pub fn decode_broadcast(result: &[u8]) -> Result<BroadcastResult, Box<dyn Error>> {
    let r: BroadcastResult = serde_json::from_slice(result)?;
    Ok(r)
}

pub fn decode_history(result: &[u8]) -> Result<HistoryResult, Box<dyn Error>> {
    let r: HistoryResult = serde_json::from_slice(result)?;
    Ok(r)
}

pub fn decode_channels(result: &[u8]) -> Result<ChannelsResult, Box<dyn Error>> {
    let r: ChannelsResult = serde_json::from_slice(result)?;
    Ok(r)
}

pub fn decode_info(result: &[u8]) -> Result<InfoResult, Box<dyn Error>> {
    let r: InfoResult = serde_json::from_slice(result)?;
    Ok(r)
}

pub fn decode_presence(result: &[u8]) -> Result<PresenceResult, Box<dyn Error>> {
    let r: PresenceResult = serde_json::from_slice(result)?;
    Ok(r)
}

pub fn decode_presence_stats(result: &[u8]) -> Result<PresenceStatsResult, Box<dyn Error>> {
    let r: PresenceStatsResult = serde_json::from_slice(result)?;
    Ok(r)
}
