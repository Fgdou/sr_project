use serde::{Deserialize, Serialize};
use ts_rs::TS;
use websocket::OwnedMessage;

use super::{Event, Infos};
#[derive(TS)]
#[ts(export)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageServer {
    Error(String),
    Infos(Infos),
    ChangeInfos{events: Vec<Event>, count: i32},
    SetId(i32)
}

impl Into<OwnedMessage> for &MessageServer {
    fn into(self) -> OwnedMessage {
        OwnedMessage::Text(serde_json::to_string(self).unwrap())
    }
}