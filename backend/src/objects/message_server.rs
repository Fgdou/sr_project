use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::Infos;
#[derive(TS)]
#[ts(export)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageServer {
    Error(String),
    Infos(Infos),   
}