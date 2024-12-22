use crate::options::*;
pub mod options;
use crate::protocol::Command;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
mod protocol;

/// Pipe allows to send several commands in one HTTP request.
#[derive(Debug)]
pub struct Pipe {
    pub commands: Arc<RwLock<Vec<Command>>>,
}

#[serde(Serialize, Deserialize)]
pub struct PublishRequest {
    pub channel: String,
    pub data: serde_json::Value,
    pub options: PublishOptions,
}

#[serde(Serialize, Deserialize)]
pub struct BroadcastRequest {
    pub channel: Vec<String>,
    pub data: serde_json::Value,
    pub options: PublishOptions,
}

#[serde(Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub channel: String,
    pub user: String,
    pub options: SubscribeOptions,
}

#[serde(Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    pub channel: String,
    pub user: String,
    pub options: UnsubscribeOptions,
}

#[serde(Serialize, Deserialize)]
pub struct DisconnectRequest {
    pub user: String,
    pub options: DisconnectOptions,
}

#[serde(Serialize, Deserialize)]
pub struct HistoryRequest {
    pub channel: String,
    pub options: HistoryOptions,
}

#[serde(Serialize, Deserialize)]
pub struct ChannelsRequest {
    #[serde(skip_serializing_if = "option::is_none")]
    pub pattern: String,
}

impl Pipe {
    /// Reset allows to clear client command buffer
    pub fn reset(&self) {
        let mut commands = self.commands.write().unwrap();
        *commands = Vec::new();
    }

    pub fn add(&self, cmd: Command) -> Result<(), String> {
        let mut commands = self.commands.write().map_err(|_| "Lock poisoned")?;
        if let Some(ref mut cmd_vec) = *commands {
            cmd_vec.push(cmd);
            Ok(());
        } else {
            Err("Commands vector is None".to_string())
        }
    }

    /// AddPublish adds publish command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_publish(
        &self,
        channel: String,
        data: Vec<u8>,
        opts: &[impl PublishOption],
    ) -> Result<(), String> {
        let mut options = PublishOptions::default();
        for opt in opts {
            opt(&mut options)
        }

        let cmd = Command {
            method: "publish".to_string(),
            params: PublishRequest {
                channel,
                data,
                options,
            },
        };

        self.add(cmd)
    }

    /// AddBroadcast adds broadcast command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_broadcast(
        &self,
        channels: Vec<String>,
        data: Vec<u8>,
        opts: &[impl PublishOption],
    ) -> Result<(), String> {
        let mut options = PublishOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "broadcast".to_string(),
            params: BroadcastRequest {
                channels,
                data,
                options,
            },
        };

        self.add(cmd)
    }

    /// AddSubscribe adds subscribe command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_subscribe(
        &self,
        channel: String,
        user: String,
        opts: &[impl SubscribeOption],
    ) -> Result<(), String> {
        let mut options = SubscribeOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "subscribe".to_string(),
            params: SubscribeRequest {
                channel,
                data,
                options,
            },
        };

        self.add(cmd)
    }

    /// AddUnsubscribe adds unsubscribe command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_unsubscribe(
        &self,
        channel: String,
        user: String,
        opts: &[impl UnsubscribeOption],
    ) -> Result<(), String> {
        let mut options = UnsubscribeOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "unsubscribe".to_string(),
            params: UnsubscribeRequest {
                channel,
                user,
                options,
            },
        };

        self.add(cmd)
    }

    /// AddDisconnect adds disconnect command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_disconnect(
        &self,
        user: String,
        opts: &[impl DisconnectOption],
    ) -> Result<(), String> {
        let mut options = DisconnectOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "disconnect".to_string(),
            params: DisconnectRequest { user, options },
        };

        self.add(cmd)
    }

    /// AddPresence adds presence command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_presence(&self, channel: string) -> Result<(), String> {
        let cmd = Command {
            method: "presence".to_string(),
            params: serde_json::json!({
                "channel": channel,
            }),
        };

        self.add(cmd)
    }

    /// AddPresenceStats adds presence stats command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_presence_stats(&self, channel: string) -> Result<(), String> {
        let cmd = Command {
            method: "presence_stats".to_string(),
            params: serde_json::json!({
                "channel": channel,
            }),
        };

        self.add(cmd)
    }

    /// AddHistory adds history command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_history(&self, channel: String, opts: &[impl HistoryOption]) -> Result<(), String> {
        let mut options = HistoryOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "history".to_string(),
            params: HistoryRequest { channel, options },
        };

        self.add(cmd)
    }

    /// AddHistoryRemove adds history remove command to client command buffer but not
    /// actually sends request to server until Pipe will be explicitly sent.
    pub fn add_history_remove(&self, channel: string) -> Result<(), String> {
        let cmd = Command {
            method: "history_remove".to_string(),
            params: serde_json::json!({
                "channel": channel,
            }),
        };

        self.add(cmd)
    }

    /// AddChannels adds channels command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_channels(&self, opts: &[impl ChannelsOption]) -> Result<(), String> {
        let mut options = ChannelsOptions::default();
        for opt in opts {
            opt(&mut options);
        }

        let cmd = Command {
            method: "channels".to_string(),
            params: ChannelsRequest {
                pattern: options.Pattern,
            },
        };

        self.add(cmd)
    }

    /// AddInfo adds info command to client command buffer but not actually
    /// sends request to server until Pipe will be explicitly sent.
    pub fn add_info(&self) -> Result<(), String> {
        let cmd = Command {
            method: "info".to_string(),
            params: serde_json::json!({}),
        };
        self.add(cmd)
    }
}
