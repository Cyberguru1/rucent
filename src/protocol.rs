use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Error reptrests API request error.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub code: u16,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.code)
    }
}

impl std::error::Error for Error {}

/// Reply is for server response to command
#[derive(Serialize, Deserialize, Debug)]
pub struct Reply {
    pub error: Option<Error>,
    pub result: Option<serde_json::Value>,
}

/// ClientInfo represents information about one client connection to centrifugo.
/// This struct used in messages published by clients, join/leave events, presence data
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub user: String,
    pub client: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chan_info: Option<serde_json::Value>,
}

/// Publication represents message published into channel.
#[derive(Serialize, Deserialize, Debug)]
pub struct Publication {
    pub offset: u16,
    pub data: serde_json::Value,
    pub info: Option<ClientInfo>,
}

/// NodeInfo contains information and statistics about Centrifugo node.
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    /// uid is a unique id of running node.
    pub uid: String,
    /// name is a name of node (config defined or generated automatically).
    pub name: String,
    /// version of Centrifugo node.
    pub version: String,
    /// num_clients is a number of clients connected to node.
    pub num_clients: u16,
    /// num_users is a number of unique users connected to node
    pub num_users: u16,
    /// num_channels is a number of channels on node
    pub num_channels: u16,
    /// uptime of node in seconds.
    pub uptime: u16,
}

/// Info Result is a reulst of info command
#[derive(Serialize, Deserialize, Debug)]
pub struct InfoResult {
    pub nodes: Vec<NodeInfo>,
}

/// PublishResult is a result of publish command
#[derive(Serialize, Deserialize, Debug)]
pub struct PublishResult {
    pub offset: Option<u16>,
    pub epoch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishResponse {
    pub error: Option<Error>,
    pub result: PublishResult,
}

/// BroadcastResult is a result of broadcast command
#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastResult {
    pub responses: Vec<PublishResponse>,
}

/// PresenceStatsResult is a result of presence command
#[derive(Serialize, Deserialize, Debug)]
pub struct PresenceResult {
    pub presence: HashMap<String, ClientInfo>,
}

/// PresenceStatsResult is a reuslt of info command
#[derive(Serialize, Deserialize, Debug)]
pub struct PresenceStatsResult {
    pub num_users: u16,
    pub num_clients: u16,
}

/// HistoryResult is a result of history command
#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryResult {
    pub publication: Vec<Publication>,
    pub offset: u16,
    pub epoch: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelInfo {
    pub num_clients: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsResult {
    pub channels: HashMap<String, ChannelInfo>,
}
