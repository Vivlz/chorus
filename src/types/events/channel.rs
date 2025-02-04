use std::sync::{Arc, RwLock};

use crate::types::events::WebSocketEvent;
use crate::types::Guild;
use crate::types::{entities::Channel, JsonField, Snowflake};
use chorus_macros::JsonField;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::UpdateMessage;

#[derive(Debug, Default, Deserialize, Serialize)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-pins-update>
pub struct ChannelPinsUpdate {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}

impl WebSocketEvent for ChannelPinsUpdate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonField)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-create>
pub struct ChannelCreate {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(skip)]
    pub json: String,
}

impl WebSocketEvent for ChannelCreate {}

impl UpdateMessage<Guild> for ChannelCreate {
    fn id(&self) -> Option<Snowflake> {
        self.channel.guild_id
    }

    fn update(&mut self, object_to_update: Arc<RwLock<Guild>>) {
        let mut write = object_to_update.write().unwrap();
        let update = Arc::new(RwLock::new(self.channel.clone()));
        if write.channels.is_some() {
            write.channels.as_mut().unwrap().push(update);
        } else {
            write.channels = Some(Vec::from([update]));
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonField)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-update>
pub struct ChannelUpdate {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(skip)]
    pub json: String,
}

impl WebSocketEvent for ChannelUpdate {}

impl UpdateMessage<Channel> for ChannelUpdate {
    fn update(&mut self, object_to_update: Arc<RwLock<Channel>>) {
        let mut write = object_to_update.write().unwrap();
        *write = self.channel.clone();
    }
    fn id(&self) -> Option<Snowflake> {
        Some(self.channel.id)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// Officially undocumented.
/// Sends updates to client about a new message with its id
/// {"channel_unread_updates": [{"id": "816412869766938648", "last_message_id": "1085892012085104680"}}
pub struct ChannelUnreadUpdate {
    pub channel_unread_updates: Vec<ChannelUnreadUpdateObject>,
    pub guild_id: Snowflake,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// Contains very few fields from [Channel]
/// See also [ChannelUnreadUpdate]
pub struct ChannelUnreadUpdateObject {
    pub id: Snowflake,
    pub last_message_id: Snowflake,
    pub last_pin_timestamp: Option<String>,
}

impl WebSocketEvent for ChannelUnreadUpdate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonField)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-delete>
pub struct ChannelDelete {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(skip)]
    pub json: String,
}

impl UpdateMessage<Guild> for ChannelDelete {
    fn id(&self) -> Option<Snowflake> {
        self.channel.guild_id
    }

    fn update(&mut self, object_to_update: Arc<RwLock<Guild>>) {
        if self.id().is_none() {
            return;
        }
        let mut write = object_to_update.write().unwrap();
        if write.channels.is_none() {
            return;
        }
        for (iteration, item) in (0_u32..).zip(write.channels.as_mut().unwrap().iter()) {
            if item.read().unwrap().id == self.id().unwrap() {
                write
                    .channels
                    .as_mut()
                    .unwrap()
                    .swap_remove(iteration as usize);
                return;
            }
        }
    }
}

impl WebSocketEvent for ChannelDelete {}
