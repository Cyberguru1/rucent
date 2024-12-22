use crate::pipe::*;
pub mod pipe;
use crate::protocol::*;
pub mod protocol;
use reqwest::blocking::client;
use serde::serde_json;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

const ErrMalformedResponseString: String = "malformed response returned from server".to_string();
const ErrPipeEmptyString: String = "no commands in pipe".to_string();

#[derive(Debug)]
pub struct ErrMaiformedResponse {}

#[derive(Debug)]
pub struct ErrPipeEmpty {}

impl fmt::Display for ErrPipeEmpty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ErrPipeEmptyString)
    }
}

impl fmt::Display for ErrMalformedResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ErrMalformedResponseString)
    }
}

// Implement the `Error` trait for
impl Error for ErrPipeEmpty {}
// Implement the `Error` trait for `ErrMalformedResponse`
impl Error for ErrMalformedResponse {}

// ErrStatusCode can be returned in case request to server resulted in wrong status code.
pub struct ErrStatusCode {
    pub code: u16,
    pub body: Vec<u8>,
}

// Implement the `std::fmt::Display` trait for `ErrStatusCode`
impl fmt::Display for ErrStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "wrong status code: {}, body {}",
            self.code,
            String::from_utf8_lossy(&self.body)
        )
    }
}

// Implement the `Error` trait for `ErrStatusCode`
impl Error for ErrStatusCode {}

pub struct Config {
    pub addr: Option<String>,
    pub get_addr: Option<Arc<dyn Fn() -> Result<String, Box<dyn Error + Send + Sync>>>>,
    pub key: Option<String>,
    pub http_client: Option<Client>,
}
pub struct Client {
    pub endpoint: Option<String>,
    pub get_enpoint: Option<Arc<dyn Fn() -> Result<String, Box<dyn Error + Send + Sync>>>>,
    pub api_key: Option<String>,
    pub http_client: Option<Client>,
}

pub fn default_http_client() -> Client {
    Client::builder()
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap()
}

impl client {
    pub fn new(config: Config) -> self {
        let http_client = config.http_client.unwrap_or_else(default_http_client);
        Client {
            endpoint: Some(config.addr),
            get_endpoint: Some(config.get_addr),
            api_key: Some(config.key),
            http_client,
        }
    }

    pub fn set_http_client(&mut self, http_client: Client) {
        self.http_client = http_client;
    }

    pub fn pipe() -> Pipe {
        Pipe {
            commands: Arc::new(RwLock::new(None)),
        }
    }

    pub fn publish(
        &self,
        channel: String,
        data: Vec<u8>,
        opts: &[PublishOption],
    ) -> Result<PublishResult, Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_publish(channel, data, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_publish(&resp.result)
    }

    pub fn broadcast(
        &self,
        channels: Vec<String>,
        data: Vec<u8>,
        opts: &[PublishOption],
    ) -> Result<BroadcastResult, Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_broadcast(channels, data, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_broadcast(&resp.result)
    }

    pub fn subscribe(
        &self,
        channel: String,
        user: String,
        opts: &[SubscribeOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_subscribe(channel, user, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    pub fn unsubscribe(
        &self,
        channel: String,
        user: String,
        opts: &[UnsubscribeOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_unsubscribe(channel, user, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    pub fn disconnect(
        &self,
        user: String,
        opts: &[DisconnectOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_disconnect(channel, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    pub fn presence(&self, channel: String) -> Result<PresenceResult, Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_presence(channel, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_presence(&resp.result)
    }

    pub fn presence_stats(&self, channel: String) -> Result<PresenseStatsResult, Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_presense_stats(channel);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_presense_stats(&resp.result)
    }

    pub fn history(
        &self,
        channel: String,
        opts: &[HistoryOption],
    ) -> Result<Historyesult, Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_history(channel, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_history(&resp.result)
    }

    pub fn history_remove(&self, channel: String) -> Result<(), Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_history_remove(channel, data, &opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        Ok(())
    }

    pub fn channels(&self, opts: &[PublishOption]) -> Result<ChannelsResult, Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_channels(&opts);

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_channels(&resp.result)
    }

    pub fn info(&self) -> Result<InfoResult, Box<dyn Error>> {
        let mut pipe = self.pipe();
        pipe.add_info();

        let result = self.send_pipe(&pipe);

        if result.is_empty() {
            return Err("No reply from server".into());
        }

        let resp = &result[0];
        if let Some(err) = &resp.error {
            return Err(Box::new(err.clone()));
        }

        decode_info(&resp.result)
    }

    pub async fn send_pipe(
        &mut self,
        pipe: &mut Pipe,
    ) -> Result<Vec<Reply>, Box<dyn Error + Send + Sync>> {
        if pipe.commands.len() == 0 {
            return Err(Box::new(ErrPipeEmpty));
        }

        let result = self.send(&pipe.commands).await?;
        if result.len() != pipe.commands.len() {
            return Err(Box::new(ErrMalformedResponse));
        }

        Ok(result)
    }

    pub async fn send(
        &self,
        commands: Vec<Command>,
    ) -> Result<Vec<Reply>, Box<dyn Error + Send + Sync>> {
        // Serialze commands to json
        let payload = serde_json::to_vec(&commands)?;

        let endpoint = if let Some(get_endpoint) = &self.get_endpoint {
            get_endpoint()?
        } else {
            self.endpoint.clone()
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

        let response = request_builder.body(payload).send().await?;

        // Handle non-200 status code
        if !response.status().is_success() {
            let status = response.status();
            let resp_body = response.text().await.unwrap_or_default();
            return Err(Box::new(StatusCodeError { status, resp_body }));
        }

        // Deserialize replies.
        let replies: Vec<Reply> = response.json().await?;
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
