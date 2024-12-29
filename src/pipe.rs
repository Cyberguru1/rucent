use crate::options::{
    ChannelsOption, ChannelsOptions, DisconnectOption, DisconnectOptions, HistoryOption,
    HistoryOptions, PublishOption, PublishOptions, SubscribeOption, SubscribeOptions,
    UnsubscribeOption, UnsubscribeOptions,
};
use serde::{Deserialize, Serialize};
pub use std::error::Error;
use std::sync::{Arc, Mutex};

/// Pipe allows to send several commands in one HTTP request.
#[derive(Debug)]
pub struct Pipe {
    pub commands: Arc<Mutex<Vec<Command>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublishRequest {
    pub channel: String,
    pub data: serde_json::Value,
    pub options: PublishOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BroadcastRequest {
    pub channels: Vec<String>,
    pub data: serde_json::Value,
    pub options: PublishOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscribeRequest {
    pub channel: String,
    pub user: String,
    pub options: SubscribeOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnsubscribeRequest {
    pub channel: String,
    pub user: String,
    pub options: UnsubscribeOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisconnectRequest {
    pub user: String,
    pub options: DisconnectOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryRequest {
    pub channel: String,
    pub options: HistoryOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// # Request Kinds
/// This are types for params in Command struct
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RequestKind {
    ChannelsRequest(ChannelsRequest),
    PublishRequest(PublishRequest),
    BroadcastRequest(BroadcastRequest),
    SubscribeRequest(SubscribeRequest),
    UnsubscribeRequest(UnsubscribeRequest),
    DisconnectRequest(DisconnectRequest),
    HistoryRequest(HistoryRequest),
    Value(serde_json::Value),
}
/// # Command
/// Command represents API command to send
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub method: String,
    pub params: RequestKind,
}

/// # Pipe
impl Pipe {
    /// Reset allows to clear client command buffer
    pub fn reset(&self) {
        let mut commands = self.commands.lock().unwrap();
        *commands = Vec::new();
    }

    pub fn add(&self, cmd: Command) -> Result<(), Box<dyn Error>> {
        let mut commands = self.commands.lock().map_err(|_| "Lock poisoned")?;
        commands.push(cmd);
        Ok(())
    }

    /// AddPublish adds publish command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_publish(
        &self,
        channel: String,
        data: &str,
        opts: &[PublishOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut options = PublishOptions::default();
        for opt in opts {
            opt(&mut options)
        }

        let cmd = Command {
            method: "publish".to_string(),
            params: RequestKind::PublishRequest(PublishRequest {
                channel,
                data: serde_json::from_str(data)?,
                options,
            }),
        };

        self.add(cmd.clone())?;
        Ok(())
    }

    /// AddBroadcast adds broadcast command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_broadcast(
        &self,
        channels: Vec<String>,
        data: &str,
        opts: &[PublishOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut options = PublishOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "broadcast".to_string(),
            params: RequestKind::BroadcastRequest(BroadcastRequest {
                channels: channels,
                data: serde_json::from_str(data)?,
                options,
            }),
        };
        self.add(cmd)?;
        Ok(())
    }

    /// AddSubscribe adds subscribe command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_subscribe(
        &self,
        channel: String,
        user: String,
        opts: &[SubscribeOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut options = SubscribeOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "subscribe".to_string(),
            params: RequestKind::SubscribeRequest(SubscribeRequest {
                channel,
                user,
                options,
            }),
        };
        self.add(cmd)?;
        Ok(())
    }

    /// AddUnsubscribe adds unsubscribe command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_unsubscribe(
        &self,
        channel: String,
        user: String,
        opts: &[UnsubscribeOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut options = UnsubscribeOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "unsubscribe".to_string(),
            params: RequestKind::UnsubscribeRequest(UnsubscribeRequest {
                channel,
                user,
                options,
            }),
        };

        self.add(cmd)?;
        Ok(())
    }

    /// AddDisconnect adds disconnect command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_disconnect(
        &self,
        user: String,
        opts: &[DisconnectOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut options = DisconnectOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "disconnect".to_string(),
            params: RequestKind::DisconnectRequest(DisconnectRequest { user, options }),
        };

        self.add(cmd)?;
        Ok(())
    }

    /// AddPresence adds presence command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_presence(&self, channel: String) -> Result<(), Box<dyn Error>> {
        let cmd = Command {
            method: "presence".to_string(),
            params: RequestKind::Value(serde_json::json!({
                "channel": channel,
            })),
        };

        self.add(cmd)?;
        Ok(())
    }

    /// AddPresenceStats adds presence stats command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_presence_stats(&self, channel: String) -> Result<(), Box<dyn Error>> {
        let cmd = Command {
            method: "presence_stats".to_string(),
            params: RequestKind::Value(serde_json::json!({
                "channel": channel,
            })),
        };
        self.add(cmd)?;
        Ok(())
    }

    /// AddHistory adds history command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_history(
        &self,
        channel: String,
        opts: &[HistoryOption],
    ) -> Result<(), Box<dyn Error>> {
        let mut options = HistoryOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "history".to_string(),
            params: RequestKind::HistoryRequest(HistoryRequest { channel, options }),
        };

        self.add(cmd)?;
        Ok(())
    }

    /// AddHistoryRemove adds history remove command to client command buffer but not
    /// actually sends request to server until Pipe will be explicitly sent.
    pub fn add_history_remove(&self, channel: String) -> Result<(), Box<dyn Error>> {
        let cmd = Command {
            method: "history_remove".to_string(),
            params: RequestKind::Value(serde_json::json!({
                "channel": channel,
            })),
        };

        self.add(cmd)?;
        Ok(())
    }

    /// AddChannels adds channels command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_channels(&self, opts: &[ChannelsOption]) -> Result<(), Box<dyn Error>> {
        let mut options = ChannelsOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "channels".to_string(),
            params: RequestKind::ChannelsRequest(ChannelsRequest {
                pattern: options.pattern,
            }),
        };

        self.add(cmd)?;
        Ok(())
    }

    /// AddInfo adds info command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_info(&self) -> Result<(), Box<dyn Error>> {
        let cmd = Command {
            method: "info".to_string(),
            params: RequestKind::Value(serde_json::json!({})),
        };
        self.add(cmd)?;
        Ok(())
    }
}
