use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Command represents API command to send
#[derive(Serialize, Deserialize)]
pub struct Command {
    pub method: string,
    pub params: serde_json::value,
}

/// Error reptrests API request error.
#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: string,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.code)
    }
}

impl std::error::Error for Error {}

/// Reply is for server response to command
#[derive(Serialize, Deserialize)]
pub struct Reply {
    pub error: option<Error>,
    pub result: serde_json::value,
}

/// ClientInfo represents information about one client connection to centrifugo.
/// This struct used in messages published by clients, join/leave events, presence data
#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    pub user: string,
    pub client: string,
    #[serde(skip_serializing_if = "option::is_none")]
    pub conn_info: serde_json::value,
    #[serde(skip_serializing_if = "option::is_none")]
    pub chan_info: serde_json::value,
}

/// Publication represents message published into channel.
#[derive(Serialize, Deserialize)]
pub struct Publication {
    pub offset: uint64,
    pub data: serde_json::value,
    pub info: option<ClientInfo>,
}

/// NodeInfo contains information and statistics about Centrifugo node.
#[derive(Serialize, Deserialize)]
pub struct NodeInfo {
    /// uid is a unique id of running node.
    pub uid: string,
    /// name is a name of node (config defined or generated automatically).
    pub name: string,
    /// version of Centrifugo node.
    pub version: string,
    /// num_clients is a number of clients connected to node.
    pub num_clients: int64,
    /// num_users is a number of unique users connected to node
    pub num_users: int64,
    /// num_channels is a number of channels on node
    pub num_channels: int64,
    /// uptime of node in seconds.
    pub uptime: int64,
}

/// Info Result is a reulst of info command
#[derive(Serialize, Deserialize)]
pub struct InfoResult {
    pub nodes: Vec<NodeInfo>,
}

/// PublishResult is a result of publish command
#[derive(Serialize, Deserialize)]
pub struct PublishResult {
    offset: uint64,
    epoch: string,
}

#[derive(Serialize, Deserialize)]
pub struct PublishResponse {
    pub error: option<Error>,
    pub result: PublishResult,
}

/// BroadcastResult is a result of broadcast command
#[derive(Serialize, Deserialize)]
pub struct BroadcastResult {
    pub responses: Vec<PublishResponse>,
}

/// PresenceStatsResult is a result of presence command
#[derive(Serialize, Deserialize)]
pub struct PresenceResult {
    pub presence: HashMap<string, ClientInfo>,
}

/// PresenceStatsResult is a reuslt of info command
#[derive(Serialize, Deserialize)]
pub struct PresenceStatsResult {
    pub num_users: int32,
    pub num_clients: int32,
}

/// HistoryResult is a result of history command
#[derive(Serialize, Deserialize)]
pub struct HistoryResult {
    pub publication: Vec<Publication>,
    pub offset: uint64,
    pub epoch: string,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelInfo {
    pub num_users: int32,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelsResult {
    pub channels: HashMap<string, ChannelInfo>,
}
