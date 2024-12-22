use serde::serde_json;
use serde::{Deserialize, Serialize};
use serde_json::RawValue;

#[derive(Clone, Debug, Copy)]
#[serde(Serialize, Deserialize)]
pub struct PublishOptions {
    pub skip_history: Option<bool>,
}

/// PublishOption is a type to represent vairous publish options
pub type PublishOption = Box<dyn FnOnce(&mut PublishOptions)>;

/// with_skip_history allows to set skip_history field.
pub fn with_skip_history(skip: bool) -> PublishOption {
    Box::new(move |opts: &mut PublishOptions| {
        opts.skip_history = Some(skip);
    })
}

/// SubscribeOption define the per-subscription options
#[derive(Clone, Debug, Copy)]
#[serde(Serialize, Deserialize)]
pub struct SubscribeOptions {
    /// ChannelInfo defines custom channel information, zero value means no channel information.
    #[serde(skip_serializing_if = "option::is_none")]
    pub info: Option<RawValue>,
    /// Presence turns on participating in channel presence.
    #[serde(skip_serializing_if = "option::is_none")]
    pub presence: Option<bool>,
    /// JoinLeave enables sending Join and Leave messages for this client in channel.
    #[serde(skip_serializing_if = "option::is_none")]
    pub join_leave: Option<bool>,
    /// When position is on client will additionally sync its position inside
    /// a stream to prevent message loss. Make sure you are enabling Position in channels
    /// that maintain Publication history stream. When Position is on  Centrifuge will
    /// include StreamPosition information to subscribe response - for a client to be able
    /// to manually track its position inside a stream.
    #[serde(skip_serializing_if = "option::is_none")]
    pub position: Option<bool>,
    /// Recover turns on recovery option for a channel. In this case client will try to
    /// recover missed messages automatically upon resubscribe to a channel after reconnect
    /// to a server. This option also enables client position tracking inside a stream
    /// (like Position option) to prevent occasional message loss. Make sure you are using
    /// Recover in channels that maintain Publication history stream.
    #[serde(skip_serializing_if = "option::is_none")]
    pub recover: Option<bool>,
    /// Data to send to a client with Subscribe Push.
    #[serde(skip_serializing_if = "option::is_none")]
    pub data: Option<RawValue>,
    /// RecoverSince will try to subscribe a client and recover from a certain StreamPosition.
    #[serde(skip_serializing_if = "option::is_none")]
    pub recover_since: Option<StreamPosition>,
    /// ClientID to subscribe.
    #[serde(skip_serializing_if = "option::is_none")]
    pub client_id: Option<String>,
}

pub type SubscribeOption = Box<dyn FnOnce(&mut SubscribeOptions)>;

pub fn with_subscribe_info(chan_info: RawValue) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.info = Some(chan_info))
}

pub fn with_presence(enabled: bool) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.presence = Some(enabled))
}

pub fn with_join_leave(enabled: bool) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.join_leave = Some(enabled))
}

pub fn with_position(enabled: bool) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.position = Some(enabled))
}

pub fn with_recover(enabled: bool) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.recover = Some(enabled))
}

pub fn with_subscribe_client(client_id: string) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.client_id = Some(client_id))
}

pub fn with_subscribe_data(data: RawValue) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.data = Some(data))
}

pub fn with_recover_since(since: StreamPosition) -> SubscribeOption {
    Box::new(move |opts: &mut SubscribeOptions| opts.recover_since = Some(since))
}

pub struct UnsubscribeOptions {
    /// client_id is unsubscribe.
    #[serde(skip_serializing_if = "option::is_none")]
    pub client_id: Option<String>,
}

pub type UnsubscribeOption = Box<dyn FnOnce(&mut UnsubscribeOptions)>;

pub fn with_unsubscribe_client(client_id: string) -> UnsubscribeOption {
    Box::new(move |opts: &mut UnsubscribeOptions| opts.client_id = Some(client_id))
}

#[serde(Serialize, Deserialize)]
pub struct Disconnect {
    #[serde(skip_serializing_if = "option::is_none")]
    pub code: Option<u32>,
    pub reason: Option<String>,
    pub reconnect: Option<bool>,
}

#[serde(Serialize, Deserialize)]
pub struct DisconnectOptions {
    pub disconnect: Option<Disconnect>,
    pub client_whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "option::is_none")]
    pub clinet_id: Option<String>,
}

pub type DisconnectOption = Box<Dyn, FnOnce(&mut DisconnectOptions)>;

pub fn with_disconnect(disconnect: Disconnect) -> DisconnectOption {
    Box::new(move |opts: &mut DisconnectOptions| opts.disconnect = Some(disconnect))
}

pub fn with_disconnect_client(client_id: string) -> DisconnectOption {
    Box::new(move |opts: &mut DisconnectOptions| opts.client_id = Some(client_id))
}

pub fn with_disconnect_client_whitelist(whitelist: Vec<String>) -> DisconnectOption {
    Box::new(move |opts: &mut DisconnectOptions| opts.client_whitelist = Some(whitelist))
}

pub struct HistoryOptions {
    #[serde(skip_serializing_if = "option::is_none")]
    pub since: Option<StreamPosition>,
    #[serde(skip_serializing_if = "option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "option::is_none")]
    pub reverse: Option<bool>,
}
pub const no_limit: i32 = -1;

pub type HistoryOption = Box<dyn FnOnce(&mut HistoryOptions)>;

pub fn with_limit(limit: i32) -> HistoryOption {
    Box::new(move |opts: &mut HistoryOptions| opts.limit = Some(limit))
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StreamPosition {
    #[serde(skip_serializing_if = "option::is_none")]
    pub offset: u64,
    #[serde(skip_serializing_if = "option::is_none")]
    pub epoch: String,
}

pub fn with_since(since: StreamPosition) -> HistoryOption {
    Box::new(move |opts: &mut HistoryOptions| opts.since = Some(since))
}

pub fn with_reverse(reverse: bool) -> HistoryOption {
    Box::new(move |opts: &mut HistoryOptions| opts.reverse = Some(reverse))
}

pub struct ChannelsOptions {
    #[serde(skip_serializing_if = "option::is_none")]
    pub pattern: Option<String>,
}

pub type ChannelsOption = Box<dyn FnOnce(&mut ChannelsOptions)>;

pub fn with_pattern(pattern: string) -> ChannelsOption {
    Box::new(move |opts: &mut ChannelsOptions| opts.pattern = Some(pattern))
}
