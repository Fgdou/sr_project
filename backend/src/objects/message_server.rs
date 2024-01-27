use serde::{Deserialize, Serialize};
use ts_rs::TS;
use websocket::OwnedMessage;

use super::Infos;
#[derive(TS)]
#[ts(export)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageServer {
    Error(String),
    Infos(Infos),   
}

impl Into<OwnedMessage> for &MessageServer {
    fn into(self) -> OwnedMessage {
        OwnedMessage::Text(serde_json::to_string(self).unwrap())
    }
}