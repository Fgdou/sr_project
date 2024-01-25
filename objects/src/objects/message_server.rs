use crate::Infos;

pub enum MessageServer {
    Error(String),
    Infos(Infos),   
}