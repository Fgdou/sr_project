use serde::{Deserialize, Serialize};

use crate::Infos;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageServer {
    Error(String),
    Infos(Infos),   
}